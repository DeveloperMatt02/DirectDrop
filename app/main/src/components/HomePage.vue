<script setup lang="ts">
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';
import { storeToRefs } from 'pinia';
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import ContentStatus from '../composables/ContentStatus.vue';
import Heading from '../composables/Heading.vue';
import SettingsModal from '../composables/SettingsModal.vue';
import SideMenu from '../composables/SideMenu.vue';
import { ToastNotification } from '../vue_lib';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import { sendToEndpoint } from '../vue_lib/utils';
import DeviceRow from './DeviceRow.vue';
import TransferCard from './TransferCard.vue';
import { useDevicesStore } from '../stores/devices';
import { useSessionStore } from '../stores/session';
import { useTransfersStore } from '../stores/transfers';
import {
	QSD_BLE_PRESENCE_EVENT,
	QSD_ENDPOINTS_EVENT,
	QSD_TRANSFER_EVENT,
	QSD_VISIBILITY_EVENT,
	validateSendFiles,
	type EndpointDto,
	type TransferDto,
	type VisibilityDto,
} from '../types/qsd';

const session = useSessionStore();
const devices = useDevicesStore();
const transfers = useTransfersStore();
const toasts = useToastStore();

const { deviceName, discoveryRunning } = storeToRefs(session);

const settingsOpen = ref(false);
const stagedFiles = ref<string[]>([]);
const unlisteners: UnlistenFn[] = [];

const inbound = computed(() => transfers.inboundSorted);
const outbound = computed(() => transfers.outbound);
const endpoints = computed(() => devices.sorted);

function normalizeDrop(paths: string[]): string[] {
	return paths;
}

function onStaged(files: string[]) {
	stagedFiles.value = files;
}

async function onClearSending() {
	stagedFiles.value = [];
}

async function selectEndpoint(id: string) {
	if (stagedFiles.value.length === 0) {
		toasts.addToast('Select files first, then pick a device', ToastType.Info);
		return;
	}
	const v = validateSendFiles(stagedFiles.value);
	if (!v.ok) {
		toasts.addToast(v.error ?? 'Invalid selection', ToastType.Error);
		return;
	}
	try {
		await sendToEndpoint(id, stagedFiles.value);
		toasts.addToast('Sending…', ToastType.Info);
	} catch (e) {
		console.error(e);
	}
}

onMounted(async () => {
	await session.init();
	if (session.initError) {
		toasts.addToast(`Failed to load settings: ${session.initError}`, ToastType.Error);
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
			toasts.addToast('A phone nearby is sharing — you are Invisible', ToastType.Info);
		}),
	);
	unlisteners.push(
		await getCurrentWindow().onDragDropEvent(async (event) => {
			if (event.payload.type === 'over') {
				session.isDragHovering = true;
			} else if (event.payload.type === 'drop') {
				session.isDragHovering = false;
				const files = normalizeDrop(event.payload.paths);
				const v = validateSendFiles(files);
				if (!v.ok) {
					toasts.addToast(v.error ?? 'Invalid selection', ToastType.Error);
					return;
				}
				stagedFiles.value = files;
				if (!discoveryRunning.value) {
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
	<div class="flex flex-col w-full h-full bg-green-50 max-w-full max-h-full overflow-hidden">
		<ToastNotification />
		<SettingsModal :open="settingsOpen" @close="settingsOpen = false" />

		<Heading @open-settings="settingsOpen = true" />

		<div class="flex-1 flex flex-row">
			<SideMenu :staged-files="stagedFiles" @clear-sending="onClearSending" />

			<div class="flex-1 flex flex-col bg-white w-full max-w-full min-w-0 min-h-full rounded-tl-[3rem] p-12 h-1 overflow-y-auto">
				<ContentStatus v-model:staged-files="stagedFiles" @staged="onStaged" />

				<!-- F2: nearby devices (click to send staged files) -->
				<div v-if="endpoints.length > 0" class="mt-4" aria-label="Nearby devices">
					<DeviceRow
						v-for="ep in endpoints" :key="ep.id" :endpoint="ep"
						:selected="false" @select="selectEndpoint" />
				</div>

				<!-- F1: inbound transfers (PIN + countdown + file/text/URL actions) -->
				<div v-if="inbound.length > 0" class="mt-2" aria-label="Incoming transfers">
					<TransferCard v-for="t in inbound" :key="t.id" :transfer="t" />
				</div>

				<!-- F2: outbound transfers -->
				<div v-if="outbound.length > 0" class="mt-2" aria-label="Outgoing transfers">
					<TransferCard v-for="t in outbound" :key="t.id" :transfer="t" />
				</div>

				<p v-if="deviceName === ''" class="sr-only">
					Loading device…
				</p>
			</div>
		</div>
	</div>
</template>
