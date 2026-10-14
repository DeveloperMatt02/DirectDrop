<script setup lang="ts">
import { open as tauriDialog } from '@tauri-apps/plugin-dialog';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useDevicesStore } from '../stores/devices';
import { useSessionStore } from '../stores/session';
import { useTransfersStore } from '../stores/transfers';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import { assertSendableFiles } from '../vue_lib/utils';

const emit = defineEmits(['staged']);

const session = useSessionStore();
const devices = useDevicesStore();
const transfers = useTransfersStore();
const toasts = useToastStore();

const { discoveryRunning } = storeToRefs(session);

const hasItems = computed(() => transfers.items.size > 0 || devices.endpoints.size > 0);
const hasStaged = computed(() => stagedFiles.value.length > 0);

// Staged outbound files live in the parent (SendView/HomePage) — mirrored here via prop below.
const stagedFiles = defineModel<string[]>('stagedFiles', { default: [] });

function normalizeDialogResult(el: unknown): string[] {
	if (el === null || el === undefined) return [];
	if (Array.isArray(el)) {
		if (el.length > 0 && typeof el[0] === 'object' && el[0] !== null && 'path' in (el[0] as object)) {
			return (el as Array<{ path: string }>).map((e) => e.path);
		}
		return el as string[];
	}
	return [el as string];
}

async function ensureDiscovery() {
	if (!discoveryRunning.value) {
		try {
			await devices.startDiscovery();
			session.discoveryRunning = true;
		} catch (e) {
			toasts.addToast('Failed to start discovery', ToastType.Error);
			console.error(e);
		}
	}
}

async function stageFiles(files: string[]) {
	if (files.length === 0) return;
	try {
		assertSendableFiles(files);
	} catch {
		return;
	}
	stagedFiles.value = files;
	emit('staged', files);
	await ensureDiscovery();
}

async function openFilePicker() {
	const el = await tauriDialog({ title: 'Select a file to send', directory: false, multiple: true });
	await stageFiles(normalizeDialogResult(el));
}

// Exposed so HomePage drag&drop can reuse the same validation + discovery path.
defineExpose({ stageFiles });
</script>

<template>
	<h3 class="mb-4 font-medium text-xl">
		<span v-if="!hasItems">Ready to receive{{ hasStaged ? ' / send' : '' }}</span>
		<span v-else>Nearby devices</span>
	</h3>

	<div v-if="!hasItems && devices.endpoints.size === 0" class="m-auto status-indicator status-indicator--success status-indicator--xl" aria-hidden="true">
		<div class="circle circle--animated circle-main" />
		<div class="circle circle--animated circle-secondary" />
		<div class="circle circle--animated circle-tertiary" />
	</div>

	<div
		v-if="!hasItems && !hasStaged" class="w-full border
        rounded-2xl p-6 flex flex-col justify-center items-center transition duration-150 ease-in-out mt-auto"
		:class="{'border-green-200 bg-green-100 scale-105': session.isDragHovering}">
		<svg
			xmlns="http://www.w3.org/2000/svg" height="24"
			viewBox="0 -960 960 960" width="24" class="w-8 h-8" aria-hidden="true">
			<!-- eslint-disable-next-line -->
            <path d="M440-320v-326L336-542l-56-58 200-200 200 200-56 58-104-104v326h-80ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 33-23.5 56.5T720-160H240Z" />
		</svg>
		<h4 class="mt-2 font-medium">
			Drop files to send
		</h4>
		<button
			type="button"
			class="btn mt-2 active:scale-95 transition duration-150 ease-in-out" @click="openFilePicker()">
			<svg
				xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
				aria-hidden="true">
				<path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z" />
			</svg>
			<span class="ml-2">Select</span>
		</button>
	</div>
</template>
