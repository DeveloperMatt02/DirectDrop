<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { storeToRefs } from 'pinia';
import { useDevicesStore } from '../stores/devices';
import { useSessionStore } from '../stores/session';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import { sendToBle, sendToEndpoint } from '../vue_lib/utils';
import { bleReceiverLabel, isTerminalState } from '../types/qsd';
import { useTransfersStore } from '../stores/transfers';
import DeviceRow from './DeviceRow.vue';

const props = defineProps({
	stagedFiles: { type: Array as () => string[], default: () => [] },
});

const devices = useDevicesStore();
const session = useSessionStore();
const transfers = useTransfersStore();
const toasts = useToastStore();
const { blePresenceAt } = storeToRefs(session);
const { bleSorted, scanning, lastScanAt } = storeToRefs(devices);

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

const lastScanLabel = computed(() => {
	if (lastScanAt.value === null) return null;
	const ageS = Math.floor((now.value - lastScanAt.value) / 1000);
	if (ageS < 5) return 'JUST NOW';
	return `${ageS}s AGO`;
});

// Send BLE in corso (send_id): copre anche la finestra di connessione, quando
// lo store non ha ancora nessuna card. Si azzera al terminale (il backend ora
// emette sempre un terminale, anche sui fallimenti pre-handshake).
const pendingBle = ref<string | null>(null);
watch(
	() => transfers.items,
	(items) => {
		if (pendingBle.value) {
			const t = items.get(pendingBle.value);
			if (t && isTerminalState(t.state)) pendingBle.value = null;
		}
	},
	{ deep: true },
);

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

async function scanBle() {
	try {
		const res = await devices.scanBle();
		if (res.length === 0) {
			toasts.addToast('No phones found — open Receive on the phone and retry', ToastType.Info);
		}
	} catch (e) {
		toasts.addToast('Bluetooth scan failed — is Bluetooth on?', ToastType.Error);
		console.error(e);
	}
}

async function selectBle(receiverId: string) {
	if (!hasStaged.value) {
		toasts.addToast('Select files first, then pick a device', ToastType.Info);
		return;
	}
	// Un solo send BLE alla volta: la connessione occupa l'adapter e i
	// tentativi paralleli falliscono tutti (L2CAP timeout a catena).
	const busy = pendingBle.value !== null
		|| [...transfers.items.values()].some(
			(t) => t.kind === 'Outbound' && !isTerminalState(t.state),
		);
	if (busy) {
		toasts.addToast('A send is already in progress — wait or cancel it', ToastType.Info);
		return;
	}
	try {
		pendingBle.value = await sendToBle(receiverId, props.stagedFiles);
		toasts.addToast('Sending over Bluetooth… accept on the phone', ToastType.Info);
	} catch (e) {
		pendingBle.value = null;
		console.error(e);
	}
}
</script>

<template>
	<section aria-label="Nearby devices" class="border border-slate-800 bg-slate-900 p-4 flex flex-col gap-3 min-h-0">
		<div class="flex items-baseline justify-between">
			<h2 class="font-mono text-[11px] tracking-[0.2em] text-slate-400">
				NEARBY DEVICES · {{ endpoints.length }}
			</h2>
			<span v-if="bleHint !== null" class="font-mono text-[11px] text-emerald-400 tabular-nums">BLE HINT · {{ bleHint }}s AGO</span>
			<span v-else class="font-mono text-[11px] text-slate-600">BLE · NO HINT</span>
		</div>
		<p v-if="!hasStaged" class="font-mono text-[11px] text-slate-500">
			STAGE FILES FIRST, THEN PRESS SEND.
		</p>
		<div v-if="endpoints.length > 0" class="flex flex-col gap-2" role="list">
			<DeviceRow
				v-for="ep in endpoints" :key="ep.id" :endpoint="ep" role="listitem"
				:disabled="!hasStaged" @select="selectEndpoint" />
		</div>
		<p v-else class="border border-dashed border-slate-700 p-3 font-mono text-[11px] text-slate-500">
			NO PEERS. KEEP THE PHONE RECEIVE SCREEN OPEN, BOTH UNLOCKED.
		</p>

		<div class="flex items-baseline justify-between pt-2 border-t border-slate-800">
			<h2 class="font-mono text-[11px] tracking-[0.2em] text-slate-400">
				NEARBY OVER BLUETOOTH · {{ bleSorted.length }}
			</h2>
			<span v-if="lastScanLabel !== null" class="font-mono text-[11px] text-slate-500 tabular-nums">{{ lastScanLabel }}</span>
		</div>
		<div class="flex items-center gap-2">
			<button
				type="button" :disabled="scanning"
				class="font-mono text-[12px] font-bold bg-sky-950 border border-sky-600 text-sky-300 px-4 py-1.5 hover:bg-sky-900 disabled:opacity-40"
				aria-label="Scan for Bluetooth receivers" @click="scanBle">
				{{ scanning ? 'SCANNING…' : 'SCAN' }}
			</button>
			<p class="font-mono text-[10px] text-slate-600">
				NO SHARED NETWORK NEEDED
			</p>
		</div>
		<div v-if="bleSorted.length > 0" class="flex flex-col gap-2" role="list" aria-label="Bluetooth receivers">
			<div
				v-for="r in bleSorted" :key="`${r.endpoint_id}-${r.address}`"
				class="border border-slate-700 bg-slate-950 p-3 flex items-center gap-3 min-w-0" role="listitem">
				<svg
					width="26" height="34" viewBox="0 0 26 38" fill="none"
					stroke="#94a3b8" stroke-width="1.2" class="shrink-0" aria-hidden="true">
					<rect
						x="3" y="2" width="20" height="34"
						rx="2" />
					<circle
						cx="13" cy="31" r="1.2" fill="#94a3b8"
						stroke="none" />
				</svg>
				<span class="flex-1 min-w-0">
					<span class="flex items-center gap-2">
						<span class="font-mono text-[13px] font-bold text-slate-100 truncate" :title="bleReceiverLabel(r)">{{ bleReceiverLabel(r) }}</span>
						<span class="font-mono text-[10px] border border-sky-700 text-sky-400 px-1.5 py-px shrink-0">BLE</span>
					</span>
					<span class="font-mono text-[11px] text-slate-500 tabular-nums truncate block" :title="r.address">{{ r.address }}</span>
				</span>
				<button
					type="button" :disabled="!hasStaged || scanning || pendingBle !== null"
					class="font-mono text-[12px] font-bold bg-sky-950 border border-sky-600 text-sky-300 px-4 py-1.5 shrink-0 hover:bg-sky-900 disabled:opacity-40"
					:aria-label="`Send staged files to ${bleReceiverLabel(r)} over Bluetooth`"
					@click="selectBle(`${r.endpoint_id}-${r.address}`)">
					SEND
				</button>
			</div>
		</div>
		<p v-else-if="lastScanAt === null" class="border border-dashed border-slate-700 p-3 font-mono text-[11px] text-slate-500">
			NOT SCANNED YET — PRESS SCAN WITH THE PHONE ON ITS RECEIVE SCREEN.
		</p>
		<p v-else class="border border-dashed border-slate-700 p-3 font-mono text-[11px] text-slate-500">
			NONE FOUND — IS THE PHONE ON ITS RECEIVE SCREEN?
		</p>

		<div class="mt-auto pt-2 border-t border-slate-800">
			<svg
				viewBox="0 0 200 100" class="w-full h-[86px]" fill="none" aria-label="Scan radar"
				role="img">
				<circle
					cx="100" cy="100" r="38" stroke="#334155"
					stroke-width="1" class="radar-ping" />
				<circle
					cx="100" cy="100" r="64" stroke="#334155"
					stroke-width="1" opacity="0.7" />
				<circle
					cx="100" cy="100" r="90" stroke="#334155"
					stroke-width="1" opacity="0.4" />
				<g class="radar-sweep">
					<path d="M100 100 L100 12 A88 88 0 0 1 162 38 Z" fill="#38bdf8" opacity="0.18" stroke="none" />
					<line
						x1="100" y1="100" x2="100" y2="12"
						stroke="#38bdf8" stroke-width="1.5" />
				</g>
				<circle
					cx="100" cy="100" r="2.5" fill="#38bdf8"
					stroke="none" />
				<circle
					cx="132" cy="72" r="2" fill="#34d399"
					stroke="none" class="radar-ping" />
				<circle
					cx="70" cy="52" r="2" fill="#34d399"
					stroke="none" class="radar-ping" />
			</svg>
			<p class="font-mono text-[10px] text-slate-600 text-center tracking-[0.25em]">
				SCAN · LAN
			</p>
		</div>
	</section>
</template>
