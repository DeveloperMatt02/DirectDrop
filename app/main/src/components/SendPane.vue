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
	<section aria-label="Send files" class="flex min-w-0 flex-col gap-2">
		<h2 class="font-mono text-[11px] font-semibold tracking-[0.2em] text-dim">
			SEND FILES
		</h2>
		<button
			type="button"
			class="flex w-full items-center gap-3 rounded-lg border border-dashed border-seam bg-ground px-3 py-3 text-left hover:border-accent"
			:class="{ 'qsd-drop-active': session.isDragHovering }"
			aria-label="Select files to send (or drop files anywhere)" @click="openFilePicker">
			<span class="grid h-9 w-9 shrink-0 place-items-center rounded-full bg-tx-bg text-tx" aria-hidden="true">
				<svg
					width="16" height="16" viewBox="0 0 16 16"
					fill="none" stroke="currentColor" stroke-width="1.8">
					<path d="M8 13.5v-10M3.5 7.5 8 3l4.5 4.5" stroke-linecap="round" stroke-linejoin="round" />
				</svg>
			</span>
			<span class="flex min-w-0 flex-1 flex-col gap-0.5">
				<span v-if="!hasStaged" class="font-mono text-[12px] font-semibold text-ink">DROP FILES OR SELECT</span>
				<span v-else class="truncate font-mono text-[12px] font-semibold tabular-nums text-ink">
					{{ stagedFiles.length }} STAGED · {{ baseName(stagedFiles[0]) }}{{ stagedFiles.length > 1 ? ` +${stagedFiles.length - 1}` : '' }}
				</span>
				<span class="font-mono text-[10px] text-dim">REGULAR FILES ONLY — FOLDERS REJECTED</span>
			</span>
			<span class="shrink-0 rounded-md border border-seam px-3 py-1.5 font-mono text-[11px] font-semibold text-ink" aria-hidden="true">SELECT</span>
		</button>
		<button
			v-if="hasStaged" type="button"
			class="qsd-action self-end"
			@click="emit('clearSending')">
			CLEAR STAGED
		</button>
	</section>
</template>
