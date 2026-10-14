<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-shell';
import { useSessionStore } from '../stores/session';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import { getLatestVersion, loadAppVersion } from '../vue_lib/utils';

const emit = defineEmits(['openSettings']);

const session = useSessionStore();
const { deviceName } = storeToRefs(session);

const version = ref<string | null>(null);
const newVersion = ref<string | null>(null);

async function init() {
	try {
		version.value = await loadAppVersion();
		newVersion.value = await getLatestVersion(version.value);
	} catch (e) {
		console.error(e);
	}
}
void init();

async function openUrl(url: string) {
	try {
		await open(url);
	} catch (e) {
		useToastStore().addToast('Error opening URL, it may not be a valid URI', ToastType.Error);
		console.error('Error opening URL', e);
	}
}
</script>

<template>
	<div class="flex flex-row justify-between items-center px-6 py-4">
		<div>
			<h4 class="text-md">
				Device name
			</h4>
			<h2 class="text-2xl font-medium">
				{{ deviceName }}
			</h2>
		</div>
		<div class="flex justify-center items-center gap-4">
			<button
				v-if="newVersion" type="button"
				class="btn flex items-center gap-2 text-sm transition duration-150 ease-in-out active:scale-95"
				@click="openUrl('https://github.com/Martichou/rquickshare/releases/latest')">
				<span>Update available</span>
				<span>v{{ version }}</span>
				<span class="text-lg">→</span>
				<span>v{{ newVersion }}</span>
			</button>
			<p v-else class="text-sm">
				v{{ version }}
			</p>
			<button
				type="button" aria-label="Open settings"
				class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out" @click="emit('openSettings')">
				<svg
					xmlns="http://www.w3.org/2000/svg" height="24"
					viewBox="0 -960 960 960" width="24">
					<!-- eslint-disable-next-line -->
						<path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-106q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8-57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Zm-2-140Z"/>
				</svg>
			</button>
		</div>
	</div>
</template>
