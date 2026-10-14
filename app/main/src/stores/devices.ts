import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { computed, ref } from 'vue';
import type { EndpointDto } from '../types/qsd';

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

	return { endpoints, sorted, upsertEndpoint, clear, startDiscovery, stopDiscovery };
});
