<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import type { TransferView } from '../types/qsd';
import { baseName, describeTransfer } from '../types/qsd';

const props = defineProps({
	transfer: { type: Object as () => TransferView, required: true },
	disabled: { type: Boolean, default: false },
});

const emit = defineEmits(['accept', 'decline']);

const now = ref(Date.now());
let timer: ReturnType<typeof setInterval> | null = null;

onMounted(() => {
	timer = setInterval(() => {
		now.value = Date.now();
	}, 500);
});
onBeforeUnmount(() => {
	if (timer) clearInterval(timer);
});

const remainingSec = computed(() => {
	const dl = props.transfer.ui.consent_deadline;
	if (!dl) return null;
	return Math.max(0, Math.ceil((dl - now.value) / 1000));
});

const description = computed(() => describeTransfer(props.transfer.metadata));
const deviceName = computed(() => props.transfer.metadata?.source?.name ?? 'Unknown');
const pin = computed(() => props.transfer.metadata?.pin_code ?? null);
const files = computed(() => props.transfer.metadata?.files ?? []);
</script>

<template>
	<div class="flex-1 flex flex-col justify-between min-w-0">
		<p class="mt-4">
			<span class="font-medium">{{ deviceName }}</span>
			wants to share {{ description || 'some file(s).' }}
		</p>
		<ul v-if="files.length > 0" class="mt-1 text-xs opacity-80">
			<li
				v-for="(f, i) in files" :key="`${f}-${i}`"
				class="overflow-hidden whitespace-nowrap text-ellipsis" :title="f">
				{{ baseName(f) }}
			</li>
		</ul>
		<div v-if="pin" class="mt-2 flex items-center gap-2" aria-live="polite">
			<svg
				xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
				class="w-5 h-5" aria-hidden="true">
				<path d="M420-360h120l-23-129q20-10 31.5-29t11.5-42q0-33-23.5-56.5T480-640q-33 0-56.5 23.5T400-560q0 23 11.5 42t31.5 29l-23 129Zm60 280q-139-35-229.5-159.5T160-516v-244l320-120 320 120v244q0 152-90.5 276.5T480-80Zm0-84q104-33 172-132t68-220v-189l-240-90-240 90v189q0 121 68 220t172 132Zm0-316Z" />
			</svg>
			<span class="text-2xl font-medium tracking-widest tabular-nums" data-testid="consent-pin">{{ pin }}</span>
			<span v-if="remainingSec !== null" class="text-xs tabular-nums" data-testid="consent-countdown">
				{{ transfer.ui.expired ? 'expired' : `${remainingSec}s` }}
			</span>
		</div>
		<div class="flex flex-row justify-end gap-4 mt-2">
			<button
				type="button" :disabled="disabled"
				class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none disabled:opacity-50"
				@click.stop="emit('accept')">
				Accept
			</button>
			<button
				type="button" :disabled="disabled"
				class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none disabled:opacity-50"
				@click.stop="emit('decline')">
				Decline
			</button>
		</div>
	</div>
</template>
