import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import { syncSystemTheme, type SystemTheme, type ThemeSource } from '../../src/composables/systemTheme';

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
		expect(off).toBe(unlisten);
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
});
