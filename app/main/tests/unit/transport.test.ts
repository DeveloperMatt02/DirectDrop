import { beforeEach, describe, expect, test, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { createSSRApp, h } from 'vue';
import { renderToString } from 'vue/server-renderer';
import TransportBadge from '../../src/components/TransportBadge.vue';
import { useTransfersStore } from '../../src/stores/transfers';
import { useToastStore } from '../../src/vue_lib/stores/useToastStore';
import { transportLabel, transportNotice, type TransferDto, type TransportDto } from '../../src/types/qsd';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(async () => undefined) }));

const wifiDirect: TransportDto = {
	medium: 'WifiDirect',
	expected: 'WifiDirect',
	is_fallback: false,
	reason: null,
	reason_text: null,
};
const bleFallback: TransportDto = {
	medium: 'Ble',
	expected: 'WifiDirect',
	is_fallback: true,
	reason: 'peer_join_failed',
	reason_text: "the phone couldn't join the Wi-Fi link",
};
const lanNoHelper: TransportDto = {
	medium: 'WifiLan',
	expected: 'WifiLan',
	is_fallback: false,
	reason: 'helper_missing',
	reason_text: "the Wi-Fi helper isn't installed",
};

function dto(transport: TransportDto | null | undefined, state: TransferDto['state'] = 'ReceivingFiles'): TransferDto {
	return { id: 'ble-1', kind: 'Inbound', state, metadata: null, transport };
}

async function renderBadge(transport: TransportDto | null): Promise<string> {
	return renderToString(createSSRApp({ render: () => h(TransportBadge, { transport }) }));
}

describe('transport notice', () => {
	test('labels', () => {
		expect(transportLabel('Ble')).toBe('Bluetooth');
		expect(transportLabel('BleL2cap')).toBe('Bluetooth');
		expect(transportLabel('WifiDirect')).toBe('Wi-Fi Direct');
	});
	test('only fallbacks get a notice', () => {
		expect(transportNotice(bleFallback)).toBe(
			"Transfer on Bluetooth: Wi-Fi Direct unavailable — the phone couldn't join the Wi-Fi link",
		);
		expect(transportNotice(wifiDirect)).toBeNull();
		expect(transportNotice(lanNoHelper)).toBeNull();
		expect(transportNotice(null)).toBeNull();
		expect(transportNotice({ ...bleFallback, reason_text: null })).toBe(
			'Transfer on Bluetooth: Wi-Fi Direct unavailable',
		);
	});
});

describe('TransportBadge', () => {
	test('medium only when not a fallback', async () => {
		const html = await renderBadge(lanNoHelper);
		expect(html).toContain('WI-FI LAN');
		expect(html).not.toContain('UNAVAILABLE');
	});
	test('fallback shows the reason', async () => {
		const html = await renderBadge(bleFallback);
		expect(html).toContain('BLUETOOTH');
		expect(html).toContain('WI-FI DIRECT UNAVAILABLE');
		expect(html).toContain('JOIN THE WI-FI LINK');
	});
	test('nothing without a transport', async () => {
		expect(await renderBadge(null)).not.toContain('transport-badge');
	});
});

describe('transfers store: transport', () => {
	beforeEach(() => {
		setActivePinia(createPinia());
		vi.useFakeTimers();
	});
	test('keeps the latest transport; a link change is not a no-op', () => {
		const s = useTransfersStore();
		s.upsert(dto({ ...wifiDirect, medium: 'Ble', expected: 'Ble' }));
		expect(s.upsert(dto(wifiDirect))).toBe(true);
		expect(s.items.get('ble-1')!.transport?.medium).toBe('WifiDirect');
		// Updates without a transport (e.g. synthetic terminal ones) keep it.
		s.upsert(dto(null, 'Finished'));
		expect(s.items.get('ble-1')!.transport?.medium).toBe('WifiDirect');
	});
	test('one toast per transfer, only for fallbacks', () => {
		const s = useTransfersStore();
		const toasts = useToastStore();
		s.upsert(dto(lanNoHelper));
		expect(toasts.toasts).toHaveLength(0);
		s.upsert(dto(bleFallback));
		s.upsert(dto({ ...bleFallback, reason: 'timeout', reason_text: 'x' }));
		s.upsert(dto(bleFallback, 'Finished'));
		expect(toasts.toasts).toHaveLength(1);
		expect(toasts.toasts[0].message).toContain('Wi-Fi Direct unavailable');
	});
});
