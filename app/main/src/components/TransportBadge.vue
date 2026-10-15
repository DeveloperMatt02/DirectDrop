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

<template>
	<div v-if="label" class="mt-1 font-mono text-[11px]" data-test="transport-badge">
		<span class="border border-slate-700 px-1 text-slate-400">{{ label }}</span>
		<p v-if="notice" class="mt-1 text-amber-300" role="status">
			! {{ notice }}
		</p>
	</div>
</template>
