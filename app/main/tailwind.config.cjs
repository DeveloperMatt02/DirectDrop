/* eslint-disable @typescript-eslint/no-require-imports */
/* eslint-disable no-undef */
const resolve = require('path').resolve

/** Theme colors: CSS variables from src/vue_lib/assets/main.postcss (light/dark sets). */
const v = (name) => `var(--qsd-${name})`

module.exports = {
	content: [
		resolve(__dirname, 'index.html'),
		resolve(__dirname, 'src/**/*.{vue,ts}'),
		resolve(__dirname, '../common/vue_lib/src/**/*.{vue,ts}')
	],
	theme: {
		extend: {
			colors: {
				ground: v('ground'),
				panel: v('panel'),
				raise: v('raise'),
				seam: v('seam'),
				ink: v('ink'),
				dim: v('dim'),
				faint: v('faint'),
				accent: {
					DEFAULT: v('accent'),
					text: v('accent-text'),
					soft: v('accent-soft'),
				},
				primary: {
					DEFAULT: v('primary'),
					ink: v('primary-ink'),
				},
				rx: {
					DEFAULT: v('rx'),
					bg: v('rx-bg'),
				},
				tx: {
					DEFAULT: v('tx'),
					bg: v('tx-bg'),
				},
				cancel: {
					DEFAULT: v('cancel'),
					bg: v('cancel-bg'),
					border: v('cancel-border'),
				},
				warn: {
					DEFAULT: v('warn'),
					bg: v('warn-bg'),
				},
			},
		},
	},
	plugins: [
		require('@tailwindcss/typography'),
	],
}
