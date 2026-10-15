import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export type SystemTheme = 'light' | 'dark';

/** The part of Tauri's Window used here (lets tests pass a fake). */
export interface ThemeSource {
	theme(): Promise<SystemTheme | null>;
	onThemeChanged(handler: (event: { payload: SystemTheme }) => void): Promise<UnlistenFn>;
}

/** Desktop color-scheme setting (freedesktop portal); null = no preference. */
export interface PortalThemeSource {
	read(): Promise<SystemTheme | null>;
	onChanged(handler: (theme: SystemTheme | null) => void): Promise<UnlistenFn>;
}

/** Backend event for portal changes (src-tauri cmds/system_theme.rs). */
export const QSD_SYSTEM_THEME_EVENT = 'qsd://system-theme';

export const THEME_TIMEOUT_MS = 1500;
/** The backend bounds each portal call to 2 s (connect + read). */
export const PORTAL_TIMEOUT_MS = 5000;

function withTimeout<T>(p: Promise<T>, ms: number): Promise<T> {
	let timer: ReturnType<typeof setTimeout> | undefined;
	const timeout = new Promise<never>((_, reject) => {
		timer = setTimeout(() => reject(new Error(`timed out after ${ms} ms`)), ms);
	});
	return Promise.race([p, timeout]).finally(() => clearTimeout(timer));
}

function asTheme(v: unknown): SystemTheme | null {
	return v === 'dark' || v === 'light' ? v : null;
}

/** Portal source backed by the qsd_system_theme command and qsd://system-theme. */
export function tauriPortal(): PortalThemeSource {
	return {
		read: async () => asTheme(await invoke<string | null>('qsd_system_theme')),
		onChanged: (handler) =>
			listen<{ theme: string | null }>(QSD_SYSTEM_THEME_EVENT, (e) => handler(asTheme(e.payload?.theme))),
	};
}

const DARK_QUERY = '(prefers-color-scheme: dark)';

/** Event sources that trigger a re-read of the Tauri theme (injectable for tests). */
export interface ThemeEnv {
	/** The prefers-color-scheme: dark query; its `change` re-reads the theme. */
	media: (EventTarget & { readonly matches: boolean }) | null;
	/** Gets `focus` events; a re-read on focus covers platforms without ThemeChanged. */
	focus: EventTarget | null;
}

function browserEnv(): ThemeEnv {
	if (typeof window === 'undefined') return { media: null, focus: null };
	return {
		media: typeof window.matchMedia === 'function' ? window.matchMedia(DARK_QUERY) : null,
		focus: window,
	};
}

export interface ThemeOptions {
	root?: HTMLElement;
	timeoutMs?: number;
	portalTimeoutMs?: number;
	env?: ThemeEnv;
	/** Desktop setting; null/absent = skip straight to the Tauri theme. */
	portal?: PortalThemeSource | null;
}

/**
 * Sets html[data-theme] from, in order: the desktop color-scheme setting
 * (portal), Tauri's window theme(), and otherwise nothing, so the CSS
 * prefers-color-scheme sets apply. A portal "no preference" (or a missing
 * portal) falls through to the next source.
 *
 * On Linux tao (0.31) never emits ThemeChanged and theme() only looks at the
 * GTK theme name, as does WebKitGTK's media query: GNOME's "prefer-dark"
 * with Adwaita only shows up through the portal. theme() is re-read on every
 * prefers-color-scheme change and on window focus.
 */
export async function syncSystemTheme(win: ThemeSource, opts: ThemeOptions = {}): Promise<UnlistenFn> {
	const root = opts.root ?? document.documentElement;
	const timeoutMs = opts.timeoutMs ?? THEME_TIMEOUT_MS;
	const env = opts.env ?? browserEnv();
	const portal = opts.portal ?? null;

	let lastMismatch = '';
	const apply = (theme: SystemTheme | null) => {
		if (theme !== 'light' && theme !== 'dark') {
			delete root.dataset.theme;
			return;
		}
		root.dataset.theme = theme;
		const media = env.media ? (env.media.matches ? 'dark' : 'light') : theme;
		const mismatch = media !== theme ? `${theme}/${media}` : '';
		if (mismatch && mismatch !== lastMismatch) {
			console.info(`[theme] using ${theme}, prefers-color-scheme reports ${media}`);
		}
		lastMismatch = mismatch;
	};

	/** Latest desktop setting; when set it wins over everything else. */
	let portalTheme: SystemTheme | null = null;
	// Only the newest resolution may apply (an older, slower theme() must not win).
	let seq = 0;
	const resolve = async () => {
		const mine = ++seq;
		if (portalTheme) {
			apply(portalTheme);
			return;
		}
		try {
			const theme = await withTimeout(win.theme(), timeoutMs);
			if (mine === seq) apply(theme);
		} catch (e) {
			if (mine === seq) apply(null);
			console.warn('[theme] theme() unavailable, following prefers-color-scheme', e);
		}
	};

	const unlisteners: UnlistenFn[] = [];
	let portalRead: Promise<void> | null = null;
	if (portal) {
		let changed = false;
		try {
			unlisteners.push(await portal.onChanged((theme) => {
				changed = true;
				portalTheme = theme;
				void resolve();
			}));
		} catch (e) {
			console.warn('[theme] portal events unavailable', e);
		}
		portalRead = withTimeout(portal.read(), opts.portalTimeoutMs ?? PORTAL_TIMEOUT_MS).then(
			(theme) => {
				// A change event that arrived meanwhile is newer than this read.
				if (!changed) portalTheme = theme;
			},
			(e) => console.warn('[theme] portal unavailable, using the window theme', e),
		);
	}
	try {
		unlisteners.push(await win.onThemeChanged((event) => {
			if (portalTheme) return;
			seq += 1;
			apply(event.payload);
		}));
	} catch (e) {
		console.warn('[theme] onThemeChanged unavailable', e);
	}
	const onChange = () => {
		void resolve();
	};
	env.media?.addEventListener('change', onChange);
	env.focus?.addEventListener('focus', onChange);
	// Fallback chain right away, then the portal value once it answers.
	await resolve();
	if (portalRead) {
		await portalRead;
		if (portalTheme) await resolve();
	}

	return () => {
		unlisteners.forEach((fn) => fn());
		env.media?.removeEventListener('change', onChange);
		env.focus?.removeEventListener('focus', onChange);
	};
}
