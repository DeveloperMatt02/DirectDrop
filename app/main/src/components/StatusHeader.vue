<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useSessionStore } from '../stores/session';
import type { VisibilityDto } from '../types/qsd';

const emit = defineEmits(['openSettings']);

const session = useSessionStore();
const { visibility, deviceName } = storeToRefs(session);

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

const dotClass = computed(() => {
	if (visibility.value === 'Visible') return 'bg-emerald-400 pulse-dot';
	if (visibility.value === 'Temporarily') return 'bg-amber-400 pulse-dot';
	return 'bg-slate-600';
});

async function setVisibility(v: VisibilityDto) {
	await session.setVisibility(v);
}
</script>

<template>
	<header class="flex items-center gap-3 h-14 px-4 shrink-0 border-b border-slate-800 bg-slate-950">
		<svg width="30" height="30" viewBox="0 0 32 32" fill="none" stroke="currentColor" stroke-width="1.5" class="text-slate-200 shrink-0" aria-label="Linux">
			<path d="M16 3C10.5 3 7.5 9.5 7.5 16.5c0 6 3.8 10.5 8.5 10.5s8.5-4.5 8.5-10.5C24.5 9.5 21.5 3 16 3Z" />
			<ellipse cx="16" cy="19" rx="4.5" ry="6" />
			<circle cx="13.5" cy="11" r="0.9" fill="currentColor" stroke="none" />
			<circle cx="18.5" cy="11" r="0.9" fill="currentColor" stroke="none" />
			<path d="M14.5 14.5 16 16.5 17.5 14.5" class="stroke-orange-400" />
			<path d="M9 18C7 19.5 6 21.5 6.5 23M23 18c2 1.5 3 3.5 2.5 5" />
			<path d="M11 27.5h3.5M17.5 27.5H21" />
		</svg>
		<div class="flex flex-col leading-none min-w-0">
			<span class="font-mono font-bold tracking-[0.2em] text-lg text-slate-100">DirectDrop</span>
			<span class="font-mono text-[11px] text-slate-500 truncate">directdrop · {{ deviceName || '…' }}</span>
		</div>
		<div class="flex-1" />
		<span class="hidden md:inline-block w-2.5 h-2.5 rounded-full shrink-0" :class="dotClass" aria-hidden="true" />
		<span class="qsd-label hidden md:inline" aria-live="polite">{{ visibility.toUpperCase() }}</span>
		<div class="flex items-center border border-slate-700 rounded-sm overflow-hidden font-mono text-[12px]" role="radiogroup" aria-label="Device visibility">
			<button
				type="button" role="radio" :aria-checked="visibility === 'Visible'"
				class="px-3 py-1.5 border-r border-slate-700 text-slate-400 hover:bg-slate-900"
				:class="{ 'bg-sky-950 text-sky-300': visibility === 'Visible' }"
				@click="setVisibility('Visible')">
				Visible
			</button>
			<button
				type="button" role="radio" :aria-checked="visibility === 'Temporarily'"
				class="px-3 py-1.5 border-r border-slate-700 text-slate-400 hover:bg-slate-900"
				:class="{ 'bg-sky-950 text-sky-300': visibility === 'Temporarily' }"
				@click="setVisibility('Temporarily')">
				Temporarily
			</button>
			<button
				type="button" role="radio" :aria-checked="visibility === 'Invisible'"
				class="px-3 py-1.5 text-slate-400 hover:bg-slate-900"
				:class="{ 'bg-sky-950 text-sky-300': visibility === 'Invisible' }"
				@click="setVisibility('Invisible')">
				Hidden
			</button>
		</div>
		<span v-if="tempLeft !== null" class="font-mono text-[12px] text-amber-300 tabular-nums" aria-live="polite">
			0:{{ String(tempLeft).padStart(2, '0') }}
		</span>
		<button
			type="button" class="p-2 border border-slate-700 rounded-sm text-slate-400 hover:bg-slate-900 hover:text-slate-200" aria-label="Open settings"
			@click="emit('openSettings')">
			<svg xmlns="http://www.w3.org/2000/svg" height="18" viewBox="0 -960 960 960" width="18" aria-hidden="true" fill="currentColor">
				<path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-106q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8-57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Z" />
			</svg>
		</button>
	</header>
</template>
