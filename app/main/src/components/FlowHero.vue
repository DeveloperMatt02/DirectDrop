<script setup lang="ts">
defineProps({
	deviceName: { type: String, default: '' },
	peerName: { type: String, default: '' },
});
</script>

<template>
	<svg viewBox="0 0 420 220" class="w-full max-w-[420px] h-auto" fill="none" role="img" aria-label="Laptop and phone awaiting link">
		<g stroke="#1e293b" stroke-width="1">
			<path d="M8 8h16M16 0v16M404 8h-16M404 0v16M8 212h16M16 204v16M404 212h-16M404 204v16" />
		</g>
		<ellipse cx="210" cy="110" rx="180" ry="86" stroke="#334155" stroke-width="1" stroke-dasharray="4 6" class="flow-line" />
		<ellipse cx="210" cy="110" rx="128" ry="58" stroke="#1e293b" stroke-width="1" />
		<g stroke="#94a3b8" stroke-width="1.5">
			<rect x="52" y="72" width="76" height="52" />
			<rect x="59" y="79" width="62" height="30" stroke="#38bdf8" stroke-width="1" />
			<path d="M44 132h92l-10-8H54l-10 8Z" />
		</g>
		<g stroke="#94a3b8" stroke-width="1.5">
			<rect x="292" y="66" width="40" height="76" rx="4" />
			<rect x="298" y="78" width="28" height="46" stroke="#38bdf8" stroke-width="1" />
			<circle cx="312" cy="134" r="1.8" fill="#94a3b8" stroke="none" />
		</g>
		<circle cx="210" cy="110" r="20" stroke="#fb923c" stroke-width="1.5" />
		<path d="M214 100l-9 14h6l-3 8 9-14h-6l3-8Z" stroke="#fb923c" stroke-width="1.5" stroke-linejoin="round" />
		<circle r="3" fill="#38bdf8" stroke="none">
			<animateMotion dur="6s" repeatCount="indefinite">
				<mpath href="#bp-hero-orbit" />
			</animateMotion>
		</circle>
		<path id="bp-hero-orbit" d="M30 110a180 86 0 1 0 360 0a180 86 0 1 0 -360 0" stroke="none" />
		<g font-family="ui-monospace, Menlo, Consolas, monospace" font-size="10" fill="#64748b">
			<text x="52" y="148">{{ (deviceName || 'LAPTOP').slice(0, 14).toUpperCase() }}</text>
			<text x="292" y="156">{{ (peerName || 'PHONE').slice(0, 14).toUpperCase() }}</text>
			<text x="210" y="152" text-anchor="middle" fill="#475569">AWAITING LINK</text>
		</g>
		<g fill="#34d399" stroke="none" class="radar-ping">
			<circle cx="120" cy="60" r="2" />
			<circle cx="306" cy="52" r="2" />
		</g>
	</svg>
</template>
