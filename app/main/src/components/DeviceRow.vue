<script setup lang="ts">
import { computed } from 'vue';
import type { EndpointDto } from '../types/qsd';

const props = defineProps({
	endpoint: { type: Object as () => EndpointDto, required: true },
	disabled: { type: Boolean, default: false },
});

const emit = defineEmits(['select']);

const label = computed(() => props.endpoint.name ?? 'UNKNOWN');
const addr = computed(() => {
	if (props.endpoint.ip && props.endpoint.port) return `${props.endpoint.ip}:${props.endpoint.port}`;
	return 'ADDR?';
});
</script>

<template>
	<div class="border border-slate-700 bg-slate-950 p-3 flex items-center gap-3 min-w-0">
		<svg width="26" height="34" viewBox="0 0 26 38" fill="none" stroke="#94a3b8" stroke-width="1.2" class="shrink-0" aria-hidden="true">
			<rect x="3" y="2" width="20" height="34" rx="2" />
			<circle cx="13" cy="31" r="1.2" fill="#94a3b8" stroke="none" />
		</svg>
		<span class="flex-1 min-w-0">
			<span class="flex items-center gap-2">
				<span class="font-mono text-[13px] font-bold text-slate-100 truncate">{{ label }}</span>
				<span class="font-mono text-[10px] border border-emerald-700 text-emerald-400 px-1.5 py-px shrink-0">LAN</span>
			</span>
			<span class="font-mono text-[11px] text-slate-500 tabular-nums truncate block">{{ addr }}</span>
		</span>
		<button
			type="button" :disabled="disabled"
			class="font-mono text-[12px] font-bold bg-sky-950 border border-sky-600 text-sky-300 px-4 py-1.5 shrink-0 hover:bg-sky-900 disabled:opacity-40"
			:aria-label="`Send staged files to ${label}`"
			@click="emit('select', endpoint.id)">
			SEND
		</button>
	</div>
</template>
