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
		class="w-full h-2 rounded-full bg-gray-200 overflow-hidden" role="progressbar"
		:aria-valuenow="Math.round(pct)" aria-valuemin="0" aria-valuemax="100">
		<div class="h-full bg-green-600 transition-all duration-300" :style="{ width: `${pct}%` }" />
	</div>
</template>
