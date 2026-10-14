<script setup lang="ts">
import { open as tauriDialog } from '@tauri-apps/plugin-dialog';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useDevicesStore } from '../stores/devices';
import { useSessionStore } from '../stores/session';
import { baseName } from '../types/qsd';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import { assertSendableFiles } from '../vue_lib/utils';

const props = defineProps({
	stagedFiles: { type: Array as () => string[], default: () => [] },
});

const emit = defineEmits(['staged', 'clearSending']);

const session = useSessionStore();
const devices = useDevicesStore();
const toasts = useToastStore();
const { discoveryRunning } = storeToRefs(session);

const hasStaged = computed(() => props.stagedFiles.length > 0);

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
	emit('staged', files);
	await ensureDiscovery();
}

async function openFilePicker() {
	const el = await tauriDialog({ title: 'Select files to send', directory: false, multiple: true });
	await stageFiles(normalizeDialogResult(el));
}

defineExpose({ stageFiles });
</script>

<template>
	<section aria-label="Send files" class="border border-slate-800 bg-slate-900 p-4 flex flex-col gap-3">
		<h2 class="font-mono text-[11px] tracking-[0.2em] text-slate-400">SEND FILES</h2>
		<button
			type="button"
			class="w-full border border-dashed border-slate-600 p-5 text-left hover:border-sky-400"
			:class="{ 'border-sky-400 bg-slate-800/50': session.isDragHovering }"
			@click="openFilePicker" aria-label="Select files to send (or drop files anywhere)">
			<span class="flex flex-col items-center gap-2 text-center">
				<svg width="44" height="32" viewBox="0 0 44 32" fill="none" stroke="#38bdf8" stroke-width="1.5" aria-hidden="true">
					<path d="M22 28v-20M15 13l7-7 7 7" />
					<path d="M12 12a14 14 0 0 1 20 0" />
					<path d="M6 18a22 22 0 0 1 32 0" opacity="0.5" />
					<circle cx="22" cy="29" r="1.5" fill="#38bdf8" stroke="none" />
				</svg>
				<span v-if="!hasStaged" class="font-mono text-[13px] text-slate-200">DROP FILES OR SELECT</span>
				<span v-else class="font-mono text-[13px] text-slate-200 tabular-nums">
					{{ stagedFiles.length }} STAGED · {{ baseName(stagedFiles[0]) }}{{ stagedFiles.length > 1 ? ` +${stagedFiles.length - 1}` : '' }}
				</span>
				<span class="font-mono text-[11px] text-amber-300">! REGULAR FILES ONLY — FOLDERS REJECTED</span>
				<span class="font-mono text-[12px] border border-slate-500 px-4 py-1.5 text-slate-200" aria-hidden="true">SELECT</span>
			</span>
		</button>
		<button
			v-if="hasStaged" type="button"
			class="font-mono text-[12px] border border-slate-700 px-3 py-1.5 text-slate-400 self-end hover:border-slate-400 hover:text-slate-200"
			@click="emit('clearSending')">
			CLEAR STAGED
		</button>
	</section>
</template>
