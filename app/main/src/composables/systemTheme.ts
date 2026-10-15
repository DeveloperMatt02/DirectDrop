import type { UnlistenFn } from '@tauri-apps/api/event';

export type SystemTheme = 'light' | 'dark';

/** The part of Tauri's Window used here (lets tests pass a fake). */
export interface ThemeSource {
	theme(): Promise<SystemTheme | null>;
	onThemeChanged(handler: (event: { payload: SystemTheme }) => void): Promise<UnlistenFn>;
}

export const THEME_TIMEOUT_MS = 1500;

function withTimeout<T>(p: Promise<T>, ms: number): Promise<T> {
	let timer: ReturnType<typeof setTimeout> | undefined;
	const timeout = new Promise<never>((_, reject) => {
		timer = setTimeout(() => reject(new Error(`theme() timed out after ${ms} ms`)), ms);
	});
	return Promise.race([p, timeout]).finally(() => clearTimeout(timer));
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

/**
 * Follows the window theme reported by Tauri by setting html[data-theme].
 * The Tauri value wins; if theme() fails, times out or returns null,
 * data-theme is removed and the CSS prefers-color-scheme sets apply.
 *
 * On Linux tao (0.31) never emits ThemeChanged and theme() only looks at the
 * GTK theme name, so theme() is re-read on every prefers-color-scheme change
 * and on window focus; reading it once at startup pinned the first value
 * until a restart. GNOME's color-scheme setting (Adwaita + "prefer-dark")
 * reaches neither theme() nor WebKitGTK's media query.
 */
export async function syncSystemTheme(
	win: ThemeSource,
	root: HTMLElement = document.documentElement,
	timeoutMs: number = THEME_TIMEOUT_MS,
	env: ThemeEnv = browserEnv(),
): Promise<UnlistenFn> {
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
			console.info(`[theme] Tauri reports ${theme}, prefers-color-scheme reports ${media}; using ${theme}`);
		}
		lastMismatch = mismatch;
	};

	// Only the newest read may apply (an older, slower theme() must not win).
	let seq = 0;
	const refresh = async () => {
		const mine = ++seq;
		try {
			const theme = await withTimeout(win.theme(), timeoutMs);
			if (mine === seq) apply(theme);
		} catch (e) {
			if (mine === seq) apply(null);
			console.warn('[theme] theme() unavailable, following prefers-color-scheme', e);
		}
	};

	let unlistenTauri: UnlistenFn | null = null;
	try {
		unlistenTauri = await win.onThemeChanged((event) => {
			seq += 1;
			apply(event.payload);
		});
	} catch (e) {
		console.warn('[theme] onThemeChanged unavailable', e);
	}
	const onChange = () => {
		void refresh();
	};
	env.media?.addEventListener('change', onChange);
	env.focus?.addEventListener('focus', onChange);
	await refresh();

	return () => {
		unlistenTauri?.();
		env.media?.removeEventListener('change', onChange);
		env.focus?.removeEventListener('focus', onChange);
	};
}
