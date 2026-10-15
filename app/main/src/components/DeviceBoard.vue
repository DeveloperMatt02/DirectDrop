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
	<section aria-label="Nearby devices" class="flex min-h-0 min-w-0 flex-col gap-2">
		<div class="flex items-baseline justify-between gap-2">
			<h2 class="font-mono text-[11px] font-semibold tracking-[0.2em] text-dim">
				NEARBY DEVICES · {{ endpoints.length }}
			</h2>
			<span v-if="bleHint !== null" class="font-mono text-[11px] tabular-nums text-rx">BLE HINT · {{ bleHint }}s AGO</span>
			<span v-else class="font-mono text-[11px] text-faint">BLE · NO HINT</span>
		</div>
		<p v-if="!hasStaged" class="font-mono text-[11px] text-faint">
			STAGE FILES FIRST, THEN PRESS SEND.
		</p>
		<div v-if="endpoints.length > 0" class="flex flex-col gap-1.5" role="list">
			<DeviceRow
				v-for="ep in endpoints" :key="ep.id" :endpoint="ep" role="listitem"
				:disabled="!hasStaged" @select="selectEndpoint" />
		</div>
		<p v-else class="rounded-lg border border-dashed border-seam px-3 py-2 font-mono text-[11px] text-faint">
			NO PEERS. KEEP THE PHONE RECEIVE SCREEN OPEN, BOTH UNLOCKED.
		</p>

		<div class="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 border-t border-seam pt-2">
			<h2 class="font-mono text-[11px] font-semibold tracking-[0.2em] text-dim">
				NEARBY OVER BLUETOOTH · {{ bleSorted.length }}
			</h2>
			<span class="font-mono text-[10px] text-faint">NO SHARED NETWORK NEEDED</span>
			<span class="flex-1" />
			<span v-if="lastScanLabel !== null" class="font-mono text-[11px] tabular-nums text-faint">{{ lastScanLabel }}</span>
			<button
				type="button" :disabled="scanning"
				class="qsd-action"
				aria-label="Scan for Bluetooth receivers" @click="scanBle">
				{{ scanning ? 'SCANNING…' : 'SCAN' }}
			</button>
		</div>
		<div v-if="bleSorted.length > 0" class="flex flex-col gap-1.5" role="list" aria-label="Bluetooth receivers">
			<div
				v-for="r in bleSorted" :key="`${r.endpoint_id}-${r.address}`"
				class="flex min-w-0 items-center gap-3 rounded-lg border border-seam bg-ground px-3 py-2" role="listitem">
				<svg
					width="18" height="26" viewBox="0 0 26 38"
					fill="none" class="shrink-0 stroke-faint" stroke-width="1.6" aria-hidden="true">
					<rect
						x="3" y="2" width="20"
						height="34" rx="3" />
					<path d="M10 31h6" stroke-linecap="round" />
				</svg>
				<span class="min-w-0 flex-1">
					<span class="flex items-center gap-2">
						<span class="truncate text-[13px] font-semibold text-ink" :title="bleReceiverLabel(r)">{{ bleReceiverLabel(r) }}</span>
						<span class="shrink-0 rounded-full border border-seam px-1.5 font-mono text-[10px] text-dim">BLE</span>
					</span>
					<span class="block truncate font-mono text-[11px] tabular-nums text-faint" :title="r.address">{{ r.address }}</span>
				</span>
				<button
					type="button" :disabled="!hasStaged || scanning || pendingBle !== null"
					class="qsd-action qsd-action-primary shrink-0"
					:aria-label="`Send staged files to ${bleReceiverLabel(r)} over Bluetooth`"
					@click="selectBle(`${r.endpoint_id}-${r.address}`)">
					SEND
				</button>
			</div>
		</div>
		<template v-else-if="hasStaged">
			<p v-if="lastScanAt === null" class="rounded-lg border border-dashed border-seam px-3 py-2 font-mono text-[11px] text-faint">
				NOT SCANNED YET — PRESS SCAN WITH THE PHONE ON ITS RECEIVE SCREEN.
			</p>
			<p v-else class="rounded-lg border border-dashed border-seam px-3 py-2 font-mono text-[11px] text-faint">
				NONE FOUND — IS THE PHONE ON ITS RECEIVE SCREEN?
			</p>
		</template>
	</section>
</template>
