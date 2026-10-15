<script lang="ts" setup>
import { PropType } from 'vue';
import { ToastType } from '../../types';

defineProps({
	id: {
		type: Number,
		required: true,
	},
	type: {
		type: String as PropType<ToastType>,
		default: ToastType.Success,
	},
	message: {
		type: String,
		required: true,
	},
});
</script>

<template>
	<div
		class="w-full p-4 rounded-xl shadow-md border"
		:class="{'bg-cancel-bg text-cancel border-cancel-border': type === ToastType.Error, 'bg-rx-bg text-rx border-seam': type === ToastType.Success, 'bg-accent-soft text-ink border-seam': type === ToastType.Info}">
		<span class="text-sm">
			{{ message }}
		</span>
	</div>
</template>