<script setup lang="ts">
import { computed } from 'vue';
import type { EndpointDto } from '../types/qsd';

const props = defineProps({
	endpoint: { type: Object as () => EndpointDto, required: true },
	selected: { type: Boolean, default: false },
	disabled: { type: Boolean, default: false },
});

const emit = defineEmits(['select']);

const label = computed(() => props.endpoint.name ?? 'Unknown');
</script>

<template>
	<button
		type="button" :disabled="disabled"
		class="w-full rounded-3xl flex flex-row gap-6 p-4 mb-4 bg-green-100 text-left
			active:scale-[0.99] transition duration-150 ease-in-out disabled:opacity-50"
		:class="{ 'outline outline-2 outline-green-600': selected }"
		@click="emit('select', endpoint.id)">
		<div class="h-14 w-14 rounded-full bg-white shrink-0" aria-hidden="true" />
		<span class="flex-1 flex flex-col justify-center text-sm min-w-0">
			<span class="text-base font-medium overflow-hidden whitespace-nowrap text-ellipsis">{{ label }}</span>
			<span v-if="endpoint.device_type" class="text-xs opacity-70">{{ endpoint.device_type }}</span>
		</span>
	</button>
</template>
