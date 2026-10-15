<script setup lang="ts">
import { open as tauriDialog } from '@tauri-apps/plugin-dialog';
import { isEnabled as autostartEnabled } from '@tauri-apps/plugin-autostart';
import { getStore } from '@tauri-apps/plugin-store';
import { storeToRefs } from 'pinia';
import { computed, onMounted, ref, watch } from 'vue';
import { useSessionStore } from '../stores/session';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import { setAutoStart } from '../vue_lib/utils';

const props = defineProps({
	open: { type: Boolean, default: false },
});
const emit = defineEmits(['close']);

const session = useSessionStore();
const { deviceName, downloadPath, port } = storeToRefs(session);
const toasts = useToastStore();

const autostart = ref(true);
const realclose = ref(false);
/**
 * "KEEP RUNNING ON CLOSE" is the inverse of the stored `realclose` flag
 * (main.rs handle_window_event: realclose true = closing quits, false =
 * the window hides and the app stays in the tray). The key is unchanged.
 */
const keepRunning = computed(() => !realclose.value);
const startminimized = ref(false);
const draftName = ref('');

async function loadPrefs() {
	try {
		const store = (await getStore('.settings.json'))!;
		autostart.value = ((await store.get<boolean>('autostart')) ?? true) as boolean;
		realclose.value = ((await store.get<boolean>('realclose')) ?? false) as boolean;
		startminimized.value = ((await store.get<boolean>('startminimized')) ?? false) as boolean;
	} catch (e) {
		console.error(e);
	}
	// Show what the OS actually does: the stored flag defaults to true but
	// nothing enables the autostart entry until the toggle is used.
	try {
		autostart.value = await autostartEnabled();
	} catch (e) {
		console.error(e);
	}
	draftName.value = deviceName.value;
}

onMounted(loadPrefs);
watch(() => props.open, (v) => {
	if (v) loadPrefs();
});

function close() {
	emit('close');
}

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

async function toggleKeepRunning() {
	const next = !realclose.value;
	try {
		await persist('realclose', next);
		realclose.value = next;
	} catch (e) {
		toasts.addToast('Failed to save setting', ToastType.Error);
		console.error(e);
	}
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

function onKeydown(e: KeyboardEvent) {
	if (e.key === 'Escape') close();
}

defineExpose({ loadPrefs, keepRunning, realclose, toggleKeepRunning });
</script>

<template>
	<div
		v-if="open" class="fixed inset-0 z-40 font-mono" role="dialog"
		aria-modal="true" aria-label="Settings" @keydown="onKeydown">
		<div class="absolute inset-0 bg-overlay" @click="close" />
		<aside class="absolute top-0 right-0 h-full w-[360px] max-w-[92vw] flex flex-col border-l border-seam bg-panel text-ink">
			<div class="flex items-center justify-between px-4 h-12 border-b border-seam shrink-0">
				<h2 class="text-[13px] font-bold tracking-[0.2em] text-ink">
					SETTINGS
				</h2>
				<button type="button" class="qsd-action" @click="close" autofocus>
					CLOSE
				</button>
			</div>
			<div class="flex-1 overflow-y-auto p-4 flex flex-col gap-4 text-[12px]">
				<div>
					<label class="text-[11px] tracking-[0.15em] text-faint" for="qsd-device-name">DEVICE NAME</label>
					<input
						id="qsd-device-name" v-model="draftName" type="text" maxlength="255"
						class="mt-1 w-full bg-ground border border-seam px-2 py-1.5 text-ink" @change="saveDeviceName">
					<p class="text-faint mt-1">
						APPLIES TO NEW ANNOUNCEMENTS. NEVER RESTARTS MID-TRANSFER.
					</p>
				</div>
				<div>
					<p class="text-[11px] tracking-[0.15em] text-faint">
						DOWNLOAD DIR
					</p>
					<button type="button" class="mt-1 w-full text-left bg-ground border border-seam px-2 py-1.5 hover:border-accent" @click="openDownloadPicker">
						<span class="block truncate tabular-nums text-ink">{{ downloadPath ?? 'SYSTEM DOWNLOAD DIR' }}</span>
					</button>
				</div>
				<div class="flex flex-col">
					<button type="button" class="flex items-center justify-between w-full min-h-[40px] py-2 border-b border-seam" @click="toggleAutostart" :aria-pressed="autostart">
						<span class="text-ink">START ON BOOT</span>
						<span class="tabular-nums" :class="autostart ? 'text-rx' : 'text-faint'">{{ autostart ? 'ON' : 'OFF' }}</span>
					</button>
					<button
						type="button" class="flex items-center justify-between w-full min-h-[40px] py-2"
						:aria-pressed="keepRunning" aria-describedby="qsd-keep-running-hint" @click="toggleKeepRunning">
						<span class="text-ink">KEEP RUNNING ON CLOSE</span>
						<span class="tabular-nums" :class="keepRunning ? 'text-rx' : 'text-faint'">{{ keepRunning ? 'ON' : 'OFF' }}</span>
					</button>
					<p id="qsd-keep-running-hint" class="py-1.5 border-b border-seam text-[11px] text-faint">
						ON: CLOSING THE WINDOW HIDES IT AND DIRECTDROP KEEPS RUNNING. OFF: CLOSING THE WINDOW QUITS.
					</p>
					<button type="button" class="flex items-center justify-between w-full min-h-[40px] py-2 border-b border-seam" @click="toggleStartMinimized" :aria-pressed="startminimized">
						<span class="text-ink">START MINIMIZED</span>
						<span class="tabular-nums" :class="startminimized ? 'text-rx' : 'text-faint'">{{ startminimized ? 'ON' : 'OFF' }}</span>
					</button>
				</div>
				<p v-if="port !== null" class="tabular-nums text-faint">
					PORT {{ port }} · READ-ONLY
				</p>
			</div>
		</aside>
	</div>
</template>
