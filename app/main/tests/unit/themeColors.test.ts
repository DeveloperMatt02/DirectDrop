import { readdirSync, readFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, test } from 'vitest';

// Components must use the theme tokens (tailwind.config.cjs), never fixed colors.
const SRC = join(dirname(fileURLToPath(import.meta.url)), '../../src');
const PALETTE =
	/\b(?:bg|text|border|stroke|fill|ring|outline|divide|from|via|to|placeholder|caret|decoration|shadow)-(?:slate|gray|zinc|neutral|stone|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose|black|white)(?:-\d{2,3})?(?:\/\d+)?\b/g;
const HEX = /#[0-9a-fA-F]{3,8}\b/g;

function vueFiles(dir: string): string[] {
	return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
		const p = join(dir, e.name);
		if (e.isDirectory()) return vueFiles(p);
		return e.name.endsWith('.vue') ? [p] : [];
	});
}

function fixedColors(file: string): string[] {
	const src = readFileSync(file, 'utf8');
	const start = src.indexOf('<template>');
	const template = start >= 0 ? src.slice(start) : '';
	return [...(template.match(PALETTE) ?? []), ...(template.match(HEX) ?? [])];
}

const files = vueFiles(SRC).map((f) => relative(SRC, f));

describe('theme tokens only', () => {
	test('the scan covers the settings drawer and the toasts', () => {
		expect(files).toContain('composables/SettingsDrawer.vue');
		expect(files).toContain('vue_lib/components/atoms/ToastMessage.vue');
	});

	test.each(files)('%s has no fixed color classes', (file) => {
		expect(fixedColors(join(SRC, file))).toEqual([]);
	});
});
