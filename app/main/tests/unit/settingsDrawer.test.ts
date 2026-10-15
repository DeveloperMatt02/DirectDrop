import { beforeEach, describe, expect, test, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { createSSRApp, h, type Ref } from 'vue';
import { renderToString } from 'vue/server-renderer';
import SettingsDrawer from '../../src/composables/SettingsDrawer.vue';

const saved = new Map<string, unknown>();
const store = {
	get: vi.fn(async (k: string) => saved.get(k)),
	set: vi.fn(async (k: string, v: unknown) => {
		saved.set(k, v);
	}),
	save: vi.fn(async () => undefined),
};
const autostartEnabled = vi.fn(async () => false);

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(async () => undefined) }));
vi.mock('@tauri-apps/api/app', () => ({ getVersion: vi.fn(async () => '0.0.0') }));
vi.mock('@tauri-apps/plugin-dialog', () => ({ open: vi.fn(async () => null) }));
vi.mock('@tauri-apps/plugin-store', () => ({ getStore: vi.fn(async () => store) }));
vi.mock('@tauri-apps/plugin-autostart', () => ({
	isEnabled: () => autostartEnabled(),
	enable: vi.fn(async () => undefined),
	disable: vi.fn(async () => undefined),
}));

interface Exposed {
	loadPrefs: () => Promise<void>;
	toggleKeepRunning: () => Promise<void>;
	keepRunning: Ref<boolean>;
	realclose: Ref<boolean>;
}

async function renderDrawer() {
	const vnode = h(SettingsDrawer, { open: true });
	const html = await renderToString(createSSRApp({ render: () => vnode }));
	return { html, exposed: vnode.component!.exposed as unknown as Exposed };
}

/** The KEEP RUNNING ON CLOSE row: <button aria-pressed=…>…label…ON|OFF</button>. */
function keepRunningRow(html: string) {
	const m = html.match(/<button[^>]*aria-pressed="(true|false)"[^>]*>\s*<span[^>]*>KEEP RUNNING ON CLOSE<\/span>\s*<span[^>]*>(ON|OFF)<\/span>/);
	return m ? { pressed: m[1], shown: m[2] } : null;
}

describe('SettingsDrawer: KEEP RUNNING ON CLOSE', () => {
	beforeEach(() => {
		setActivePinia(createPinia());
		saved.clear();
		vi.clearAllMocks();
	});

	test('default realclose=false (window hides, app keeps running) shows ON, aria-pressed true, with the hint', async () => {
		const { html } = await renderDrawer();
		expect(keepRunningRow(html)).toEqual({ pressed: 'true', shown: 'ON' });
		expect(html).toContain('ON: CLOSING THE WINDOW HIDES IT AND DIRECTDROP KEEPS RUNNING. OFF: CLOSING THE WINDOW QUITS.');
	});

	test('stored realclose=true reads as OFF; a click saves realclose=false (ON), another saves true (OFF)', async () => {
		saved.set('realclose', true);
		const { exposed } = await renderDrawer();
		await exposed.loadPrefs();
		expect(exposed.keepRunning.value).toBe(false);

		await exposed.toggleKeepRunning();
		expect(store.set).toHaveBeenLastCalledWith('realclose', false);
		expect(store.save).toHaveBeenCalled();
		expect(saved.get('realclose')).toBe(false);
		expect(exposed.keepRunning.value).toBe(true);

		await exposed.toggleKeepRunning();
		expect(saved.get('realclose')).toBe(true);
		expect(exposed.keepRunning.value).toBe(false);
	});

	test('a failed save keeps the shown state', async () => {
		const { exposed } = await renderDrawer();
		store.set.mockRejectedValueOnce(new Error('disk full'));
		vi.spyOn(console, 'error').mockImplementation(() => undefined);
		await exposed.toggleKeepRunning();
		expect(exposed.keepRunning.value).toBe(true);
	});
});
