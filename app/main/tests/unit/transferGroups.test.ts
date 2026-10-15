import { describe, expect, test } from 'vitest';
import { groupTransfers, startOfLocalDay, type TransferTiming } from '../../src/stores/transferGroups';
import type { TransferKindDto, TransferStateDto, TransferView } from '../../src/types/qsd';

// Local-time dates, so the tests hold in any time zone.
const at = (day: number, h: number, m = 0) => new Date(2026, 9, day, h, m).getTime();

function view(id: string, state: TransferStateDto, kind: TransferKindDto = 'Inbound'): TransferView {
	return {
		id,
		kind,
		state,
		metadata: null,
		ui: { consent_deadline: null, expired: false },
	};
}

const ids = (list: TransferView[]) => list.map((t) => t.id);

describe('groupTransfers', () => {
	test('today vs earlier follows local midnight, not the last 24 hours', () => {
		const now = at(16, 0, 30);
		const timing = new Map<string, TransferTiming>([
			['just-after', { startedAt: at(15, 23, 58), endedAt: at(16, 0, 1) }],
			['just-before', { startedAt: at(15, 23, 50), endedAt: at(15, 23, 59) }],
			['yesterday-noon', { startedAt: at(15, 12), endedAt: at(15, 12, 5) }],
		]);
		const g = groupTransfers(
			[view('just-after', 'Finished'), view('just-before', 'Finished'), view('yesterday-noon', 'Cancelled')],
			timing,
			now,
		);
		expect(ids(g.today)).toEqual(['just-after']);
		expect(ids(g.earlier)).toEqual(['just-before', 'yesterday-noon']);
		expect(g.now).toEqual([]);
	});

	test('an ended transfer is grouped by endedAt, not startedAt', () => {
		const timing = new Map([['a', { startedAt: at(15, 23, 55), endedAt: at(16, 0, 5) }]]);
		const g = groupTransfers([view('a', 'Finished')], timing, at(16, 9));
		expect(ids(g.today)).toEqual(['a']);
	});

	test('active and handshake transfers are always NOW, even when started days ago', () => {
		const timing = new Map<string, TransferTiming>([
			['old-active', { startedAt: at(10, 8), endedAt: null }],
			['handshake', { startedAt: at(16, 9), endedAt: null }],
			['waiting-phone', { startedAt: at(16, 8), endedAt: null }],
		]);
		const g = groupTransfers(
			[
				view('old-active', 'ReceivingFiles'),
				view('handshake', 'SentUkeyServerInit'),
				view('waiting-phone', 'WaitingForUserConsent', 'Outbound'),
			],
			timing,
			at(16, 10),
		);
		expect(ids(g.now)).toEqual(['handshake', 'waiting-phone', 'old-active']);
		expect(g.today).toEqual([]);
		expect(g.earlier).toEqual([]);
	});

	test('pending inbound consent is left out of every group', () => {
		const timing = new Map([['c', { startedAt: at(16, 9), endedAt: null }]]);
		const g = groupTransfers([view('c', 'WaitingForUserConsent')], timing, at(16, 10));
		expect([...g.now, ...g.today, ...g.earlier]).toEqual([]);
	});

	test('newest first within each group, ties by id', () => {
		const timing = new Map<string, TransferTiming>([
			['b', { startedAt: at(16, 8), endedAt: at(16, 9) }],
			['a', { startedAt: at(16, 7), endedAt: at(16, 9) }],
			['c', { startedAt: at(16, 9), endedAt: at(16, 9, 30) }],
			['x', { startedAt: at(16, 6), endedAt: null }],
			['y', { startedAt: at(16, 7), endedAt: null }],
		]);
		const g = groupTransfers(
			[view('a', 'Finished'), view('b', 'Rejected'), view('c', 'Disconnected'), view('x', 'SendingFiles'), view('y', 'ReceivingFiles')],
			timing,
			at(16, 10),
		);
		expect(ids(g.today)).toEqual(['c', 'a', 'b']);
		expect(ids(g.now)).toEqual(['y', 'x']);
	});

	test('startOfLocalDay is local midnight', () => {
		expect(startOfLocalDay(at(16, 23, 59))).toBe(at(16, 0));
		expect(startOfLocalDay(at(16, 0))).toBe(at(16, 0));
	});
});
