<script setup lang="ts">
import { computed } from 'vue';
import { storeToRefs } from 'pinia';
import { useDevicesStore } from '../stores/devices';
import { useSessionStore } from '../stores/session';
import { useTransfersStore } from '../stores/transfers';
import { humanSize } from '../types/qsd';
import FlowHero from './FlowHero.vue';
import TransferCard from './TransferCard.vue';

const session = useSessionStore();
const devices = useDevicesStore();
const transfers = useTransfersStore();
const { deviceName, port } = storeToRefs(session);

const consent = computed(() => transfers.inboundSorted.filter((t) => t.state === 'WaitingForUserConsent'));
const active = computed(() =>
	[...transfers.items.values()].filter((t) => ['SentIntroduction', 'SendingFiles', 'ReceivingFiles'].includes(t.state)),
);
const focus = computed(() => consent.value[0] ?? active.value[0] ?? null);
const live = computed(() => focus.value !== null);

const phoneName = computed(() => {
	const src = focus.value?.metadata?.source?.name;
	if (src) return src;
	return devices.sorted[0]?.name ?? 'PHONE';
});
const phoneAddr = computed(() => {
	const ep = devices.sorted.find((e) => e.name === phoneName.value) ?? devices.sorted[0];
	if (ep?.ip && ep?.port) return `${ep.ip}:${ep.port}`;
	return 'NO LINK';
});
const phoneStatus = computed(() => {
	if (active.value.length > 0) return { label: 'CONNECTED', cls: 'text-emerald-400' };
	if (consent.value.length > 0) return { label: 'REQUESTING', cls: 'text-orange-400' };
	if (devices.sorted.length > 0) return { label: 'IN RANGE', cls: 'text-sky-300' };
	return { label: 'NO LINK', cls: 'text-slate-500' };
});

const linkLine = computed(() => {
	const f = focus.value;
	if (!f) return 'LINK IDLE · AWAITING PEER';
	const ack = humanSize(f.metadata?.ack_bytes ?? 0);
	const total = humanSize(f.metadata?.total_bytes ?? 0);
	return `LINK ${f.id.slice(0, 18)} · ${f.state.toUpperCase()} · ${ack} / ${total}`;
});
</script>

<template>
	<section aria-label="Transfers and network flow" class="border border-slate-800 bg-slate-950/60 p-4">
		<div class="flex items-baseline justify-between">
			<h2 class="font-mono text-[11px] tracking-[0.2em] text-slate-400">TRANSFERS &amp; NETWORK FLOW</h2>
			<span class="font-mono text-[11px]" :class="live ? 'text-emerald-400' : 'text-slate-600'" aria-live="polite">
				{{ live ? '● LIVE' : '○ IDLE' }}
			</span>
		</div>

		<div class="mt-2 grid grid-cols-1 md:grid-cols-[180px_minmax(0,1fr)_180px] gap-4 items-start">
			<figure class="flex flex-col items-center gap-1 border border-slate-800 bg-slate-900 p-3">
				<svg width="96" height="64" viewBox="0 0 96 64" fill="none" stroke="#94a3b8" stroke-width="1.5" aria-hidden="true">
					<rect x="18" y="6" width="60" height="38" />
					<rect x="23" y="11" width="50" height="28" stroke="#38bdf8" stroke-width="1" />
					<path d="M8 52h80l-8-8H16l-8 8Z" />
					<circle cx="48" cy="48" r="1.5" fill="#94a3b8" stroke="none" />
				</svg>
				<figcaption class="font-mono text-[11px] text-slate-200 text-center truncate w-full" :title="deviceName">
					{{ deviceName || 'LAPTOP' }}
				</figcaption>
				<p class="font-mono text-[11px] text-slate-500 tabular-nums">:{{ port ?? '–' }} · THIS DEVICE</p>
			</figure>

			<div class="flex flex-col items-center gap-2 min-w-0">
				<svg viewBox="0 0 400 72" class="w-full h-[72px]" fill="none" aria-hidden="true">
					<defs>
						<marker id="bp-arrow" viewBox="0 0 8 8" refX="7" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
							<path d="M0 0 8 4 0 8" :stroke="live ? '#38bdf8' : '#475569'" stroke-width="1.2" fill="none" />
						</marker>
					</defs>
					<path
						id="bp-link" d="M8 36 H392" :stroke="live ? '#38bdf8' : '#334155'" stroke-width="1.5"
						:class="{ 'flow-line': live }" stroke-dasharray="6 6" marker-end="url(#bp-arrow)" marker-start="url(#bp-arrow)" />
					<circle v-if="live" r="2.5" fill="#38bdf8">
						<animateMotion dur="1.8s" repeatCount="indefinite">
							<mpath href="#bp-link" />
						</animateMotion>
					</circle>
					<circle v-if="live" r="1.8" fill="#fb923c">
						<animateMotion dur="2.7s" repeatCount="indefinite">
							<mpath href="#bp-link" />
						</animateMotion>
					</circle>
				</svg>
				<div v-if="focus" class="w-full max-w-[420px]">
					<TransferCard :transfer="focus" />
				</div>
				<div v-else class="w-full max-w-[420px] border border-dashed border-slate-700 p-2">
					<FlowHero :device-name="deviceName" :peer-name="devices.sorted[0]?.name ?? ''" />
				</div>
			</div>

			<figure class="flex flex-col items-center gap-1 border border-slate-800 bg-slate-900 p-3">
				<svg width="48" height="72" viewBox="0 0 48 72" fill="none" stroke="#94a3b8" stroke-width="1.5" aria-hidden="true">
					<rect x="8" y="4" width="32" height="64" rx="3" />
					<rect x="13" y="12" width="22" height="44" stroke="#38bdf8" stroke-width="1" />
					<circle cx="24" cy="61" r="1.5" fill="#94a3b8" stroke="none" />
				</svg>
				<figcaption class="font-mono text-[11px] text-slate-200 text-center truncate w-full" :title="phoneName">
					{{ phoneName }}
				</figcaption>
				<p class="font-mono text-[11px] text-slate-500 tabular-nums truncate w-full text-center">{{ phoneAddr }}</p>
				<p class="font-mono text-[11px]" :class="phoneStatus.cls">{{ phoneStatus.label }}</p>
			</figure>
		</div>

		<p class="mt-3 font-mono text-[11px] text-slate-500 truncate" aria-live="polite">{{ linkLine }}</p>
	</section>
</template>
