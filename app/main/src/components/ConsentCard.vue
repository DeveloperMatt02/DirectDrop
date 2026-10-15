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
	<section class="w-full max-w-[560px] rounded-xl border border-accent bg-panel p-4" aria-label="Consent request">
		<div class="flex items-start gap-4">
			<div class="min-w-0 flex-1">
				<p class="font-mono text-[11px] font-semibold tracking-[0.2em] text-dim">
					CONSENT // UKEY2
				</p>
				<p class="mt-1 text-[13px] text-ink">
					<span class="font-semibold">{{ deviceName }}</span>
					<span class="text-faint"> → </span>
					<span class="text-dim">{{ description || 'incoming transfer' }}</span>
				</p>
			</div>
			<svg
				width="60" height="60" viewBox="0 0 60 60"
				class="shrink-0" role="timer" aria-label="Consent countdown">
				<circle
					cx="30" cy="30" :r="R"
					fill="none" class="stroke-raise" stroke-width="5" />
				<circle
					cx="30" cy="30" :r="R"
					fill="none" :class="transfer.ui.expired ? 'stroke-cancel' : 'stroke-accent'" stroke-width="5"
					stroke-linecap="butt" :stroke-dasharray="CIRC" :stroke-dashoffset="CIRC * (1 - frac)"
					transform="rotate(-90 30 30)" style="transition: stroke-dashoffset 250ms linear;" />
				<text
					x="30" y="34" text-anchor="middle"
					class="fill-ink" font-size="13" font-family="ui-monospace, Menlo, Consolas, monospace"
					data-testid="consent-countdown">
					{{ transfer.ui.expired ? 'EXP' : `${remainingSec ?? '–'}s` }}
				</text>
			</svg>
		</div>
		<p
			v-if="pin" class="mt-2 text-center font-mono font-semibold tabular-nums text-ink"
			style="font-size: 56px; letter-spacing: 0.15em; line-height: 1.2;" data-testid="consent-pin" aria-live="polite">
			{{ pin }}
		</p>
		<p v-else class="mt-2 text-center font-mono text-[12px] text-faint">
			NO PIN PROVIDED
		</p>
		<ul v-if="files.length > 0" class="mt-1 space-y-0.5 font-mono text-[11px] text-dim">
			<li
				v-for="(f, i) in files.slice(0, 3)" :key="`${f}-${i}`"
				class="truncate" :title="f">
				{{ baseName(f) }}
			</li>
		</ul>
		<div class="mt-3 flex gap-2">
			<button
				type="button" :disabled="disabled"
				class="qsd-action qsd-action-primary flex-1"
				@click="emit('accept')">
				ACCEPT
			</button>
			<button
				type="button" :disabled="disabled"
				class="qsd-action flex-1"
				@click="emit('decline')">
				DECLINE
			</button>
		</div>
	</section>
</template>
