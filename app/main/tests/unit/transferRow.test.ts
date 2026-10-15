import { beforeEach, describe, expect, test, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { createSSRApp, h, type Component } from 'vue';
import { renderToString } from 'vue/server-renderer';
import TransferCard from '../../src/components/TransferCard.vue';
import TransferQueue from '../../src/components/TransferQueue.vue';
import { avgSpeedBps, formatClock } from '../../src/stores/transferGroups';
import { useTransfersStore } from '../../src/stores/transfers';
import type { TransferKindDto, TransferStateDto, TransferView, TransportDto } from '../../src/types/qsd';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(async () => undefined) }));
vi.mock('@tauri-apps/plugin-shell', () => ({ open: vi.fn(async () => undefined) }));
vi.mock('@tauri-apps/plugin-clipboard-manager', () => ({ writeText: vi.fn(async () => undefined) }));

const MB = 1024 * 1024;

const bleFallback: TransportDto = {
	medium: 'Ble',
	expected: 'WifiDirect',
	is_fallback: true,
	reason: 'wifi_off',
	reason_text: 'Wi-Fi is off on this PC',
};

function view(
	state: TransferStateDto,
	kind: TransferKindDto = 'Inbound',
	opts: { id?: string; files?: string[]; total?: number; ack?: number; transport?: TransportDto | null } = {},
): TransferView {
	const id = opts.id ?? 'rx-1';
	const total = opts.total ?? 4 * MB;
	return {
		id,
		kind,
		state,
		metadata: {
			id,
			source: { name: 'Pixel 10', device_type: 'Phone' },
			pin_code: null,
			payload_kind: 'Files',
			payload_preview: null,
			files: opts.files ?? ['photo.jpg'],
			text: null,
			url: null,
			wifi_ssid: null,
			total_bytes: total,
			ack_bytes: opts.ack ?? total,
		},
		transport: opts.transport ?? null,
		ui: { consent_deadline: null, expired: false },
	};
}

function render(component: Component, props: Record<string, unknown> = {}): Promise<string> {
	return renderToString(createSSRApp({ render: () => h(component, props) }));
}

const button = (label: string) => new RegExp(`>\\s*${label}\\s*</button>`);

describe('avgSpeedBps', () => {
	test('total bytes over endedAt − startedAt', () => {
		expect(avgSpeedBps(8 * MB, { startedAt: 1000, endedAt: 5000 })).toBe(2 * MB);
	});
	test('null when the transfer lasted 1 s or less, has not ended, or is empty', () => {
		expect(avgSpeedBps(8 * MB, { startedAt: 1000, endedAt: 2000 })).toBeNull();
		expect(avgSpeedBps(8 * MB, { startedAt: 1000, endedAt: null })).toBeNull();
		expect(avgSpeedBps(0, { startedAt: 1000, endedAt: 9000 })).toBeNull();
		expect(avgSpeedBps(8 * MB, undefined)).toBeNull();
	});
});

describe('formatClock', () => {
	test('time only on the same local day, date and time otherwise', () => {
		const now = new Date(2026, 9, 16, 18, 0).getTime();
		expect(formatClock(new Date(2026, 9, 16, 9, 5).getTime(), now)).toBe('09:05');
		expect(formatClock(new Date(2026, 9, 15, 23, 59).getTime(), now)).toBe('15/10 23:59');
	});
});

describe('transfer row', () => {
	beforeEach(() => setActivePinia(createPinia()));

	test('finished: AVG speed when it lasted more than 1 s', async () => {
		useTransfersStore().timing.set('rx-1', { startedAt: 1000, endedAt: 5000 });
		const html = await render(TransferCard, { transfer: view('Finished') });
		expect(html).toContain('data-test="avg-speed"');
		expect(html).toContain('AVG 1 MB/s');
	});

	test('finished in 1 s or less, or not finished: no AVG', async () => {
		const s = useTransfersStore();
		s.timing.set('rx-1', { startedAt: 1000, endedAt: 2000 });
		expect(await render(TransferCard, { transfer: view('Finished') })).not.toContain('AVG');
		s.timing.set('rx-1', { startedAt: 1000, endedAt: 9000 });
		expect(await render(TransferCard, { transfer: view('Cancelled') })).not.toContain('AVG');
	});

	test('active row: badge, fallback notice, live speed, ETA and CANCEL in the row', async () => {
		const html = await render(TransferCard, {
			transfer: view('ReceivingFiles', 'Inbound', { ack: MB, transport: bleFallback }),
		});
		expect(html).toContain('data-test="transport-badge"');
		expect(html).toContain('BLUETOOTH');
		expect(html).toContain('TRANSFER ON BLUETOOTH: WI-FI DIRECT UNAVAILABLE — WI-FI IS OFF ON THIS PC');
		expect(html).toMatch(/RX 0 B\/s/);
		expect(html).toContain('ETA');
		expect(html).toContain('role="progressbar"');
		expect(html).toMatch(button('CANCEL'));
		expect(html).not.toMatch(button('CLEAR'));
		expect(html).not.toContain('OPEN FOLDER');
	});

	test('inbound finished: received title, open buttons, CLEAR, no CANCEL', async () => {
		const html = await render(TransferCard, { transfer: view('Finished', 'Inbound') });
		expect(html).toContain('data-test="transfer-inbound"');
		expect(html).toContain('RECEIVED FROM');
		expect(html).toMatch(button('OPEN FILE'));
		expect(html).toMatch(button('OPEN FOLDER'));
		expect(html).toMatch(button('CLEAR'));
		expect(html).not.toMatch(button('CANCEL'));
	});

	test('outbound finished: sent title, no open buttons', async () => {
		const html = await render(TransferCard, { transfer: view('Finished', 'Outbound') });
		expect(html).toContain('data-test="transfer-outbound"');
		expect(html).toContain('SENT TO');
		expect(html).not.toContain('OPEN FILE');
		expect(html).not.toContain('OPEN FOLDER');
		expect(html).toMatch(button('CLEAR'));
	});

	test('outbound cancelled: "SEND TO X CANCELLED", CLEAR only', async () => {
		const html = await render(TransferCard, { transfer: view('Cancelled', 'Outbound') });
		expect(html).toContain('SEND TO');
		expect(html).toContain('CANCELLED');
		expect(html).toMatch(button('CLEAR'));
		expect(html).not.toMatch(button('CANCEL'));
	});

	test('outbound sending: TX speed and CANCEL, no open buttons', async () => {
		const html = await render(TransferCard, {
			transfer: view('SendingFiles', 'Outbound', { ack: MB }),
		});
		expect(html).toContain('SENDING TO');
		expect(html).toMatch(/TX 0 B\/s/);
		expect(html).toMatch(button('CANCEL'));
		expect(html).not.toContain('OPEN FOLDER');
	});
});

describe('activity list', () => {
	beforeEach(() => setActivePinia(createPinia()));

	test('shows only the non-empty groups', async () => {
		const s = useTransfersStore();
		s.items.set('a', view('ReceivingFiles', 'Inbound', { id: 'a', ack: MB }));
		s.items.set('b', view('Finished', 'Outbound', { id: 'b' }));
		const t = Date.now();
		s.timing.set('a', { startedAt: t, endedAt: null });
		s.timing.set('b', { startedAt: t - 5000, endedAt: t });
		const html = await render(TransferQueue);
		expect(html).toContain('data-test="group-now"');
		expect(html).toContain('data-test="group-today"');
		expect(html).not.toContain('data-test="group-earlier"');
		expect(html).not.toContain('NO ACTIVITY YET');
	});

	test('empty list shows the empty state', async () => {
		const html = await render(TransferQueue);
		expect(html).toContain('NO ACTIVITY YET');
		expect(html).not.toContain('data-test="group-');
	});
});
