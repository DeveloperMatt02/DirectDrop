import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import type { SnapshotDto, VisibilityDto } from '../types/qsd';

/**
 * Session store: visibility, device_name, download_path, discovery flag.
 * Timer for Temporarily→Invisible (60s) lives in core (mdns.rs); GUI never timers it.
 */
export const useSessionStore = defineStore('session', () => {
	const visibility = ref<VisibilityDto>('Visible');
	const deviceName = ref('');
	const downloadPath = ref<string | null>(null);
	const port = ref<number | null>(null);
	const discoveryRunning = ref(false);
	const isDragHovering = ref(false);
	const blePresenceAt = ref<number | null>(null);
	const initialized = ref(false);
	const initError = ref<string | null>(null);

	async function init() {
		try {
			const snap = await invoke<SnapshotDto>('qsd_snapshot');
			visibility.value = snap.visibility;
			deviceName.value = snap.device_name;
			downloadPath.value = snap.download_path;
			port.value = snap.port;
			discoveryRunning.value = snap.discovery_running;
			initialized.value = true;
		} catch (e) {
			initError.value = e instanceof Error ? e.message : String(e);
		}
	}

	async function setVisibility(v: VisibilityDto) {
		await invoke('qsd_set_visibility', { visibility: v });
		visibility.value = v;
	}

	async function invertVisibility() {
		if (visibility.value === 'Temporarily') return;
		await setVisibility(visibility.value === 'Visible' ? 'Invisible' : 'Visible');
	}

	/** Called from qsd://visibility events (bridge watches core watch::Sender). */
	function applyVisibility(v: VisibilityDto) {
		visibility.value = v;
	}

	function applyBlePresence(at: number) {
		blePresenceAt.value = at;
	}

	async function setDownloadPath(path: string | null) {
		await invoke('qsd_set_download_path', { path });
		downloadPath.value = path;
	}

	async function setDeviceName(name: string) {
		const trimmed = name.trim();
		if (trimmed.length < 1 || trimmed.length > 255) throw new Error('Device name must be 1–255 characters');
		await invoke('qsd_set_device_name', { name: trimmed });
		deviceName.value = trimmed;
	}

	return {
		visibility,
		deviceName,
		downloadPath,
		port,
		discoveryRunning,
		isDragHovering,
		blePresenceAt,
		initialized,
		initError,
		init,
		setVisibility,
		invertVisibility,
		applyVisibility,
		applyBlePresence,
		setDownloadPath,
		setDeviceName,
	};
});
