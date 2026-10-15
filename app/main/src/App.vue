<script setup lang="ts">
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import DeviceBoard from './components/DeviceBoard.vue';
import SendPane from './components/SendPane.vue';
import StatusHeader from './components/StatusHeader.vue';
import TransferCard from './components/TransferCard.vue';
import TransferQueue from './components/TransferQueue.vue';
import SettingsDrawer from './composables/SettingsDrawer.vue';
import { syncSystemTheme, tauriPortal } from './composables/systemTheme';
import { ToastNotification } from './vue_lib';
import { useToastStore } from './vue_lib/stores/useToastStore';
import { ToastType } from './vue_lib/types';
import { useDevicesStore } from './stores/devices';
import { useSessionStore } from './stores/session';
import { isPendingConsent } from './stores/transferGroups';
import { useTransfersStore } from './stores/transfers';
import {
	QSD_BLE_PRESENCE_EVENT,
	QSD_ENDPOINTS_EVENT,
	QSD_TRANSFER_EVENT,
	QSD_VISIBILITY_EVENT,
	validateSendFiles,
	type EndpointDto,
	type TransferDto,
	type VisibilityDto,
} from './types/qsd';

const session = useSessionStore();
const devices = useDevicesStore();
const transfers = useTransfersStore();
const toasts = useToastStore();

const pendingConsent = computed(() => transfers.inboundSorted.filter(isPendingConsent));

const settingsOpen = ref(false);
const stagedFiles = ref<string[]>([]);
const unlisteners: UnlistenFn[] = [];

function onStaged(files: string[]) {
	stagedFiles.value = files;
}

async function onClearSending() {
	stagedFiles.value = [];
	try {
		await devices.stopDiscovery();
	} catch (e) {
		console.error(e);
	} finally {
		devices.clear();
		session.discoveryRunning = false;
	}
}

onMounted(async () => {
	// Not awaited: theme() may hang on some WebKitGTK setups (bounded by a timeout).
	syncSystemTheme(getCurrentWindow(), { portal: tauriPortal() })
		.then((unlisten) => {
			if (unlisten) unlisteners.push(unlisten);
		})
		.catch((e) => console.error(e));

	await session.init();
	if (session.initError) {
		toasts.addToast(`Failed to load settings: ${session.initError}`, ToastType.Error);
	}

	try {
		await devices.startDiscovery();
		session.discoveryRunning = true;
	} catch (e) {
		toasts.addToast('Discovery unavailable — stage files to retry', ToastType.Error);
		console.error(e);
	}

	try {
		if (!(await isPermissionGranted())) await requestPermission();
	} catch (e) {
		console.error(e);
	}

	unlisteners.push(
		await listen<TransferDto>(QSD_TRANSFER_EVENT, (event) => {
			transfers.upsert(event.payload);
		}),
	);
	unlisteners.push(
		await listen<EndpointDto>(QSD_ENDPOINTS_EVENT, (event) => {
			devices.upsertEndpoint(event.payload);
		}),
	);
	unlisteners.push(
		await listen<{ visibility: VisibilityDto }>(QSD_VISIBILITY_EVENT, (event) => {
			session.applyVisibility(event.payload.visibility);
		}),
	);
	unlisteners.push(
		await listen<{ at: number }>(QSD_BLE_PRESENCE_EVENT, (event) => {
			session.applyBlePresence(event.payload.at);
			toasts.addToast('A phone nearby is sharing — check visibility', ToastType.Info);
		}),
	);
	unlisteners.push(
		await getCurrentWindow().onDragDropEvent(async (event) => {
			if (event.payload.type === 'over') {
				session.isDragHovering = true;
			} else if (event.payload.type === 'drop') {
				session.isDragHovering = false;
				const v = validateSendFiles(event.payload.paths);
				if (!v.ok) {
					toasts.addToast(v.error ?? 'Invalid selection', ToastType.Error);
					return;
				}
				stagedFiles.value = event.payload.paths;
				if (!session.discoveryRunning) {
					try {
						await devices.startDiscovery();
						session.discoveryRunning = true;
					} catch (e) {
						toasts.addToast('Failed to start discovery', ToastType.Error);
						console.error(e);
					}
				}
			} else {
				session.isDragHovering = false;
			}
		}),
	);
});

onBeforeUnmount(() => {
	unlisteners.forEach((fn) => {
		try {
			fn();
		} catch (e) {
			console.error(e);
		}
	});
	unlisteners.length = 0;
});
</script>

<template>
	<div class="w-screen h-screen flex flex-col min-[760px]:flex-row overflow-hidden select-none bg-ground text-ink font-sans">
		<ToastNotification />
		<SettingsDrawer :open="settingsOpen" @close="settingsOpen = false" />
		<StatusHeader @open-settings="settingsOpen = true" />
		<main class="flex-1 min-h-0 min-w-0 overflow-y-auto">
			<div class="flex flex-col gap-4 p-4 max-w-[1100px] mx-auto items-stretch">
				<section v-if="pendingConsent.length > 0" aria-label="Incoming requests" class="flex flex-col gap-3">
					<TransferCard v-for="t in pendingConsent" :key="t.id" :transfer="t" />
				</section>
				<div class="grid grid-cols-1 min-[1000px]:grid-cols-[300px_minmax(0,1fr)] gap-4 items-start rounded-xl border border-seam bg-panel p-3">
					<SendPane :staged-files="stagedFiles" @staged="onStaged" @clear-sending="onClearSending" />
					<DeviceBoard :staged-files="stagedFiles" />
				</div>
				<TransferQueue />
			</div>
		</main>
	</div>
</template>
