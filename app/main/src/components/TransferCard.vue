<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import type { TransferView } from '../types/qsd';
import { HANDSHAKE_STATES, baseName, humanSize } from '../types/qsd';
import { avgSpeedBps, formatClock } from '../stores/transferGroups';
import { useTransfersStore } from '../stores/transfers';
import ConsentCard from './ConsentCard.vue';
import TransportBadge from './TransportBadge.vue';

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
const isInbound = computed(() => t.value.kind === 'Inbound');
const timing = computed(() => transfers.timing.get(t.value.id));

/** Row title: "RECEIVED FROM <peer>", "SEND TO <peer> CANCELLED", … */
const title = computed(() => {
	const rx = isInbound.value;
	const st = t.value.state;
	if (isActive.value) return { verb: rx ? 'RECEIVING FROM' : 'SENDING TO', suffix: '' };
	if (st === 'Finished') return { verb: rx ? 'RECEIVED FROM' : 'SENT TO', suffix: '' };
	const failed = { Cancelled: 'CANCELLED', Rejected: 'DECLINED', Disconnected: 'INTERRUPTED' } as Record<string, string>;
	if (failed[st]) return { verb: rx ? 'RECEIVE FROM' : 'SEND TO', suffix: failed[st] };
	if (isOutboundWaiting.value) return { verb: 'WAITING FOR', suffix: '' };
	return { verb: rx ? 'CONNECTING WITH' : 'CONNECTING TO', suffix: '' };
});

const status = computed(() => {
	const st = t.value.state;
	const dir = isInbound.value ? 'text-rx' : 'text-tx';
	if (st === 'Finished') return { label: 'DONE', cls: dir };
	if (st === 'Cancelled') return { label: 'CANCELLED', cls: 'text-cancel' };
	if (st === 'Rejected') return { label: 'DECLINED', cls: 'text-cancel' };
	if (st === 'Disconnected') return { label: 'LINK LOST', cls: 'text-cancel' };
	if (isActive.value) return { label: `${Math.round(frac.value * 100)}%`, cls: dir };
	if (isOutboundWaiting.value && st === 'WaitingForUserConsent') return { label: 'WAITING', cls: 'text-dim' };
	return { label: 'HANDSHAKE', cls: 'text-dim' };
});

/** Metadata line: file count and size, or the payload kind. */
const summary = computed(() => {
	const m = meta.value;
	if (!m) return '';
	const size = (m.total_bytes ?? 0) > 0 ? ` · ${humanSize(m.total_bytes)}` : '';
	if (files.value.length > 0 || m.payload_kind === 'Files') {
		const n = files.value.length;
		return `${n} FILE${n === 1 ? '' : 'S'}${size}`;
	}
	if (m.payload_kind === 'Text') return `TEXT${size}`;
	if (m.payload_kind === 'Url') return 'LINK';
	if (m.payload_kind === 'WiFi') return `WI-FI${m.wifi_ssid ? ` · ${m.wifi_ssid}` : ''}`;
	return size.slice(3);
});

const clockLabel = computed(() => {
	const tm = timing.value;
	if (!tm) return null;
	return formatClock(tm.endedAt ?? tm.startedAt, Date.now());
});

/** AVG speed of a finished transfer (handshake included, see avgSpeedBps). */
const avgBps = computed(() =>
	(isFinished.value ? avgSpeedBps(meta.value?.total_bytes ?? 0, timing.value) : null),
);

const frac = computed(() => {
	const ack = meta.value?.ack_bytes ?? 0;
	const total = meta.value?.total_bytes ?? 0;
	if (!total || total <= 0) return 0;
	return Math.min(1, Math.max(0, ack / total));
});

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
	<article
		v-else class="flex gap-3 rounded-lg border border-seam bg-panel p-3"
		:data-test="`transfer-${t.kind.toLowerCase()}`">
		<span
			class="mt-0.5 grid h-9 w-9 shrink-0 place-items-center rounded-full"
			:class="isInbound ? 'bg-rx-bg text-rx' : 'bg-tx-bg text-tx'" aria-hidden="true">
			<svg
				width="16" height="16" viewBox="0 0 16 16"
				fill="none" stroke="currentColor" stroke-width="1.8">
				<path v-if="isInbound" d="M8 2.5v10M3.5 8.5 8 13l4.5-4.5" stroke-linecap="round" stroke-linejoin="round" />
				<path v-else d="M8 13.5v-10M3.5 7.5 8 3l4.5 4.5" stroke-linecap="round" stroke-linejoin="round" />
			</svg>
		</span>

		<div class="min-w-0 flex-1">
			<div class="flex min-w-0 items-baseline gap-2">
				<p class="min-w-0 flex-1 truncate text-[13px] text-ink" :title="deviceName">
					<span class="font-mono text-[11px] tracking-[0.06em] text-dim">{{ title.verb }}</span>
					{{ ' ' }}<span class="font-semibold">{{ deviceName }}</span>
					<template v-if="title.suffix">
						{{ ' ' }}<span class="font-mono text-[11px] tracking-[0.06em] text-cancel">{{ title.suffix }}</span>
					</template>
				</p>
				<span v-if="clockLabel" class="shrink-0 font-mono text-[11px] tabular-nums text-faint">
					{{ clockLabel }}
				</span>
			</div>
			<p class="mt-0.5 flex flex-wrap items-center gap-x-2 font-mono text-[11px] tabular-nums text-dim">
				<span v-if="summary">{{ summary }}</span>
				<span v-if="summary" aria-hidden="true">·</span>
				<span class="font-semibold" :class="status.cls">{{ status.label }}</span>
			</p>

			<TransportBadge :transport="t.transport" class="mt-1.5">
				<span v-if="isActive" class="tabular-nums" :class="isSending ? 'text-tx' : 'text-rx'" aria-live="polite">
					{{ isSending ? 'TX' : 'RX' }} {{ humanSize(speedBps) }}/s
				</span>
				<span v-else-if="avgBps !== null" class="tabular-nums text-dim" data-test="avg-speed">
					AVG {{ humanSize(avgBps) }}/s
				</span>
			</TransportBadge>

			<template v-if="isOutboundWaiting">
				<p class="mt-1.5 font-mono text-[11px] text-dim" aria-live="polite">
					WAITING FOR ACCEPT ON {{ outboundPeer.toUpperCase() }} — CHECK THE PIN THERE, NO BUTTON HERE<template v-if="outboundPin">
						· PIN <span class="font-semibold text-ink">{{ outboundPin }}</span>
					</template>
				</p>
				<div class="mt-2 flex justify-end">
					<button type="button" class="qsd-action qsd-action-cancel" @click="cancel">
						CANCEL
					</button>
				</div>
			</template>

			<template v-else-if="isActive">
				<div
					class="mt-2 h-1.5 overflow-hidden rounded-full bg-raise" role="progressbar"
					:aria-valuenow="Math.round(frac * 100)" aria-valuemin="0" aria-valuemax="100"
					:aria-label="`Progress ${Math.round(frac * 100)}%`">
					<div
						class="h-full rounded-full transition-[width] duration-300" :class="isSending ? 'bg-tx' : 'bg-rx'"
						:style="{ width: `${frac * 100}%` }" />
				</div>
				<div class="mt-1 flex justify-between gap-2 font-mono text-[11px] tabular-nums text-dim">
					<span>{{ humanSize(meta?.ack_bytes ?? 0) }} / {{ humanSize(meta?.total_bytes ?? 0) }}</span>
					<span>ETA {{ eta }}</span>
				</div>
				<ul class="mt-1 font-mono text-[11px] text-faint">
					<li v-for="(f, i) in files.slice(0, 3)" :key="`${f}-${i}`" class="truncate" :title="f">
						{{ baseName(f) }}
					</li>
				</ul>
				<div class="mt-2 flex justify-end">
					<button type="button" class="qsd-action qsd-action-cancel" @click="cancel">
						CANCEL
					</button>
				</div>
			</template>

			<div v-else-if="isFinished" class="mt-2 flex flex-wrap justify-end gap-2">
				<template v-if="meta?.payload_kind === 'Files'">
					<button
						v-if="t.kind === 'Inbound' && files.length === 1"
						type="button" class="qsd-action" @click="openFile">
						OPEN FILE
					</button>
					<button
						v-if="t.kind === 'Inbound' && files.length > 0"
						type="button" class="qsd-action" @click="openFolder">
						OPEN FOLDER
					</button>
				</template>
				<button
					v-else-if="meta?.url"
					type="button" class="qsd-action" @click="openTarget">
					OPEN
				</button>
				<button
					v-if="(meta?.payload_kind === 'Text' || meta?.payload_kind === 'Url') && (meta?.text || meta?.url || meta?.payload_preview)"
					type="button" class="qsd-action" @click="copyText">
					{{ meta?.payload_kind === 'Url' ? 'COPY LINK' : 'COPY' }}
				</button>
				<button type="button" class="qsd-action" @click="clear">
					CLEAR
				</button>
			</div>

			<div v-else-if="isTerminalError" class="mt-2 flex justify-end">
				<button type="button" class="qsd-action" @click="clear">
					CLEAR
				</button>
			</div>
		</div>
	</article>
</template>
