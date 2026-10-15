import type { TransferView } from '../types/qsd';
import { isTerminalState } from '../types/qsd';

/** Client-side timestamps kept by the transfers store (`timing`). */
export interface TransferTiming {
	startedAt: number;
	endedAt: number | null;
}

export interface TransferGroups {
	/** Active or still in handshake, newest start first. */
	now: TransferView[];
	/** Ended since local midnight, newest end first. */
	today: TransferView[];
	/** Ended before local midnight, newest end first. */
	earlier: TransferView[];
}

/** Local midnight of the calendar day containing `ts`. */
export function startOfLocalDay(ts: number): number {
	const d = new Date(ts);
	d.setHours(0, 0, 0, 0);
	return d.getTime();
}

/** Inbound request waiting for Accept/Decline: shown above the list, not in a group. */
export function isPendingConsent(t: TransferView): boolean {
	return t.kind === 'Inbound' && t.state === 'WaitingForUserConsent';
}

/**
 * Activity groups. A non-terminal transfer is always in `now`; a terminal one
 * goes to `today` or `earlier` by its endedAt against local midnight of `now`
 * (calendar day, not the last 24 hours).
 */
export function groupTransfers(
	items: Iterable<TransferView>,
	timing: ReadonlyMap<string, TransferTiming>,
	now: number,
): TransferGroups {
	const midnight = startOfLocalDay(now);
	const started = (t: TransferView) => timing.get(t.id)?.startedAt ?? 0;
	const ended = (t: TransferView) => timing.get(t.id)?.endedAt ?? started(t);
	const groups: TransferGroups = { now: [], today: [], earlier: [] };

	for (const t of items) {
		if (isPendingConsent(t)) continue;
		if (!isTerminalState(t.state)) groups.now.push(t);
		else if (ended(t) >= midnight) groups.today.push(t);
		else groups.earlier.push(t);
	}

	const newestBy = (key: (t: TransferView) => number) =>
		(a: TransferView, b: TransferView) => key(b) - key(a) || a.id.localeCompare(b.id);
	groups.now.sort(newestBy(started));
	groups.today.sort(newestBy(ended));
	groups.earlier.sort(newestBy(ended));
	return groups;
}
