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

function mediaTheme(): SystemTheme | null {
	if (typeof window === 'undefined' || typeof window.matchMedia !== 'function') return null;
	return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

/**
 * Follows the window theme reported by Tauri by setting html[data-theme].
 * The Tauri value wins; if theme() fails, times out or returns null,
 * data-theme stays unset and the CSS prefers-color-scheme sets apply.
 * WebKitGTK may not follow the desktop theme, so a disagreement between
 * Tauri and the media query is logged.
 */
export async function syncSystemTheme(
	win: ThemeSource,
	root: HTMLElement = document.documentElement,
	timeoutMs: number = THEME_TIMEOUT_MS,
): Promise<UnlistenFn | null> {
	const apply = (theme: SystemTheme | null) => {
		if (theme !== 'light' && theme !== 'dark') return;
		root.dataset.theme = theme;
		const media = mediaTheme();
		if (media !== null && media !== theme) {
			console.info(`[theme] Tauri reports ${theme}, prefers-color-scheme reports ${media}; using ${theme}`);
		}
	};

	let unlisten: UnlistenFn | null = null;
	try {
		unlisten = await win.onThemeChanged((event) => apply(event.payload));
	} catch (e) {
		console.warn('[theme] onThemeChanged unavailable, following prefers-color-scheme', e);
	}
	try {
		apply(await withTimeout(win.theme(), timeoutMs));
	} catch (e) {
		console.warn('[theme] theme() unavailable, following prefers-color-scheme', e);
	}
	return unlisten;
}
