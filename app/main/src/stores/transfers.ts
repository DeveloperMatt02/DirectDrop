import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { computed, ref } from 'vue';
import {
	QSD_CONSENT_TIMEOUT_MS,
	type ConsentDto,
	type SendBleInput,
	type SendPayloadDto,
	type TransferActionDto,
	type TransferDto,
	type TransferView,
} from '../types/qsd';

/**
 * Transfers store (F1/F2). Core repeats state updates for the same id —
 * upsert is idempotent per id and drops no-op updates with the same
 * (state, ack_bytes, total_bytes) triple (§2.4).
 *
 * Consent deadline: now + QSD_CONSENT_TIMEOUT (60s, PRODUCT.md). On expiry
 * without user action the store sends ConsentDecline (CLI parity) and marks
 * expired:true. Countdown display reads ui.consent_deadline.
 */
export const useTransfersStore = defineStore('transfers', () => {
	const items = ref(new Map<string, TransferView>());
	const expiryTimers = new Map<string, ReturnType<typeof setTimeout>>();

	function isNoop(prev: TransferView | undefined, dto: TransferDto): boolean {
		if (!prev) return false;
		return (
			prev.state === dto.state &&
			(prev.metadata?.ack_bytes ?? 0) === (dto.metadata?.ack_bytes ?? 0) &&
			(prev.metadata?.total_bytes ?? 0) === (dto.metadata?.total_bytes ?? 0)
		);
	}

	function scheduleExpiry(id: string) {
		clearExpiry(id);
		expiryTimers.set(
			id,
			setTimeout(async () => {
				const cur = items.value.get(id);
				if (!cur || cur.state !== 'WaitingForUserConsent' || cur.ui.expired) return;
				try {
					await consent(id, 'ConsentDecline');
				} catch {
					// fire-and-forget like CLI; keep card visible with expired flag
				} finally {
					const next = items.value.get(id);
					if (next) items.value.set(id, { ...next, ui: { ...next.ui, expired: true } });
				}
			}, QSD_CONSENT_TIMEOUT_MS),
		);
	}

	function clearExpiry(id: string) {
		const t = expiryTimers.get(id);
		if (t) clearTimeout(t);
		expiryTimers.delete(id);
	}

	/** Idempotent upsert. Returns true if the map changed. */
	function upsert(dto: TransferDto): boolean {
		const prev = items.value.get(dto.id);
		if (isNoop(prev, dto)) return false;

		const view: TransferView = {
			...dto,
			metadata: dto.metadata ? { ...dto.metadata } : null,
			ui: prev?.ui ?? { consent_deadline: null, expired: false },
		};

		if (dto.state === 'WaitingForUserConsent' && prev?.state !== 'WaitingForUserConsent') {
			view.ui = { consent_deadline: Date.now() + QSD_CONSENT_TIMEOUT_MS, expired: false };
			scheduleExpiry(dto.id);
		} else if (dto.state !== 'WaitingForUserConsent') {
			clearExpiry(dto.id);
			view.ui.consent_deadline = null;
		}

		items.value.set(dto.id, view);
		return true;
	}

	async function consent(id: string, action: TransferActionDto) {
		const dto: ConsentDto = { id, action };
		await invoke('qsd_consent', { m: dto });
		if (action !== 'TransferCancel') clearExpiry(id);
	}

	async function cancel(id: string) {
		// BLE sends (`ble-`): qsd_cancel_send fa scattare il CancellationToken
		// (fase pre-handshake) + invia TransferCancel (onorato da OutboundRequest).
		if (id.startsWith('ble-')) {
			try {
				await invoke('qsd_cancel_send', { send_id: id });
				return;
			} catch {
				// fallback al path consenso sotto
			}
		}
		await consent(id, 'TransferCancel');
	}

	function clear(id: string) {
		clearExpiry(id);
		items.value.delete(id);
	}

	async function sendLan(payload: SendPayloadDto) {
		await invoke('qsd_send_lan', { m: payload });
	}

	/** BLE send: ritorna il send_id (`ble-…`) per il tracking/cancel. */
	async function sendBle(payload: SendBleInput): Promise<string> {
		return await invoke<string>('qsd_send_ble', { m: payload });
	}

	const inbound = computed(() =>
		[...items.value.values()].filter((t) => t.kind === 'Inbound'),
	);
	const outbound = computed(() =>
		[...items.value.values()].filter((t) => t.kind === 'Outbound'),
	);
	const inboundSorted = computed(() =>
		[...inbound.value].sort((a, b) => (b.ui.consent_deadline ?? 0) - (a.ui.consent_deadline ?? 0)),
	);

	return { items, inbound, outbound, inboundSorted, upsert, consent, cancel, clear, sendLan, sendBle };
});
