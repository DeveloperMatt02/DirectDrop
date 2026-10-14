<script setup lang="ts">
import { computed } from 'vue';
import { useTransfersStore } from '../stores/transfers';
import TransferCard from './TransferCard.vue';

const transfers = useTransfersStore();

const connecting = computed(() =>
	[...transfers.items.values()].filter((t) =>
		!['WaitingForUserConsent', 'SentIntroduction', 'SendingFiles', 'ReceivingFiles', 'Finished', 'Cancelled', 'Rejected', 'Disconnected'].includes(t.state)),
);
const done = computed(() =>
	[...transfers.items.values()].filter((t) => ['Finished', 'Cancelled', 'Rejected', 'Disconnected'].includes(t.state)),
);
</script>

<template>
	<section aria-label="Transfer log" class="border border-slate-800 bg-slate-900 p-4 flex flex-col gap-3">
		<h2 class="font-mono text-[11px] tracking-[0.2em] text-slate-400">LOG · HANDSHAKE + COMPLETED</h2>
		<div v-if="connecting.length > 0" class="flex flex-col gap-2" aria-label="Connecting">
			<TransferCard v-for="t in connecting" :key="t.id" :transfer="t" />
		</div>
		<div v-if="done.length > 0" class="flex flex-col gap-2" aria-label="Completed">
			<TransferCard v-for="t in done" :key="t.id" :transfer="t" />
		</div>
		<p
			v-if="connecting.length === 0 && done.length === 0"
			class="border border-dashed border-slate-700 p-3 font-mono text-[11px] text-slate-500">
			LOG EMPTY — LIVE TRANSFERS RENDER IN THE FLOW DIAGRAM ABOVE.
		</p>
	</section>
</template>
