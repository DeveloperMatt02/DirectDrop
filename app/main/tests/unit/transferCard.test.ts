import { beforeEach, describe, expect, test, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { createSSRApp, h } from 'vue';
import { renderToString } from 'vue/server-renderer';
import TransferCard from '../../src/components/TransferCard.vue';
import { useToastStore } from '../../src/vue_lib/stores/useToastStore';
import { ToastType } from '../../src/vue_lib/types';
import type { TransferView } from '../../src/types/qsd';

const mockInvoke = vi.fn(async () => undefined);
const mockOpen = vi.fn(async () => undefined);

vi.mock('@tauri-apps/api/core', () => ({
	invoke: (...args: unknown[]) => mockInvoke(...args),
}));

vi.mock('@tauri-apps/plugin-shell', () => ({
	open: (...args: unknown[]) => mockOpen(...args),
}));

vi.mock('@tauri-apps/plugin-clipboard-manager', () => ({
	writeText: vi.fn(async () => undefined),
}));

function makeTransferView(
	payloadKind: 'Files' | 'Url' | 'Text' | 'WiFi',
	files: string[] = [],
	url: string | null = null,
	kind: 'Inbound' | 'Outbound' = 'Inbound',
): TransferView {
	return {
		id: 'rx-1',
		kind,
		state: 'Finished',
		metadata: {
			id: 'rx-1',
			source: { name: 'Pixel Phone', device_type: 'Phone' },
			pin_code: null,
			payload_kind: payloadKind,
			payload_preview: null,
			files,
			text: null,
			url,
			wifi_ssid: null,
			total_bytes: 1024,
			ack_bytes: 1024,
		},
		ui: {
			consent_deadline: null,
		},
	};
}

async function renderCard(transfer: TransferView) {
	const vnode = h(TransferCard, { transfer });
	const app = createSSRApp({
		render() {
			return vnode;
		},
	});
	const html = await renderToString(app);
	const exposed = (vnode.component?.exposed ?? {}) as {
		openFile: () => Promise<void>;
		openFolder: () => Promise<void>;
		openTarget: () => Promise<void>;
	};
	return { html, exposed };
}

describe('TransferCard buttons in Finished state (§T2)', () => {
	beforeEach(() => {
		setActivePinia(createPinia());
		mockInvoke.mockClear();
		mockOpen.mockClear();
	});

	test('1 file: renders both OPEN FILE and OPEN FOLDER buttons', async () => {
		const transfer = makeTransferView('Files', ['photo.jpg']);
		const { html } = await renderCard(transfer);

		expect(html).toContain('OPEN FILE');
		expect(html).toContain('OPEN FOLDER');
		expect(html).not.toContain('RX LINK');
	});

	test('N files (3 files): renders only OPEN FOLDER button', async () => {
		const transfer = makeTransferView('Files', ['doc1.pdf', 'doc2.pdf', 'doc3.pdf']);
		const { html } = await renderCard(transfer);

		expect(html).not.toContain('OPEN FILE');
		expect(html).toContain('OPEN FOLDER');
	});

	test('Outbound Finished: no open buttons (1 file and N files)', async () => {
		for (const files of [['photo.jpg'], ['a.pdf', 'b.pdf']]) {
			const { html } = await renderCard(makeTransferView('Files', files, null, 'Outbound'));
			expect(html).not.toContain('OPEN FILE');
			expect(html).not.toContain('OPEN FOLDER');
		}
	});

	test('URL: renders only OPEN button and unchanged for URL', async () => {
		const transfer = makeTransferView('Url', [], 'https://example.com');
		const { html } = await renderCard(transfer);

		expect(html).not.toContain('OPEN FILE');
		expect(html).not.toContain('OPEN FOLDER');
		expect(html).toContain('OPEN');
		expect(html).toContain('COPY LINK');
	});

	test('Text / WiFi: does not render open buttons', async () => {
		const textTransfer = makeTransferView('Text', []);
		const { html: textHtml } = await renderCard(textTransfer);

		expect(textHtml).not.toContain('OPEN FILE');
		expect(textHtml).not.toContain('OPEN FOLDER');
		expect(textHtml).not.toContain('>OPEN<');

		const wifiTransfer = makeTransferView('WiFi', []);
		const { html: wifiHtml } = await renderCard(wifiTransfer);

		expect(wifiHtml).not.toContain('OPEN FILE');
		expect(wifiHtml).not.toContain('OPEN FOLDER');
		expect(wifiHtml).not.toContain('>OPEN<');
	});

	test('1 file openFile invokes qsd_open_path with file name and reveal=false', async () => {
		const transfer = makeTransferView('Files', ['landscape.png']);
		const { exposed } = await renderCard(transfer);

		await exposed.openFile();
		expect(mockInvoke).toHaveBeenCalledTimes(1);
		expect(mockInvoke).toHaveBeenCalledWith('qsd_open_path', {
			name: 'landscape.png',
			reveal: false,
		});
	});

	test('1 file openFolder invokes qsd_open_path with file name and reveal=true', async () => {
		const transfer = makeTransferView('Files', ['landscape.png']);
		const { exposed } = await renderCard(transfer);

		await exposed.openFolder();
		expect(mockInvoke).toHaveBeenCalledTimes(1);
		expect(mockInvoke).toHaveBeenCalledWith('qsd_open_path', {
			name: 'landscape.png',
			reveal: true,
		});
	});

	test('N files openFolder invokes qsd_open_path with name=null and reveal=true', async () => {
		const transfer = makeTransferView('Files', ['f1.txt', 'f2.txt']);
		const { exposed } = await renderCard(transfer);

		await exposed.openFolder();
		expect(mockInvoke).toHaveBeenCalledTimes(1);
		expect(mockInvoke).toHaveBeenCalledWith('qsd_open_path', {
			name: null,
			reveal: true,
		});
	});

	test('URL openTarget opens via plugin-shell open when protocol is http/https', async () => {
		const transfer = makeTransferView('Url', [], 'https://news.ycombinator.com');
		const { exposed } = await renderCard(transfer);

		await exposed.openTarget();
		expect(mockOpen).toHaveBeenCalledTimes(1);
		expect(mockOpen).toHaveBeenCalledWith('https://news.ycombinator.com');
		expect(mockInvoke).not.toHaveBeenCalled();
	});

	test('URL openTarget rejects non-http/https URL', async () => {
		const transfer = makeTransferView('Url', [], 'file:///etc/passwd');
		const toasts = useToastStore();
		const { exposed } = await renderCard(transfer);

		await exposed.openTarget();
		expect(mockOpen).not.toHaveBeenCalled();
		expect(toasts.toasts.length).toBeGreaterThan(0);
		expect(toasts.toasts[0].type).toBe(ToastType.Error);
	});

	test('shows error toast when invoke rejects with error message', async () => {
		mockInvoke.mockRejectedValueOnce({ message: 'File not found in download directory' });
		const transfer = makeTransferView('Files', ['missing.txt']);
		const toasts = useToastStore();
		const { exposed } = await renderCard(transfer);

		await exposed.openFile();
		expect(toasts.toasts.length).toBe(1);
		expect(toasts.toasts[0].type).toBe(ToastType.Error);
		expect(toasts.toasts[0].message).toBe('File not found in download directory');
	});
});
