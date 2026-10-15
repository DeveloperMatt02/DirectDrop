import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { useTransfersStore } from '../../src/stores/transfers';
import type { TransferDto, TransferStateDto } from '../../src/types/qsd';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(async () => undefined) }));

function dto(id: string, state: TransferStateDto = 'Initial', ack = 0): TransferDto {
	return {
		id,
		kind: 'Outbound',
		state,
		metadata: {
			id,
			source: { name: 'Pixel', device_type: 'Phone' },
			pin_code: null,
			payload_kind: 'Files',
			payload_preview: null,
			files: ['a.jpg'],
			text: null,
			url: null,
			wifi_ssid: null,
			total_bytes: 100,
			ack_bytes: ack,
		},
	};
}

describe('transfer ordering and timing', () => {
	beforeEach(() => {
		vi.useFakeTimers();
		vi.setSystemTime(1000);
		setActivePinia(createPinia());
	});
	afterEach(() => vi.useRealTimers());

	test('newest first; progress on the oldest does not reorder', () => {
		const s = useTransfersStore();
		s.upsert(dto('a'));
		vi.setSystemTime(2000);
		s.upsert(dto('b'));
		vi.setSystemTime(3000);
		s.upsert(dto('c'));
		expect(s.sortedItems.map((t) => t.id)).toEqual(['c', 'b', 'a']);

		vi.setSystemTime(4000);
		s.upsert(dto('a', 'ReceivingFiles', 50));
		expect(s.sortedItems.map((t) => t.id)).toEqual(['c', 'b', 'a']);
	});

	test('ties on startedAt resolve by id', () => {
		const s = useTransfersStore();
		s.upsert(dto('b'));
		s.upsert(dto('a'));
		expect(s.sortedItems.map((t) => t.id)).toEqual(['a', 'b']);
	});

	test('startedAt is never overwritten; endedAt is set once', () => {
		const s = useTransfersStore();
		s.upsert(dto('a'));
		vi.setSystemTime(2000);
		s.upsert(dto('a', 'ReceivingFiles', 10));
		expect(s.timing.get('a')).toEqual({ startedAt: 1000, endedAt: null });

		vi.setSystemTime(3000);
		s.upsert(dto('a', 'Finished', 100));
		expect(s.timing.get('a')).toEqual({ startedAt: 1000, endedAt: 3000 });

		vi.setSystemTime(4000);
		s.upsert(dto('a', 'Disconnected', 100));
		expect(s.timing.get('a')).toEqual({ startedAt: 1000, endedAt: 3000 });
	});
});
