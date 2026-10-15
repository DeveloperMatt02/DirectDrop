<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import { groupTransfers } from '../stores/transferGroups';
import { useTransfersStore } from '../stores/transfers';
import TransferCard from './TransferCard.vue';

const transfers = useTransfersStore();

// Re-evaluated periodically so finished transfers move to EARLIER after midnight.
const now = ref(Date.now());
let timer: ReturnType<typeof setInterval> | null = null;
onMounted(() => {
	timer = setInterval(() => {
		now.value = Date.now();
	}, 30_000);
});
onBeforeUnmount(() => {
	if (timer) clearInterval(timer);
});

const groups = computed(() => {
	const g = groupTransfers(transfers.items.values(), transfers.timing, now.value);
	return [
		{ key: 'now', label: 'NOW', items: g.now },
		{ key: 'today', label: 'TODAY', items: g.today },
		{ key: 'earlier', label: 'EARLIER', items: g.earlier },
	].filter((group) => group.items.length > 0);
});
</script>

<template>
	<section aria-labelledby="activity-title" class="flex flex-col gap-4">
		<h2 id="activity-title" class="font-mono text-[11px] font-semibold tracking-[0.2em] text-dim">
			ACTIVITY
		</h2>
		<section
			v-for="group in groups" :key="group.key" :aria-label="group.label"
			class="flex flex-col gap-2" :data-test="`group-${group.key}`">
			<h3 class="font-mono text-[11px] tracking-[0.16em] text-faint">
				{{ group.label }} <span class="tabular-nums">· {{ group.items.length }}</span>
			</h3>
			<ul class="flex flex-col gap-2">
				<li v-for="t in group.items" :key="t.id">
					<TransferCard :transfer="t" />
				</li>
			</ul>
		</section>
		<p
			v-if="groups.length === 0"
			class="rounded-lg border border-dashed border-seam p-4 font-mono text-[11px] text-faint">
			NO ACTIVITY YET — RECEIVED AND SENT FILES SHOW UP HERE.
		</p>
	</section>
</template>
