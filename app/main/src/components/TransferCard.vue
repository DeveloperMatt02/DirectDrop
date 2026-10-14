<script setup lang="ts">
import { computed } from 'vue';
import { open } from '@tauri-apps/plugin-shell';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { useToastStore } from '../vue_lib/stores/useToastStore';
import { ToastType } from '../vue_lib/types';
import type { TransferView } from '../types/qsd';
import { baseName, humanSize } from '../types/qsd';
import { useTransfersStore } from '../stores/transfers';
import ConsentCard from './ConsentCard.vue';
import ProgressBar from './ProgressBar.vue';

const props = defineProps({
	transfer: { type: Object as () => TransferView, required: true },
});

const transfers = useTransfersStore();
const toasts = useToastStore();

const t = computed(() => props.transfer);
const meta = computed(() => props.transfer.metadata);
const deviceName = computed(() => meta.value?.source?.name ?? 'Unknown');
const files = computed(() => meta.value?.files ?? []);
const isConsent = computed(() => t.value.state === 'WaitingForUserConsent');
const isActive = computed(() => ['SentIntroduction', 'SendingFiles', 'ReceivingFiles'].includes(t.value.state));
const isSending = computed(() => ['SentIntroduction', 'SendingFiles'].includes(t.value.state));
const isFinished = computed(() => t.value.state === 'Finished');
const isTerminalError = computed(() => ['Cancelled', 'Rejected', 'Disconnected'].includes(t.value.state));
const progressLabel = computed(() => {
	const ack = meta.value?.ack_bytes ?? 0;
	const total = meta.value?.total_bytes ?? 0;
	if (!total) return '';
	return `${humanSize(ack)} / ${humanSize(total)}`;
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
		toasts.addToast('Unknown error while copying text', ToastType.Error);
		console.error(e);
	}
}

async function openTarget() {
	// File/text/URL actions (§4.2, PRODUCT parity): files → Show in folder best-effort
	// via shell.open on download dir; text → Copy; url → Open. Never saves text as file.
	const url = meta.value?.url ?? null;
	const fileHint = files.value[0] ?? null;
	const target = url ?? fileHint;
	if (!target) return;
	try {
		await open(target);
	} catch (e) {
		toasts.addToast('Error opening target, it may not be a valid URI', ToastType.Error);
		console.error(e);
	}
}
</script>

<template>
	<div class="w-full rounded-3xl flex flex-row gap-6 p-4 mb-4 bg-green-100">
		<div class="flex-1 flex flex-col text-sm min-w-0">
			<h4 class="text-base font-medium">
				{{ deviceName }}
			</h4>

			<ConsentCard v-if="isConsent" :transfer="transfer" @accept="accept" @decline="decline" />

			<div v-else-if="isActive" class="flex-1 flex flex-col">
				<p class="mt-2" aria-live="polite">
					{{ isSending ? 'Sending…' : 'Receiving…' }}
					<span v-if="progressLabel" class="text-xs opacity-70">{{ progressLabel }}</span>
				</p>
				<ProgressBar
					v-if="(meta?.total_bytes ?? 0) > 0" class="mt-2"
					:ack-bytes="meta?.ack_bytes ?? 0" :total-bytes="meta?.total_bytes ?? 0" />
				<ul class="mt-1">
					<li
						v-for="(f, i) in files" :key="`${f}-${i}`"
						class="overflow-hidden whitespace-nowrap text-ellipsis" :title="f">
						{{ baseName(f) }}
					</li>
				</ul>
				<p v-if="meta?.payload_kind === 'Text'" class="mt-1 italic opacity-70">
					Text
				</p>
				<p v-if="meta?.payload_kind === 'Url'" class="mt-1 overflow-hidden whitespace-nowrap text-ellipsis" :title="meta?.url ?? ''">
					{{ meta?.url }}
				</p>
				<div class="flex flex-row justify-end gap-4 mt-1">
					<button
						type="button"
						class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none"
						@click.stop="cancel">
						Cancel
					</button>
				</div>
			</div>

			<div v-else-if="isFinished" class="flex-1 flex flex-col">
				<p class="mt-2">
					<span v-if="meta?.payload_kind === 'Files'">Received {{ files.length }} file{{ files.length === 1 ? '' : 's' }}</span>
					<span v-else-if="meta?.payload_kind === 'Text'">Received text</span>
					<span v-else-if="meta?.payload_kind === 'Url'">Received link</span>
					<span v-else-if="meta?.payload_kind === 'WiFi'">Received Wi-Fi</span>
					<span v-else>Received</span>
				</p>
				<ul v-if="files.length > 0" class="mt-1">
					<li
						v-for="(f, i) in files" :key="`${f}-${i}`"
						class="overflow-hidden whitespace-nowrap text-ellipsis" :title="f">
						{{ baseName(f) }}
					</li>
				</ul>
				<p
					v-if="meta?.payload_kind === 'Text' && (meta?.text || meta?.payload_preview)"
					class="!select-text cursor-text overflow-hidden whitespace-nowrap text-ellipsis"
					:title="meta?.text ?? meta?.payload_preview ?? ''">
					{{ meta?.text ?? meta?.payload_preview }}
				</p>
				<p
					v-if="meta?.payload_kind === 'Url' && meta?.url"
					class="overflow-hidden whitespace-nowrap text-ellipsis" :title="meta?.url">
					{{ meta?.url }}
				</p>
				<p v-if="meta?.payload_kind === 'WiFi' && meta?.wifi_ssid" class="mt-1">
					Wi-Fi network {{ meta?.wifi_ssid }}
				</p>
				<div class="flex flex-row justify-end gap-4 mt-1">
					<button
						v-if="meta?.url || files.length > 0" type="button"
						class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none"
						@click.stop="openTarget">
						Open
					</button>
					<button
						v-if="(meta?.payload_kind === 'Text' || meta?.payload_kind === 'Url') && (meta?.text || meta?.url || meta?.payload_preview)"
						type="button"
						class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none"
						@click.stop="copyText">
						{{ meta?.payload_kind === 'Url' ? 'Copy link' : 'Copy' }}
					</button>
					<button
						type="button"
						class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none"
						@click.stop="clear">
						Clear
					</button>
				</div>
			</div>

			<div v-else-if="isTerminalError">
				<p class="mt-2">
					{{ t.state === 'Cancelled' ? 'Transfer cancelled' : t.state === 'Rejected' ? 'Transfer rejected' : 'Unexpected disconnection' }}
				</p>
				<div class="flex flex-row justify-end gap-4 mt-1">
					<button
						type="button"
						class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none"
						@click.stop="clear">
						Clear
					</button>
				</div>
			</div>

			<div v-else>
				<p class="mt-2 opacity-70">
					Connecting…
				</p>
			</div>
		</div>
	</div>
</template>
