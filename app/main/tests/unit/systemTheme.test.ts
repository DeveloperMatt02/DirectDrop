import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import {
	syncSystemTheme,
	type PortalThemeSource,
	type SystemTheme,
	type ThemeEnv,
	type ThemeSource,
} from '../../src/composables/systemTheme';

function fakeRoot() {
	return { dataset: {} as Record<string, string> } as unknown as HTMLElement;
}

function fakeWindow(theme: () => Promise<SystemTheme | null>) {
	let handler: ((e: { payload: SystemTheme }) => void) | null = null;
	const unlisten = vi.fn();
	const win: ThemeSource = {
		theme,
		onThemeChanged: async (h) => {
			handler = h;
			return unlisten;
		},
	};
	return { win, unlisten, emit: (t: SystemTheme) => handler?.({ payload: t }) };
}

function fakeEnv(dark: boolean) {
	const media = Object.assign(new EventTarget(), { matches: dark });
	const focus = new EventTarget();
	const env: ThemeEnv = { media, focus };
	return {
		env,
		/** System switch as WebKitGTK sees it: the media query flips and fires change. */
		flip(nowDark: boolean) {
			media.matches = nowDark;
			media.dispatchEvent(new Event('change'));
		},
		focus: () => focus.dispatchEvent(new Event('focus')),
	};
}

/** Desktop setting via the portal: read() gives the startup value, change() a signal. */
function fakePortal(read: () => Promise<SystemTheme | null>) {
	let handler: ((t: SystemTheme | null) => void) | null = null;
	const unlisten = vi.fn();
	const portal: PortalThemeSource = {
		read,
		onChanged: async (h) => {
			handler = h;
			return unlisten;
		},
	};
	return { portal, unlisten, change: (t: SystemTheme | null) => handler?.(t) };
}

const settle = () => new Promise((r) => setTimeout(r, 0));

describe('syncSystemTheme', () => {
	beforeEach(() => {
		vi.spyOn(console, 'warn').mockImplementation(() => undefined);
		vi.spyOn(console, 'info').mockImplementation(() => undefined);
	});
	afterEach(() => vi.restoreAllMocks());

	test('Tauri theme sets data-theme and follows changes', async () => {
		const root = fakeRoot();
		const { win, unlisten, emit } = fakeWindow(async () => 'dark');
		const off = await syncSystemTheme(win, { root });
		expect(root.dataset.theme).toBe('dark');
		emit('light');
		expect(root.dataset.theme).toBe('light');
		off();
		expect(unlisten).toHaveBeenCalledTimes(1);
	});

	test('null, failure or timeout leave data-theme unset (media query applies)', async () => {
		for (const theme of [
			async () => null,
			async () => {
				throw new Error('no theme');
			},
			() => new Promise<SystemTheme | null>(() => undefined),
		]) {
			const root = fakeRoot();
			await syncSystemTheme(fakeWindow(theme).win, { root, timeoutMs: 10 });
			expect(root.dataset.theme).toBeUndefined();
		}
	});

	test('a system switch without ThemeChanged (Linux) still updates data-theme', async () => {
		let tauri: SystemTheme = 'light';
		const root = fakeRoot();
		const sys = fakeEnv(false);
		await syncSystemTheme(fakeWindow(async () => tauri).win, { root, timeoutMs: 50, env: sys.env });
		expect(root.dataset.theme).toBe('light');

		tauri = 'dark';
		sys.flip(true);
		await settle();
		expect(root.dataset.theme).toBe('dark');

		tauri = 'light';
		sys.focus();
		await settle();
		expect(root.dataset.theme).toBe('light');
	});

	test('theme() failing after a change removes data-theme (media query applies)', async () => {
		let fail = false;
		const root = fakeRoot();
		const sys = fakeEnv(false);
		const { win } = fakeWindow(async () => {
			if (fail) throw new Error('gone');
			return 'light';
		});
		await syncSystemTheme(win, { root, timeoutMs: 50, env: sys.env });
		expect(root.dataset.theme).toBe('light');
		fail = true;
		sys.flip(true);
		await settle();
		expect(root.dataset.theme).toBeUndefined();
	});

	test('cleanup removes the Tauri and portal listeners and the change/focus listeners', async () => {
		const theme = vi.fn(async (): Promise<SystemTheme> => 'dark');
		const { win, unlisten } = fakeWindow(theme);
		const p = fakePortal(async () => null);
		const sys = fakeEnv(true);
		const off = await syncSystemTheme(win, { root: fakeRoot(), timeoutMs: 50, env: sys.env, portal: p.portal });
		off();
		expect(unlisten).toHaveBeenCalledTimes(1);
		expect(p.unlisten).toHaveBeenCalledTimes(1);
		theme.mockClear();
		sys.flip(false);
		sys.focus();
		await settle();
		expect(theme).not.toHaveBeenCalled();
	});
});

describe('syncSystemTheme source priority: portal, then Tauri theme(), then media query', () => {
	beforeEach(() => {
		vi.spyOn(console, 'warn').mockImplementation(() => undefined);
		vi.spyOn(console, 'info').mockImplementation(() => undefined);
	});
	afterEach(() => vi.restoreAllMocks());

	test('portal preference wins over Tauri and the media query (GNOME prefer-dark + Adwaita)', async () => {
		const root = fakeRoot();
		const sys = fakeEnv(false);
		const w = fakeWindow(async () => 'light');
		const p = fakePortal(async () => 'dark');
		await syncSystemTheme(w.win, { root, env: sys.env, portal: p.portal });
		expect(root.dataset.theme).toBe('dark');

		w.emit('light');
		sys.flip(false);
		sys.focus();
		await settle();
		expect(root.dataset.theme).toBe('dark');
	});

	test('portal "no preference" falls back to Tauri theme()', async () => {
		const root = fakeRoot();
		const p = fakePortal(async () => null);
		await syncSystemTheme(fakeWindow(async () => 'dark').win, { root, portal: p.portal });
		expect(root.dataset.theme).toBe('dark');
	});

	test('portal changes apply live; switching to no preference returns to the fallback chain', async () => {
		const root = fakeRoot();
		const p = fakePortal(async () => 'light');
		await syncSystemTheme(fakeWindow(async () => 'light').win, { root, portal: p.portal });
		expect(root.dataset.theme).toBe('light');

		p.change('dark');
		await settle();
		expect(root.dataset.theme).toBe('dark');

		p.change(null);
		await settle();
		expect(root.dataset.theme).toBe('light');
	});

	test('portal missing, failing or slow: Tauri theme() decides, then the media query', async () => {
		for (const read of [
			async (): Promise<SystemTheme | null> => {
				throw new Error('no portal');
			},
			() => new Promise<SystemTheme | null>(() => undefined),
		]) {
			const root = fakeRoot();
			const p = fakePortal(read);
			await syncSystemTheme(fakeWindow(async () => 'dark').win, { root, portalTimeoutMs: 10, portal: p.portal });
			expect(root.dataset.theme).toBe('dark');

			const bare = fakeRoot();
			await syncSystemTheme(fakeWindow(async () => null).win, { root: bare, portalTimeoutMs: 10, portal: fakePortal(read).portal });
			expect(bare.dataset.theme).toBeUndefined();
		}
	});

	test('a change event during the startup read is not overwritten by the older read', async () => {
		const root = fakeRoot();
		let answer: (t: SystemTheme | null) => void = () => undefined;
		const p = fakePortal(() => new Promise((r) => {
			answer = r;
		}));
		const done = syncSystemTheme(fakeWindow(async () => 'light').win, { root, portal: p.portal });
		await settle();
		p.change('dark');
		answer('light');
		await done;
		expect(root.dataset.theme).toBe('dark');
	});

	test('the backend set_theme echo (media query flips too) converges in either order, without loops', async () => {
		// After set_theme the media query and theme() follow the portal (GTK prefer-dark).
		let gtkDark = true;
		const theme = vi.fn(async (): Promise<SystemTheme> => (gtkDark ? 'dark' : 'light'));
		const root = fakeRoot();
		const sys = fakeEnv(true);
		const p = fakePortal(async () => 'dark');
		await syncSystemTheme(fakeWindow(theme).win, { root, env: sys.env, portal: p.portal });
		expect(root.dataset.theme).toBe('dark');
		theme.mockClear();

		// Desktop -> no preference: the media query change lands before the portal event.
		gtkDark = false;
		sys.flip(false);
		await settle();
		p.change(null);
		await settle();
		expect(root.dataset.theme).toBe('light');
		expect(theme.mock.calls.length).toBeLessThanOrEqual(1);

		// Desktop -> dark: portal event first, then the media query echo.
		theme.mockClear();
		p.change('dark');
		gtkDark = true;
		sys.flip(true);
		await settle();
		expect(root.dataset.theme).toBe('dark');
		expect(theme).not.toHaveBeenCalled();
	});
});
