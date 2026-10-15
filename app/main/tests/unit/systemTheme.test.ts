import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import { syncSystemTheme, type SystemTheme, type ThemeEnv, type ThemeSource } from '../../src/composables/systemTheme';

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

const settle = () => new Promise((r) => setTimeout(r, 0));

describe('syncSystemTheme', () => {
	beforeEach(() => {
		vi.spyOn(console, 'warn').mockImplementation(() => undefined);
	});
	afterEach(() => vi.restoreAllMocks());

	test('Tauri theme sets data-theme and follows changes', async () => {
		const root = fakeRoot();
		const { win, unlisten, emit } = fakeWindow(async () => 'dark');
		const off = await syncSystemTheme(win, root);
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
			await syncSystemTheme(fakeWindow(theme).win, root, 10);
			expect(root.dataset.theme).toBeUndefined();
		}
	});

	test('a system switch without ThemeChanged (Linux) still updates data-theme', async () => {
		let tauri: SystemTheme = 'light';
		const root = fakeRoot();
		const sys = fakeEnv(false);
		await syncSystemTheme(fakeWindow(async () => tauri).win, root, 50, sys.env);
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
		await syncSystemTheme(fakeWindow(async () => {
			if (fail) throw new Error('gone');
			return 'light';
		}).win, root, 50, sys.env);
		expect(root.dataset.theme).toBe('light');
		fail = true;
		sys.flip(true);
		await settle();
		expect(root.dataset.theme).toBeUndefined();
	});

	test('cleanup removes the Tauri listener and the change/focus listeners', async () => {
		const theme = vi.fn(async (): Promise<SystemTheme> => 'dark');
		const { win, unlisten } = fakeWindow(theme);
		const sys = fakeEnv(true);
		const off = await syncSystemTheme(win, fakeRoot(), 50, sys.env);
		off();
		expect(unlisten).toHaveBeenCalledTimes(1);
		theme.mockClear();
		sys.flip(false);
		sys.focus();
		await settle();
		expect(theme).not.toHaveBeenCalled();
	});
});
