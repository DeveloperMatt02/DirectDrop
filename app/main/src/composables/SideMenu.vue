<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useDevicesStore } from '../stores/devices';
import { useSessionStore } from '../stores/session';
import { clearSending, invertVisibility } from '../vue_lib/utils';
import { baseName } from '../types/qsd';

const props = defineProps({
	stagedFiles: { type: Array as () => string[], default: () => [] },
});

const emit = defineEmits(['clearSending']);

const session = useSessionStore();
const { visibility } = storeToRefs(session);
void useDevicesStore();

const isSending = computed(() => props.stagedFiles.length > 0);

async function onInvert() {
	await invertVisibility();
}

async function onClear() {
	await clearSending();
	emit('clearSending');
}

function pluralize(n: number, s: string) {
	return n === 1 ? s : `${s}s`;
}
</script>

<template>
	<div class="w-72 p-3" v-if="!isSending">
		<p class="mt-4 mb-2 pt-3 px-3">
			Visibility state
		</p>
		<button
			type="button"
			class="btn font-medium flex flex-row !justify-between w-full items-center rounded-xl active:scale-95 transition duration-150 ease-in-out p-3"
			@click="onInvert">
			<span v-if="visibility === 'Visible'">Always visible</span>
			<span v-else-if="visibility === 'Invisible'">Hidden from everyone</span>
			<span v-else>Temporarily visible</span>
			<svg
				xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
				:class="{'rotate-180': visibility === 'Invisible'}" aria-hidden="true">
				<path d="M504-480 320-664l56-56 240 240-240 240-56-56 184-184Z" />
			</svg>
		</button>
		<p class="text-xs mt-2 pb-3 px-3">
			<span v-if="visibility === 'Visible'">
				Nearby devices can share files with you, but you'll always be
				notified and have to approve each transfer before receiving it.
			</span>
			<span v-else-if="visibility === 'Invisible'">
				No one can see your device at the moment. However, keep in mind that if another
				device has saved yours before, it might still attempt to start a transfer with you.
				<br>
				<br>
				You will get a notification when someone nearby is sharing
				giving you the ability to become visible for 1 minute.
			</span>
			<span v-else>
				You are temporarily visible to everyone. Visibility turns off automatically after 60 seconds.
			</span>
		</p>
	</div>
	<div class="w-72 p-6 flex flex-col justify-between" v-else>
		<div>
			<p class="mt-4 mb-2">
				Sharing {{ stagedFiles.length }} {{ pluralize(stagedFiles.length, 'file') }}
			</p>
			<div class="bg-white w-32 h-32 rounded-2xl mb-2 flex justify-center items-center">
				<svg
					xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
					class="w-8 h-8" aria-hidden="true">
					<!-- eslint-disable-next-line -->
                    <path d="M240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v480q0 33-23.5 56.5T720-80H240Zm280-520v-200H240v640h480v-440H520ZM240-800v200-200 640-640Z" />
				</svg>
			</div>
			<p v-for="(f, i) in stagedFiles" :key="`${f}-${i}`" class="overflow-hidden whitespace-nowrap text-ellipsis" :title="f">
				{{ baseName(f) }}
			</p>
			<p class="text-xs mt-3">
				Make sure both devices are unlocked, close together, and have bluetooth turned on. Device you're sharing with need
				Quick Share turned on and visible to you.
			</p>
		</div>
		<button
			type="button"
			class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out w-fit mt-4"
			@click="onClear">
			Cancel
		</button>
	</div>
</template>
