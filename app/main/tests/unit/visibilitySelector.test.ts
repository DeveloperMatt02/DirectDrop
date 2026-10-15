import { beforeEach, describe, expect, test, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { createSSRApp, h } from 'vue';
import { renderToString } from 'vue/server-renderer';
import StatusHeader from '../../src/components/StatusHeader.vue';
import { useSessionStore } from '../../src/stores/session';
import type { VisibilityDto } from '../../src/types/qsd';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(async () => undefined) }));

const WEIGHT = /\bfont-(?:thin|extralight|light|normal|medium|semibold|bold|extrabold|black)\b/g;

async function selector(visibility: VisibilityDto) {
	useSessionStore().visibility = visibility;
	const html = await renderToString(createSSRApp({ render: () => h(StatusHeader) }));
	const group = html.match(/<div class="([^"]*)" role="radiogroup"[^>]*>([\s\S]*?)<\/div>/);
	expect(group).not.toBeNull();
	const radios = [...group![2].matchAll(/<button([^>]*)>\s*([^<]*?)\s*<\/button>/g)].map(([, attrs, label]) => ({
		cls: attrs.match(/class="([^"]*)"/)?.[1] ?? '',
		checked: attrs.includes('aria-checked="true"'),
		label,
	}));
	return { groupCls: group![1], radios };
}

describe('visibility selector', () => {
	beforeEach(() => setActivePinia(createPinia()));

	test.each<VisibilityDto>(['Visible', 'Temporarily', 'Invisible'])(
		'three equal one-line columns and one font weight (%s selected)',
		async (v) => {
			const { groupCls, radios } = await selector(v);
			expect(groupCls).toContain('grid-cols-3');
			expect(radios.map((r) => r.label)).toEqual(['Visible', 'Temporarily', 'Hidden']);
			expect(radios.filter((r) => r.checked)).toHaveLength(1);
			for (const r of radios) {
				expect(r.cls).toContain('min-w-0');
				expect(r.cls).toContain('whitespace-nowrap');
				// Weight comes from the group only: selection changes background/color, not width.
				expect(r.cls.match(WEIGHT)).toBeNull();
			}
			expect(groupCls.match(WEIGHT)).toEqual(['font-medium']);
		},
	);
});
