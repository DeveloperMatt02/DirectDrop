import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { computed, ref } from 'vue';
import type { BleReceiverDto, EndpointDto } from '../types/qsd';

/** Map key = token opaco backend `endpoint_id-address`. */
export function bleKey(r: Pick<BleReceiverDto, 'address' | 'endpoint_id'>): string {
	return `${r.endpoint_id}-${r.address}`;
}

/** mDNS endpoints (LAN). present:false removes by id (§3.2). */
export const useDevicesStore = defineStore('devices', () => {
	const endpoints = ref(new Map<string, EndpointDto>());

	const sorted = computed(() =>
		[...endpoints.value.values()].sort((a, b) => (a.name ?? a.id).localeCompare(b.name ?? b.id)),
	);

	/** Idempotent upsert; removal is explicit via present:false. */
	function upsertEndpoint(dto: EndpointDto) {
		if (!dto.present) {
			endpoints.value.delete(dto.id);
			return;
		}
		endpoints.value.set(dto.id, dto);
	}

	function clear() {
		endpoints.value.clear();
	}

	async function startDiscovery() {
		await invoke('qsd_start_discovery');
	}

	async function stopDiscovery() {
		await invoke('qsd_stop_discovery');
	}

	// -- BLE receivers (one-shot scan, §1.1) ---------------------------------
	const bleReceivers = ref(new Map<string, BleReceiverDto>());
	const scanning = ref(false);
	const lastScanAt = ref<number | null>(null);

	const bleSorted = computed(() =>
		[...bleReceivers.value.values()].sort((a, b) =>
			(a.name ?? a.endpoint_id).localeCompare(b.name ?? b.endpoint_id),
		),
	);

	/** One-shot scan: sostituisce il contenuto della mappa (no streaming). */
	async function scanBle(name?: string) {
		scanning.value = true;
		try {
			const res = await invoke<BleReceiverDto[]>('qsd_scan_ble', {
				m: { name: name ?? null, timeout_s: null },
			});
			bleReceivers.value.clear();
			for (const r of res ?? []) bleReceivers.value.set(bleKey(r), r);
			lastScanAt.value = Date.now();
			return res ?? [];
		} finally {
			scanning.value = false;
		}
	}

	function clearBle() {
		bleReceivers.value.clear();
		lastScanAt.value = null;
	}

	return {
		endpoints,
		sorted,
		upsertEndpoint,
		clear,
		startDiscovery,
		stopDiscovery,
		bleReceivers,
		bleSorted,
		scanning,
		lastScanAt,
		scanBle,
		clearBle,
	};
});
