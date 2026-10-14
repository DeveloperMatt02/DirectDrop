<script setup lang="ts">
import { open as tauriDialog } from '@tauri-apps/plugin-dialog';
import { getStore } from '@tauri-apps/plugin-store';
import { storeToRefs } from 'pinia';
import { onMounted, ref } from 'vue';
import { useSessionStore } from '../stores/session';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import { setAutoStart } from '../vue_lib/utils';

defineProps({
	open: { type: Boolean, default: false },
});
const emit = defineEmits(['close']);

const session = useSessionStore();
const { deviceName, downloadPath, port } = storeToRefs(session);
const toasts = useToastStore();

const autostart = ref(true);
const realclose = ref(false);
const startminimized = ref(false);
const draftName = ref('');

onMounted(async () => {
	try {
		const store = (await getStore('.settings.json'))!;
		autostart.value = ((await store.get<boolean>('autostart')) ?? true) as boolean;
		realclose.value = ((await store.get<boolean>('realclose')) ?? false) as boolean;
		startminimized.value = ((await store.get<boolean>('startminimized')) ?? false) as boolean;
	} catch (e) {
		console.error(e);
	}
	draftName.value = deviceName.value;
});

async function persist(key: string, value: unknown) {
	const store = (await getStore('.settings.json'))!;
	await store.set(key, value);
	await store.save();
}

async function toggleAutostart() {
	const next = !autostart.value;
	try {
		await setAutoStart(next);
		autostart.value = next;
	} catch (e) {
		toasts.addToast('Failed to update autostart', ToastType.Error);
		console.error(e);
	}
}

async function toggleRealclose() {
	const next = !realclose.value;
	await persist('realclose', next);
	realclose.value = next;
}

async function toggleStartMinimized() {
	const next = !startminimized.value;
	await persist('startminimized', next);
	startminimized.value = next;
}

async function openDownloadPicker() {
	const el = await tauriDialog({ title: 'Select the destination for files', directory: true, multiple: false });
	if (el === null) return;
	const path = Array.isArray(el) ? (el[0] as string) : (el as string);
	try {
		await session.setDownloadPath(path);
	} catch (e) {
		toasts.addToast('Failed to set download folder', ToastType.Error);
		console.error(e);
	}
}

async function saveDeviceName() {
	try {
		await session.setDeviceName(draftName.value);
		toasts.addToast('Device name saved — applies to new announcements', ToastType.Info);
	} catch (e) {
		toasts.addToast(e instanceof Error ? e.message : 'Invalid device name', ToastType.Error);
	}
}
</script>

<template>
	<div
		v-if="open" class="absolute z-10 w-full h-full flex justify-center items-center bg-black bg-opacity-25" role="dialog" aria-modal="true"
		aria-label="Settings">
		<div class="bg-white rounded-xl shadow-xl p-4 w-[24rem]">
			<div class="flex flex-row justify-between items-center">
				<h3 class="font-medium text-xl">
					Settings
				</h3>
				<button
					type="button"
					class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out" @click="emit('close')">
					Close
				</button>
			</div>
			<div class="py-4 flex flex-col">
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center">
						<span class="label-text">Device name</span>
						<input
							v-model="draftName" type="text" maxlength="255" class="input input-bordered w-40 text-sm"
							@change="saveDeviceName">
					</label>
					<p class="text-xs opacity-70 mt-1 px-1">
						Applies to new announcements (restart of service). Never auto-restarts mid-transfer.
					</p>
				</div>
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<button type="button" class="cursor-pointer flex flex-row justify-between items-center w-full" @click="toggleAutostart">
						<span class="label-text">Start on boot</span>
						<input
							type="checkbox" :checked="autostart" class="checkbox focus:outline-none" tabindex="-1"
							readonly>
					</button>
				</div>
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<button type="button" class="cursor-pointer flex flex-row justify-between items-center w-full" @click="toggleRealclose">
						<span class="label-text">Keep running on close</span>
						<input
							type="checkbox" :checked="!realclose" class="checkbox focus:outline-none" tabindex="-1"
							readonly>
					</button>
				</div>
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<button type="button" class="cursor-pointer flex flex-row justify-between items-center w-full" @click="toggleStartMinimized">
						<span class="label-text">Start minimized</span>
						<input
							type="checkbox" :checked="startminimized" class="checkbox focus:outline-none" tabindex="-1"
							readonly>
					</button>
				</div>
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<button type="button" class="cursor-pointer flex flex-col items-start w-full" @click="openDownloadPicker()">
						<span class="">Change download folder</span>
						<span class="overflow-hidden whitespace-nowrap text-ellipsis text-xs max-w-80">
							&gt; {{ downloadPath ?? 'OS User\'s download folder' }}
						</span>
					</button>
				</div>
				<p v-if="port !== null" class="text-xs opacity-70 px-3 pt-1">
					Port: {{ port }} (read-only)
				</p>
			</div>
		</div>
	</div>
</template>
