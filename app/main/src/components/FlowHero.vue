<script setup lang="ts">
/** Sidebar radar: this PC at the centre; slow waves only while discoverable. */
defineProps({
	active: { type: Boolean, default: false },
});
</script>

<template>
	<svg
		viewBox="0 0 200 200" class="w-full h-full" fill="none"
		aria-hidden="true">
		<circle
			cx="100" cy="100" r="94"
			class="stroke-seam" stroke-width="1" />
		<circle
			cx="100" cy="100" r="68"
			class="stroke-seam" stroke-width="1" />
		<circle
			cx="100" cy="100" r="44"
			class="stroke-seam" stroke-width="1" />
		<template v-if="active">
			<circle
				cx="100" cy="100" r="94"
				class="stroke-accent radar-wave" stroke-width="1.5" />
			<circle
				cx="100" cy="100" r="94"
				class="stroke-accent radar-wave radar-wave-late" stroke-width="1.5" />
		</template>
		<circle
			cx="100" cy="100" r="28"
			:class="active ? 'fill-accent-soft stroke-accent' : 'fill-raise stroke-seam'" stroke-width="1" />
		<g :class="active ? 'stroke-accent-text' : 'stroke-faint'" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
			<rect
				x="86" y="88" width="28"
				height="18" rx="2" />
			<path d="M80 112h40" />
		</g>
	</svg>
</template>
