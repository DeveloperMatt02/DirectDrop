<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps({
	ackBytes: { type: Number, default: 0 },
	totalBytes: { type: Number, default: 0 },
});

const pct = computed(() => {
	if (!props.totalBytes || props.totalBytes <= 0) return 0;
	return Math.min(100, Math.max(0, (props.ackBytes / props.totalBytes) * 100));
});
</script>

<template>
	<div
		class="w-full h-1 rounded-full overflow-hidden" role="progressbar"
		:style="{ background: 'var(--qsd-seam)' }"
		:aria-valuenow="Math.round(pct)" aria-valuemin="0" aria-valuemax="100">
		<div class="h-full transition-all duration-200" :style="{ width: `${pct}%`, background: 'var(--qsd-accent)' }" />
	</div>
</template>
