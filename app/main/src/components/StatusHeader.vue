<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useSessionStore } from '../stores/session';
import type { VisibilityDto } from '../types/qsd';
import FlowHero from './FlowHero.vue';

const emit = defineEmits(['openSettings']);

const session = useSessionStore();
const { visibility, deviceName, downloadPath, port } = storeToRefs(session);

const markedAt = ref<number | null>(null);
const now = ref(Date.now());
let timer: ReturnType<typeof setInterval> | null = null;

onMounted(() => {
	timer = setInterval(() => {
		now.value = Date.now();
	}, 500);
});
onBeforeUnmount(() => {
	if (timer) clearInterval(timer);
});

watch(visibility, (v) => {
	markedAt.value = v === 'Temporarily' ? Date.now() : null;
});

const tempLeft = computed(() => {
	if (visibility.value !== 'Temporarily' || markedAt.value === null) return null;
	return Math.max(0, 60 - Math.floor((now.value - markedAt.value) / 1000));
});

const statusLine = computed(() => {
	if (visibility.value === 'Visible') return 'VISIBLE TO NEARBY DEVICES';
	if (visibility.value === 'Temporarily') {
		return tempLeft.value !== null
			? `VISIBLE FOR 0:${String(tempLeft.value).padStart(2, '0')}`
			: 'TEMPORARILY VISIBLE';
	}
	return 'HIDDEN FROM NEARBY DEVICES';
});

const options: { value: VisibilityDto; label: string }[] = [
	{ value: 'Visible', label: 'Visible' },
	{ value: 'Temporarily', label: 'Temporarily' },
	{ value: 'Invisible', label: 'Hidden' },
];

async function setVisibility(v: VisibilityDto) {
	await session.setVisibility(v);
}
</script>

<template>
	<aside
		aria-label="This device"
		class="shrink-0 flex flex-col gap-3 p-3 border-b border-seam bg-panel min-[760px]:w-[300px] min-[760px]:h-full min-[760px]:p-5 min-[760px]:gap-5 min-[760px]:border-b-0 min-[760px]:border-r min-[760px]:overflow-y-auto">
		<div class="flex items-center gap-2">
			<img
				src="/app-icon.png" width="28" height="28"
				alt="" class="shrink-0 rounded-md">
			<span class="text-[17px] font-semibold tracking-tight text-ink">
				DirectDrop
			</span>
			<span class="flex-1" />
			<button
				type="button" class="qsd-action px-0" aria-label="Open settings"
				@click="emit('openSettings')">
				<svg
					xmlns="http://www.w3.org/2000/svg" height="18" viewBox="0 -960 960 960"
					width="18" aria-hidden="true" fill="currentColor">
					<path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-106q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8-57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Z" />
				</svg>
			</button>
		</div>

		<div class="flex flex-wrap items-center gap-3 min-[760px]:flex-col min-[760px]:items-stretch min-[760px]:gap-5">
			<div class="flex flex-1 items-center gap-3 min-w-[200px] min-[760px]:flex-col min-[760px]:text-center">
				<div class="w-14 h-14 shrink-0 min-[760px]:w-[184px] min-[760px]:h-[184px]">
					<FlowHero :active="visibility !== 'Invisible'" />
				</div>
				<div class="min-w-0 min-[760px]:w-full">
					<p class="text-[15px] font-semibold text-ink truncate" :title="deviceName">
						{{ deviceName || '…' }}
					</p>
					<p class="mt-0.5 font-mono text-[11px] tabular-nums text-dim" aria-live="polite">
						{{ statusLine }}
					</p>
				</div>
			</div>

			<div
				class="grid grid-cols-3 gap-1 p-1 flex-1 min-w-[260px] rounded-lg border border-seam bg-raise text-[12px] font-medium"
				role="radiogroup" aria-label="Device visibility" data-test="visibility-selector">
				<button
					v-for="o in options" :key="o.value" type="button"
					role="radio" :aria-checked="visibility === o.value"
					class="min-w-0 min-h-[40px] px-1 rounded-md whitespace-nowrap"
					:class="visibility === o.value ? 'bg-accent-soft text-accent-text' : 'text-dim hover:text-ink hover:bg-panel'"
					@click="setVisibility(o.value)">
					{{ o.label }}
				</button>
			</div>
		</div>

		<dl class="flex flex-wrap gap-x-4 gap-y-1 font-mono text-[11px] min-w-0 min-[760px]:mt-auto min-[760px]:flex-col">
			<div class="flex gap-2 min-w-0">
				<dt class="text-faint shrink-0">
					DOWNLOADS
				</dt>
				<dd class="text-dim truncate" :title="downloadPath ?? ''">
					{{ downloadPath ?? 'SYSTEM DOWNLOAD DIR' }}
				</dd>
			</div>
			<div v-if="port !== null" class="flex gap-2">
				<dt class="text-faint">
					PORT
				</dt>
				<dd class="text-dim tabular-nums">
					{{ port }}
				</dd>
			</div>
		</dl>
	</aside>
</template>
