import { beforeEach, describe, expect, test, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { useTransfersStore } from '../../src/stores/transfers';
import { bleKey, useDevicesStore } from '../../src/stores/devices';
import {
	HANDSHAKE_STATES,
	bleReceiverLabel,
	describeTransfer,
	humanSize,
	validateSendFiles,
	type BleReceiverDto,
	type TransferDto,
} from '../../src/types/qsd';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(async () => undefined) }));

function consentDto(id = '192.168.1.5:51234'): TransferDto {
	return {
		id,
		kind: 'Inbound',
		state: 'WaitingForUserConsent',
		metadata: {
			id,
			source: { name: 'Pixel 10', device_type: 'Phone' },
			pin_code: '4821',
			payload_kind: 'Files',
			payload_preview: null,
			files: ['IMG_001.jpg', 'video.mp4'],
			text: null,
			url: null,
			wifi_ssid: null,
			total_bytes: 38210432,
			ack_bytes: 0,
		},
	};
}

describe('transfers upsert (idempotent per id, §2.4)', () => {
	beforeEach(() => {
		setActivePinia(createPinia());
		vi.useFakeTimers();
	});
	test('inserts then ignores no-op repeats', () => {
		const s = useTransfersStore();
		expect(s.upsert(consentDto())).toBe(true);
		expect(s.upsert(consentDto())).toBe(false);
		expect(s.items.size).toBe(1);
	});
	test('updates on state/ack change, sets consent deadline', () => {
		const s = useTransfersStore();
		s.upsert(consentDto());
		const v = s.items.get('192.168.1.5:51234')!;
		expect(v.ui.consent_deadline).toBeGreaterThan(Date.now());
		const progress = { ...consentDto(), state: 'ReceivingFiles' as const };
		expect(s.upsert(progress)).toBe(true);
		expect(s.items.get(progress.id)!.state).toBe('ReceivingFiles');
	});
	test('present:false removes endpoint', () => {
		const d = useDevicesStore();
		d.upsertEndpoint({ id: 'a', fullname: 'a', name: 'P', ip: '1.1.1.1', port: '1', device_type: 'Phone', present: true });
		expect(d.endpoints.size).toBe(1);
		d.upsertEndpoint({ id: 'a', fullname: 'a', name: null, ip: null, port: null, device_type: null, present: false });
		expect(d.endpoints.size).toBe(0);
	});
});

describe('CLI parity: describe + sizes', () => {
	test('files: names + size, truncation over 3', () => {
		const m = consentDto().metadata!;
		expect(describeTransfer(m)).toContain('IMG_001.jpg');
		expect(describeTransfer({ ...m, files: ['a', 'b', 'c', 'd'] })).toContain('4 files');
	});
	test('text/url/wifi labels', () => {
		const base = consentDto().metadata!;
		expect(describeTransfer({ ...base, files: null, payload_kind: 'Text', payload_preview: 'hi' })).toContain('Text');
		expect(describeTransfer({ ...base, files: null, payload_kind: 'Url', url: 'https://x' })).toBe('https://x');
		expect(describeTransfer({ ...base, files: null, payload_kind: 'WiFi', wifi_ssid: 'home' })).toContain('home');
	});
	test('humanSize', () => {
		expect(humanSize(0)).toBe('0 B');
		expect(humanSize(38210432)).toMatch(/MB/);
	});
});

describe('send validation: regular files only', () => {
	test('empty rejected, folders rejected', () => {
		expect(validateSendFiles([]).ok).toBe(false);
		expect(validateSendFiles(['/tmp/x/']).ok).toBe(false);
		expect(validateSendFiles(['/home/u/Docs/report.pdf']).ok).toBe(true);
	});
});

describe('BLE receivers (plan §3)', () => {
	beforeEach(() => {
		setActivePinia(createPinia());
		vi.clearAllMocks();
	});
	test('bleReceiverLabel mirrors CLI receiver_label', () => {
		expect(bleReceiverLabel({ name: 'Pixel 10', endpoint_id: 'ab12' })).toBe('Pixel 10');
		expect(bleReceiverLabel({ name: null, endpoint_id: 'ab12' })).toBe('unnamed device ab12');
	});
	test('scanBle replaces map contents, tracks lastScanAt', async () => {
		const rows: BleReceiverDto[] = [
			{ address: 'AA:BB:CC:DD:EE:FF', name: 'Pixel 10', endpoint_id: 'ab12' },
		];
		vi.mocked(invoke).mockResolvedValueOnce(rows);
		const d = useDevicesStore();
		const res = await d.scanBle();
		expect(res).toEqual(rows);
		expect(d.bleReceivers.size).toBe(1);
		expect(d.bleReceivers.get(bleKey(rows[0]))).toEqual(rows[0]);
		expect(d.lastScanAt).not.toBeNull();
		expect(vi.mocked(invoke)).toHaveBeenCalledWith('qsd_scan_ble', {
			m: { name: null, timeout_s: null },
		});
		// Seconda scan vuota svuota la lista (telefono uscito da Receive).
		vi.mocked(invoke).mockResolvedValueOnce([]);
		await d.scanBle();
		expect(d.bleReceivers.size).toBe(0);
		d.clearBle();
		expect(d.lastScanAt).toBeNull();
	});
	test('outbound-waiting: pre-SendingFiles Outbound states never show Accept/Decline', () => {
		// Logica mirror di TransferCard.isOutboundWaiting: WaitingForUserConsent
		// o HANDSHAKE su Outbound -> waiting (no bottoni); su Inbound -> consenso.
		const isOutboundWaiting = (t: Pick<TransferDto, 'kind' | 'state'>) =>
			t.kind === 'Outbound'
			&& (t.state === 'WaitingForUserConsent' || HANDSHAKE_STATES.has(t.state));
		expect(isOutboundWaiting({ kind: 'Outbound', state: 'WaitingForUserConsent' })).toBe(true);
		expect(isOutboundWaiting({ kind: 'Outbound', state: 'SentUkeyClientInit' })).toBe(true);
		expect(isOutboundWaiting({ kind: 'Outbound', state: 'SendingFiles' })).toBe(false);
		expect(isOutboundWaiting({ kind: 'Inbound', state: 'WaitingForUserConsent' })).toBe(false);
	});
});
