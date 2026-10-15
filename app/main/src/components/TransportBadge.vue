<script setup lang="ts">
import { computed } from 'vue';
import type { TransportDto } from '../types/qsd';
import { transportLabel, transportNotice } from '../types/qsd';

const props = defineProps({
	transport: { type: Object as () => TransportDto | null | undefined, default: null },
});

const label = computed(() => (props.transport ? transportLabel(props.transport.medium).toUpperCase() : null));
const notice = computed(() => transportNotice(props.transport)?.toUpperCase() ?? null);
</script>

<!-- The default slot sits next to the medium (TransferCard puts the speed there). -->
<template>
	<div v-if="label" class="flex flex-col gap-1.5 font-mono text-[11px]" data-test="transport-badge">
		<div class="flex flex-wrap items-center gap-x-3 gap-y-1">
			<span
				class="inline-flex items-center rounded-full border px-2 py-px text-[10px] font-semibold tracking-[0.08em]"
				:class="notice ? 'border-warn text-warn' : 'border-seam text-dim'">
				{{ label }}
			</span>
			<slot />
		</div>
		<p v-if="notice" class="flex items-start gap-1.5 rounded-md bg-warn-bg px-2 py-1 text-warn" role="status">
			<svg
				width="12" height="12" viewBox="0 0 12 12"
				class="mt-px shrink-0" aria-hidden="true">
				<path
					d="M6 1 11.5 10.5h-11Z" fill="none" stroke="currentColor"
					stroke-width="1.3" stroke-linejoin="round" />
				<path d="M6 4.5v3M6 8.8v.2" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" />
			</svg>
			<span>{{ notice }}</span>
		</p>
	</div>
	<div v-else-if="$slots.default" class="flex flex-wrap items-center gap-x-3 gap-y-1 font-mono text-[11px]">
		<slot />
	</div>
</template>
