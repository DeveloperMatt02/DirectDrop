<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import type { TransferView } from '../types/qsd';
import { HANDSHAKE_STATES, baseName, humanSize } from '../types/qsd';
import { useTransfersStore } from '../stores/transfers';
import ConsentCard from './ConsentCard.vue';
import TransportBadge from './TransportBadge.vue';

const BLOCKS = 24;

const props = defineProps({
	transfer: { type: Object as () => TransferView, required: true },
});

const transfers = useTransfersStore();
const toasts = useToastStore();

const t = computed(() => props.transfer);
const meta = computed(() => props.transfer.metadata);
const deviceName = computed(() => meta.value?.source?.name ?? 'UNKNOWN');
const files = computed(() => meta.value?.files ?? []);
const isConsent = computed(() => t.value.state === 'WaitingForUserConsent' && t.value.kind !== 'Outbound');
/** Outbound waiting-for-phone-accept: handshake transients o WaitingForUserConsent
 *  su un Outbound (il PIN si verifica sul telefono, nessun Accept/Decline qui). */
const isOutboundWaiting = computed(() =>
	t.value.kind === 'Outbound'
	&& (t.value.state === 'WaitingForUserConsent' || HANDSHAKE_STATES.has(t.value.state)),
);
const outboundPeer = computed(() => meta.value?.source?.name ?? deviceName.value);
const outboundPin = computed(() => meta.value?.pin_code ?? null);
const isActive = computed(() => ['SentIntroduction', 'SendingFiles', 'ReceivingFiles'].includes(t.value.state));
const isSending = computed(() => ['SentIntroduction', 'SendingFiles'].includes(t.value.state));
const isFinished = computed(() => t.value.state === 'Finished');
const isTerminalError = computed(() => ['Cancelled', 'Rejected', 'Disconnected'].includes(t.value.state));

const frac = computed(() => {
	const ack = meta.value?.ack_bytes ?? 0;
	const total = meta.value?.total_bytes ?? 0;
	if (!total || total <= 0) return 0;
	return Math.min(1, Math.max(0, ack / total));
});
const filledBlocks = computed(() => Math.round(frac.value * BLOCKS));

interface Sample { at: number; bytes: number }
const samples = ref<Sample[]>([]);
let clock: ReturnType<typeof setInterval> | null = null;

function sample() {
	const b = meta.value?.ack_bytes ?? 0;
	const at = Date.now();
	samples.value.push({ at, bytes: b });
	while (samples.value.length > 2 && at - samples.value[0].at > 8000) samples.value.shift();
}
onMounted(() => {
	sample();
	clock = setInterval(sample, 500);
});
onBeforeUnmount(() => {
	if (clock) clearInterval(clock);
});
watch(() => t.value.id, () => {
	samples.value = [];
	sample();
});

const speedBps = computed(() => {
	const s = samples.value;
	if (s.length < 2) return 0;
	const dt = (s[s.length - 1].at - s[0].at) / 1000;
	if (dt <= 0) return 0;
	return Math.max(0, (s[s.length - 1].bytes - s[0].bytes) / dt);
});
const eta = computed(() => {
	const total = meta.value?.total_bytes ?? 0;
	const ack = meta.value?.ack_bytes ?? 0;
	if (!total || speedBps.value <= 0) return '—';
	const s = Math.ceil((total - ack) / speedBps.value);
	if (s <= 0) return '0:00';
	return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, '0')}`;
});

async function accept() {
	try {
		await transfers.consent(t.value.id, 'ConsentAccept');
	} catch (e) {
		toasts.addToast('Failed to accept transfer', ToastType.Error);
		console.error(e);
	}
}
async function decline() {
	try {
		await transfers.consent(t.value.id, 'ConsentDecline');
	} catch (e) {
		toasts.addToast('Failed to decline transfer', ToastType.Error);
		console.error(e);
	}
}
async function cancel() {
	try {
		await transfers.cancel(t.value.id);
	} catch (e) {
		toasts.addToast('Failed to cancel transfer', ToastType.Error);
		console.error(e);
	}
}
function clear() {
	transfers.clear(t.value.id);
}

async function copyText() {
	const text = meta.value?.text ?? meta.value?.payload_preview ?? '';
	if (!text) return;
	try {
		await writeText(text);
		toasts.addToast('Copied to clipboard', ToastType.Success);
	} catch (e) {
		toasts.addToast('Copy failed', ToastType.Error);
		console.error(e);
	}
}

function errorMessage(e: unknown): string {
	if (typeof e === 'string') return e;
	if (e && typeof e === 'object' && 'message' in e && typeof (e as { message: unknown }).message === 'string') {
		return (e as { message: string }).message;
	}
	if (e instanceof Error) return e.message;
	return 'Could not open target';
}

function isHttpUrl(url: string): boolean {
	return /^https?:\/\//i.test(url);
}

async function openFile() {
	const fileName = files.value[0];
	if (!fileName) return;
	try {
		await invoke('qsd_open_path', { name: fileName, reveal: false });
	} catch (e) {
		toasts.addToast(errorMessage(e), ToastType.Error);
		console.error(e);
	}
}

async function openFolder() {
	const fileName = files.value.length === 1 ? files.value[0] : null;
	try {
		await invoke('qsd_open_path', { name: fileName, reveal: true });
	} catch (e) {
		toasts.addToast(errorMessage(e), ToastType.Error);
		console.error(e);
	}
}

async function openTarget() {
	const url = meta.value?.url ?? null;
	if (url) {
		if (!isHttpUrl(url)) {
			toasts.addToast('Could not open target', ToastType.Error);
			return;
		}
		try {
			await open(url);
		} catch (e) {
			toasts.addToast('Could not open target', ToastType.Error);
			console.error(e);
		}
		return;
	}
	if (files.value.length === 1) {
		await openFile();
	} else if (files.value.length > 1) {
		await openFolder();
	}
}

defineExpose({ openFile, openFolder, openTarget });
</script>

<template>
	<ConsentCard v-if="isConsent" :transfer="transfer" @accept="accept" @decline="decline" />
	<div v-else-if="isOutboundWaiting" class="border border-slate-800 bg-slate-900 p-3 font-mono text-[12px]" aria-live="polite">
		<div class="flex items-baseline justify-between gap-2 min-w-0">
			<p class="text-[13px] font-bold text-slate-200 truncate" :title="outboundPeer">
				{{ outboundPeer }}
			</p>
			<p class="text-slate-500 shrink-0">
				{{ t.state.toUpperCase() }}
			</p>
		</div>
		<p class="mt-1 text-slate-300">
			WAITING FOR ACCEPT ON {{ outboundPeer.toUpperCase() }} — CHECK THE PIN THERE, NO BUTTON HERE<template v-if="outboundPin">
				· PIN {{ outboundPin }}
			</template>
		</p>
		<div class="flex justify-end mt-2">
			<button type="button" class="border border-slate-600 px-3 py-1 text-slate-300 hover:border-slate-300" @click="cancel">
				CANCEL
			</button>
		</div>
	</div>
	<div v-else class="border border-slate-800 bg-slate-900 p-3 font-mono text-[12px]">
		<div class="flex items-baseline justify-between gap-2 min-w-0">
			<p class="text-[13px] font-bold text-slate-200 truncate" :title="deviceName">
				{{ deviceName }}
			</p>
			<p class="text-slate-500 shrink-0">
				{{ t.state.toUpperCase() }}
			</p>
		</div>
		<TransportBadge :transport="t.transport" />

		<div v-if="isActive" class="mt-2">
			<div class="flex justify-between text-slate-400" aria-live="polite">
				<span class="text-sky-300">{{ isSending ? 'TX' : 'RX' }} {{ humanSize(speedBps) }}/s</span>
				<span class="tabular-nums">{{ humanSize(meta?.ack_bytes ?? 0) }} / {{ humanSize(meta?.total_bytes ?? 0) }}</span>
				<span class="tabular-nums">ETA {{ eta }}</span>
			</div>
			<div
				class="grid mt-1" :style="{ gridTemplateColumns: `repeat(${BLOCKS}, minmax(0,1fr))`, gap: '2px' }" role="progressbar" :aria-valuenow="Math.round(frac * 100)"
				aria-valuemin="0" aria-valuemax="100">
				<span
					v-for="i in BLOCKS" :key="i" class="h-3"
					:class="i <= filledBlocks ? 'bg-sky-400' : 'bg-slate-800'" />
			</div>
			<ul class="mt-1 text-slate-400">
				<li v-for="(f, i) in files.slice(0, 3)" :key="`${f}-${i}`" class="truncate" :title="f">
					+ {{ baseName(f) }}
				</li>
			</ul>
			<div class="flex justify-end mt-2">
				<button type="button" class="border border-slate-600 px-3 py-1 text-slate-300 hover:border-slate-300" @click="cancel">
					CANCEL
				</button>
			</div>
		</div>

		<div v-else-if="isFinished" class="mt-1 text-slate-300">
			<p>
				<span v-if="meta?.payload_kind === 'Files'">RX {{ files.length }} FILE{{ files.length === 1 ? '' : 'S' }}</span>
				<span v-else-if="meta?.payload_kind === 'Text'">RX TEXT</span>
				<span v-else-if="meta?.payload_kind === 'Url'">RX LINK</span>
				<span v-else-if="meta?.payload_kind === 'WiFi'">RX WI-FI{{ meta?.wifi_ssid ? ` · ${meta.wifi_ssid}` : '' }}</span>
				<span v-else>RX DONE</span>
				<span v-if="(meta?.total_bytes ?? 0) > 0" class="text-slate-500 tabular-nums"> · {{ humanSize(meta?.total_bytes ?? 0) }}</span>
			</p>
			<div class="flex justify-end gap-2 mt-2">
				<template v-if="meta?.payload_kind === 'Files'">
					<button
						v-if="files.length === 1"
						type="button"
						class="border border-slate-600 px-3 py-1 text-slate-300 hover:border-slate-300"
						@click="openFile">
						OPEN FILE
					</button>
					<button
						v-if="files.length > 0"
						type="button"
						class="border border-slate-600 px-3 py-1 text-slate-300 hover:border-slate-300"
						@click="openFolder">
						OPEN FOLDER
					</button>
				</template>
				<button
					v-else-if="meta?.url"
					type="button"
					class="border border-slate-600 px-3 py-1 text-slate-300 hover:border-slate-300"
					@click="openTarget">
					OPEN
				</button>
				<button
					v-if="(meta?.payload_kind === 'Text' || meta?.payload_kind === 'Url') && (meta?.text || meta?.url || meta?.payload_preview)"
					type="button" class="border border-slate-600 px-3 py-1 text-slate-300 hover:border-slate-300" @click="copyText">
					{{ meta?.payload_kind === 'Url' ? 'COPY LINK' : 'COPY' }}
				</button>
				<button type="button" class="border border-slate-600 px-3 py-1 text-slate-300 hover:border-slate-300" @click="clear">
					CLEAR
				</button>
			</div>
		</div>

		<div v-else-if="isTerminalError" class="mt-1 text-slate-400">
			<p>{{ t.state === 'Cancelled' ? 'TX CANCELLED' : t.state === 'Rejected' ? 'TX REJECTED' : 'LINK LOST' }}</p>
			<div class="flex justify-end mt-2">
				<button type="button" class="border border-slate-600 px-3 py-1 text-slate-300 hover:border-slate-300" @click="clear">
					CLEAR
				</button>
			</div>
		</div>

		<div v-else class="mt-1 text-slate-500" aria-live="polite">
			HANDSHAKE…
		</div>
	</div>
</template>
