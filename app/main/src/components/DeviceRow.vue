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
	<div class="flex min-w-0 items-center gap-3 rounded-lg border border-seam bg-ground px-3 py-2">
		<svg
			width="18" height="26" viewBox="0 0 26 38"
			fill="none" class="shrink-0 stroke-faint" stroke-width="1.6" aria-hidden="true">
			<rect
				x="3" y="2" width="20"
				height="34" rx="3" />
			<path d="M10 31h6" stroke-linecap="round" />
		</svg>
		<span class="min-w-0 flex-1">
			<span class="flex items-center gap-2">
				<span class="truncate text-[13px] font-semibold text-ink">{{ label }}</span>
				<span class="shrink-0 rounded-full border border-seam px-1.5 font-mono text-[10px] text-dim">LAN</span>
			</span>
			<span class="block truncate font-mono text-[11px] tabular-nums text-faint">{{ addr }}</span>
		</span>
		<button
			type="button" :disabled="disabled"
			class="qsd-action qsd-action-primary shrink-0"
			:aria-label="`Send staged files to ${label}`"
			@click="emit('select', endpoint.id)">
			SEND
		</button>
	</div>
</template>
