<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import { storeToRefs } from 'pinia';
import { useDevicesStore } from '../stores/devices';
import { useSessionStore } from '../stores/session';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import { sendToEndpoint } from '../vue_lib/utils';
import DeviceRow from './DeviceRow.vue';

const props = defineProps({
	stagedFiles: { type: Array as () => string[], default: () => [] },
});

const devices = useDevicesStore();
const session = useSessionStore();
const toasts = useToastStore();
const { blePresenceAt } = storeToRefs(session);

const endpoints = computed(() => devices.sorted);
const hasStaged = computed(() => props.stagedFiles.length > 0);

const now = ref(Date.now());
let timer: ReturnType<typeof setInterval> | null = null;
onMounted(() => {
	timer = setInterval(() => {
		now.value = Date.now();
	}, 5000);
});
onBeforeUnmount(() => {
	if (timer) clearInterval(timer);
});

const bleHint = computed(() => {
	if (blePresenceAt.value === null) return null;
	const ageS = Math.floor((now.value - blePresenceAt.value) / 1000);
	if (ageS > 120) return null;
	return ageS;
});

async function selectEndpoint(id: string) {
	if (!hasStaged.value) {
		toasts.addToast('Select files first, then pick a device', ToastType.Info);
		return;
	}
	try {
		await sendToEndpoint(id, props.stagedFiles);
		toasts.addToast('Sending…', ToastType.Info);
	} catch (e) {
		console.error(e);
	}
}
</script>

<template>
	<section aria-label="Nearby devices" class="border border-slate-800 bg-slate-900 p-4 flex flex-col gap-3 min-h-0">
		<div class="flex items-baseline justify-between">
			<h2 class="font-mono text-[11px] tracking-[0.2em] text-slate-400">NEARBY DEVICES · {{ endpoints.length }}</h2>
			<span v-if="bleHint !== null" class="font-mono text-[11px] text-emerald-400 tabular-nums">BLE HINT · {{ bleHint }}s AGO</span>
			<span v-else class="font-mono text-[11px] text-slate-600">BLE · NO HINT</span>
		</div>
		<p v-if="!hasStaged" class="font-mono text-[11px] text-slate-500">STAGE FILES FIRST, THEN PRESS SEND.</p>
		<div v-if="endpoints.length > 0" class="flex flex-col gap-2" role="list">
			<DeviceRow
				v-for="ep in endpoints" :key="ep.id" :endpoint="ep" role="listitem"
				:disabled="!hasStaged" @select="selectEndpoint" />
		</div>
		<p v-else class="border border-dashed border-slate-700 p-3 font-mono text-[11px] text-slate-500">
			NO PEERS. KEEP THE PHONE RECEIVE SCREEN OPEN, BOTH UNLOCKED.
		</p>
		<div class="mt-auto pt-2 border-t border-slate-800">
			<svg viewBox="0 0 200 100" class="w-full h-[86px]" fill="none" aria-label="Scan radar" role="img">
				<circle cx="100" cy="100" r="38" stroke="#334155" stroke-width="1" class="radar-ping" />
				<circle cx="100" cy="100" r="64" stroke="#334155" stroke-width="1" opacity="0.7" />
				<circle cx="100" cy="100" r="90" stroke="#334155" stroke-width="1" opacity="0.4" />
				<g class="radar-sweep">
					<path d="M100 100 L100 12 A88 88 0 0 1 162 38 Z" fill="#38bdf8" opacity="0.18" stroke="none" />
					<line x1="100" y1="100" x2="100" y2="12" stroke="#38bdf8" stroke-width="1.5" />
				</g>
				<circle cx="100" cy="100" r="2.5" fill="#38bdf8" stroke="none" />
				<circle cx="132" cy="72" r="2" fill="#34d399" stroke="none" class="radar-ping" />
				<circle cx="70" cy="52" r="2" fill="#34d399" stroke="none" class="radar-ping" />
			</svg>
			<p class="font-mono text-[10px] text-slate-600 text-center tracking-[0.25em]">SCAN · LAN</p>
		</div>
	</section>
</template>
