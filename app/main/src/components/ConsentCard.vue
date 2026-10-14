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
	}, 250);
});
onBeforeUnmount(() => {
	if (timer) clearInterval(timer);
});

const R = 26;
const CIRC = 2 * Math.PI * R;

const remainingSec = computed(() => {
	const dl = props.transfer.ui.consent_deadline;
	if (!dl) return null;
	return Math.max(0, Math.ceil((dl - now.value) / 1000));
});

const frac = computed(() => {
	const dl = props.transfer.ui.consent_deadline;
	if (!dl) return 0;
	return Math.min(1, Math.max(0, (dl - now.value) / 60_000));
});

const description = computed(() => describeTransfer(props.transfer.metadata));
const deviceName = computed(() => props.transfer.metadata?.source?.name ?? 'UNKNOWN');
const pin = computed(() => props.transfer.metadata?.pin_code ?? null);
const files = computed(() => props.transfer.metadata?.files ?? []);
</script>

<template>
	<section class="bp-sketch bg-slate-900 p-4 w-full max-w-[380px]" aria-label="Consent request">
		<div class="flex items-center justify-between gap-3">
			<p class="font-mono text-[11px] tracking-[0.2em] text-slate-400">CONSENT // UKEY2</p>
			<div class="flex items-center gap-2">
				<svg width="60" height="60" viewBox="0 0 60 60" role="timer" aria-label="Consent countdown">
					<circle cx="30" cy="30" :r="R" fill="none" stroke="#1e293b" stroke-width="5" />
					<circle
						cx="30" cy="30" :r="R" fill="none"
						:stroke="transfer.ui.expired ? '#f87171' : '#fb923c'" stroke-width="5" stroke-linecap="butt"
						:stroke-dasharray="CIRC" :stroke-dashoffset="CIRC * (1 - frac)"
						transform="rotate(-90 30 30)" style="transition: stroke-dashoffset 250ms linear;" />
					<text
						x="30" y="34" text-anchor="middle" fill="#e2e8f0" font-size="13" font-family="ui-monospace, Menlo, Consolas, monospace"
						data-testid="consent-countdown">
						{{ transfer.ui.expired ? 'EXP' : `${remainingSec ?? '–'}s` }}
					</text>
				</svg>
			</div>
		</div>
		<p class="mt-1 font-mono text-[12px] text-slate-300">
			<span class="text-sky-300">{{ deviceName }}</span>
			<span class="text-slate-500"> → </span>
			<span>{{ description || 'incoming transfer' }}</span>
		</p>
		<p
			v-if="pin" class="font-mono font-bold text-orange-400 text-center tabular-nums"
			style="font-size: 56px; letter-spacing: 0.15em; line-height: 1.2;" data-testid="consent-pin" aria-live="polite">
			{{ pin }}
		</p>
		<p v-else class="font-mono text-[12px] text-slate-500 text-center">NO PIN PROVIDED</p>
		<ul v-if="files.length > 0" class="mt-1 font-mono text-[11px] text-slate-400 space-y-0.5">
			<li
				v-for="(f, i) in files.slice(0, 3)" :key="`${f}-${i}`"
				class="truncate" :title="f">
				+ {{ baseName(f) }}
			</li>
		</ul>
		<div class="flex gap-2 mt-3">
			<button
				type="button" :disabled="disabled"
				class="flex-1 font-mono text-[13px] font-bold bg-orange-400 text-black border border-orange-400 px-4 py-2 hover:bg-orange-300 disabled:opacity-40"
				@click="emit('accept')">
				ACCEPT
			</button>
			<button
				type="button" :disabled="disabled"
				class="flex-1 font-mono text-[13px] bg-transparent text-slate-200 border border-slate-500 px-4 py-2 hover:border-slate-200 disabled:opacity-40"
				@click="emit('decline')">
				DECLINE
			</button>
		</div>
	</section>
</template>
