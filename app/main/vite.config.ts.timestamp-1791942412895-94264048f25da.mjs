// vite.config.ts
import vue from "file:///home/matteotrossi/orca/workspaces/quickshare-direct/feat-gui-frontend/app/main/node_modules/.pnpm/@vitejs+plugin-vue@5.1.4_vite@5.4.10_@types+node@20.16.5__vue@3.5.12_typescript@5.6.3_/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import { resolve } from "path";
import AutoImport from "file:///home/matteotrossi/orca/workspaces/quickshare-direct/feat-gui-frontend/app/main/node_modules/.pnpm/unplugin-auto-import@0.19.0_rollup@4.21.3/node_modules/unplugin-auto-import/dist/vite.js";
import { defineConfig } from "file:///home/matteotrossi/orca/workspaces/quickshare-direct/feat-gui-frontend/app/main/node_modules/.pnpm/vitest@2.1.4_@types+node@20.16.5/node_modules/vitest/dist/config.js";
import path from "path";
var __vite_injected_original_dirname = "/home/matteotrossi/orca/workspaces/quickshare-direct/feat-gui-frontend/app/main";
var vite_config_default = defineConfig({
  plugins: [
    vue(),
    // See https://github.com/unplugin/unplugin-auto-import
    AutoImport({
      imports: ["vue"],
      dts: "./src/auto-imports.d.ts",
      eslintrc: {
        enabled: true,
        filepath: resolve(__vite_injected_original_dirname, ".eslintrc-auto-import.json")
      }
    })
  ],
  clearScreen: false,
  envPrefix: ["VITE_", "TAURI_"],
  server: {
    port: 1420,
    strictPort: true,
    fs: {
      allow: [
        // Optionally, include other directories
        path.resolve(__vite_injected_original_dirname)
      ]
    }
  },
  build: {
    outDir: "./dist",
    // See https://tauri.app/v1/references/webview-versions for details
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari15",
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
    emptyOutDir: true
  },
  // See https://vitest.dev/config/
  test: {
    include: ["tests/unit/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"]
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9tYXR0ZW90cm9zc2kvb3JjYS93b3Jrc3BhY2VzL3F1aWNrc2hhcmUtZGlyZWN0L2ZlYXQtZ3VpLWZyb250ZW5kL2FwcC9tYWluXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCIvaG9tZS9tYXR0ZW90cm9zc2kvb3JjYS93b3Jrc3BhY2VzL3F1aWNrc2hhcmUtZGlyZWN0L2ZlYXQtZ3VpLWZyb250ZW5kL2FwcC9tYWluL3ZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9ob21lL21hdHRlb3Ryb3NzaS9vcmNhL3dvcmtzcGFjZXMvcXVpY2tzaGFyZS1kaXJlY3QvZmVhdC1ndWktZnJvbnRlbmQvYXBwL21haW4vdml0ZS5jb25maWcudHNcIjtpbXBvcnQgdnVlIGZyb20gJ0B2aXRlanMvcGx1Z2luLXZ1ZSdcbmltcG9ydCB7IHJlc29sdmUgfSBmcm9tICdwYXRoJ1xuaW1wb3J0IEF1dG9JbXBvcnQgZnJvbSAndW5wbHVnaW4tYXV0by1pbXBvcnQvdml0ZSdcbmltcG9ydCB7IGRlZmluZUNvbmZpZyB9IGZyb20gJ3ZpdGVzdC9jb25maWcnXG5pbXBvcnQgcGF0aCBmcm9tICdwYXRoJ1xuXG4vLyBTZWUgaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZyh7XG5cdHBsdWdpbnM6IFtcblx0XHR2dWUoKSxcblx0XHQvLyBTZWUgaHR0cHM6Ly9naXRodWIuY29tL3VucGx1Z2luL3VucGx1Z2luLWF1dG8taW1wb3J0XG5cdFx0QXV0b0ltcG9ydCh7XG5cdFx0XHRpbXBvcnRzOiBbJ3Z1ZSddLFxuXHRcdFx0ZHRzOiAnLi9zcmMvYXV0by1pbXBvcnRzLmQudHMnLFxuXHRcdFx0ZXNsaW50cmM6IHtcblx0XHRcdFx0ZW5hYmxlZDogdHJ1ZSxcblx0XHRcdFx0ZmlsZXBhdGg6IHJlc29sdmUoX19kaXJuYW1lLCAnLmVzbGludHJjLWF1dG8taW1wb3J0Lmpzb24nKSxcblx0XHRcdH0sXG5cdFx0fSksXG5cdF0sXG5cdGNsZWFyU2NyZWVuOiBmYWxzZSxcblx0ZW52UHJlZml4OiBbJ1ZJVEVfJywgJ1RBVVJJXyddLFxuXHRzZXJ2ZXI6IHtcblx0XHRwb3J0OiAxNDIwLFxuXHRcdHN0cmljdFBvcnQ6IHRydWUsXG5cdFx0ZnM6IHtcblx0XHRcdGFsbG93OiBbXG5cdFx0XHRcdC8vIE9wdGlvbmFsbHksIGluY2x1ZGUgb3RoZXIgZGlyZWN0b3JpZXNcblx0XHRcdFx0cGF0aC5yZXNvbHZlKF9fZGlybmFtZSlcblx0XHRcdF1cblx0XHR9XG5cdH0sXG5cdGJ1aWxkOiB7XG5cdFx0b3V0RGlyOiAnLi9kaXN0Jyxcblx0XHQvLyBTZWUgaHR0cHM6Ly90YXVyaS5hcHAvdjEvcmVmZXJlbmNlcy93ZWJ2aWV3LXZlcnNpb25zIGZvciBkZXRhaWxzXG5cdFx0dGFyZ2V0OiBwcm9jZXNzLmVudi5UQVVSSV9QTEFURk9STSA9PSAnd2luZG93cycgPyAnY2hyb21lMTA1JyA6ICdzYWZhcmkxNScsXG5cdFx0bWluaWZ5OiAhcHJvY2Vzcy5lbnYuVEFVUklfREVCVUcgPyAnZXNidWlsZCcgOiBmYWxzZSxcblx0XHRzb3VyY2VtYXA6ICEhcHJvY2Vzcy5lbnYuVEFVUklfREVCVUcsXG5cdFx0ZW1wdHlPdXREaXI6IHRydWUsXG5cdH0sXG5cdC8vIFNlZSBodHRwczovL3ZpdGVzdC5kZXYvY29uZmlnL1xuXHR0ZXN0OiB7XG5cdFx0aW5jbHVkZTogWyd0ZXN0cy91bml0LyoqLyoue3Rlc3Qsc3BlY30ue2pzLG1qcyxjanMsdHMsbXRzLGN0cyxqc3gsdHN4fSddLFxuXHR9LFxufSlcbiJdLAogICJtYXBwaW5ncyI6ICI7QUFBK1osT0FBTyxTQUFTO0FBQy9hLFNBQVMsZUFBZTtBQUN4QixPQUFPLGdCQUFnQjtBQUN2QixTQUFTLG9CQUFvQjtBQUM3QixPQUFPLFVBQVU7QUFKakIsSUFBTSxtQ0FBbUM7QUFPekMsSUFBTyxzQkFBUSxhQUFhO0FBQUEsRUFDM0IsU0FBUztBQUFBLElBQ1IsSUFBSTtBQUFBO0FBQUEsSUFFSixXQUFXO0FBQUEsTUFDVixTQUFTLENBQUMsS0FBSztBQUFBLE1BQ2YsS0FBSztBQUFBLE1BQ0wsVUFBVTtBQUFBLFFBQ1QsU0FBUztBQUFBLFFBQ1QsVUFBVSxRQUFRLGtDQUFXLDRCQUE0QjtBQUFBLE1BQzFEO0FBQUEsSUFDRCxDQUFDO0FBQUEsRUFDRjtBQUFBLEVBQ0EsYUFBYTtBQUFBLEVBQ2IsV0FBVyxDQUFDLFNBQVMsUUFBUTtBQUFBLEVBQzdCLFFBQVE7QUFBQSxJQUNQLE1BQU07QUFBQSxJQUNOLFlBQVk7QUFBQSxJQUNaLElBQUk7QUFBQSxNQUNILE9BQU87QUFBQTtBQUFBLFFBRU4sS0FBSyxRQUFRLGdDQUFTO0FBQUEsTUFDdkI7QUFBQSxJQUNEO0FBQUEsRUFDRDtBQUFBLEVBQ0EsT0FBTztBQUFBLElBQ04sUUFBUTtBQUFBO0FBQUEsSUFFUixRQUFRLFFBQVEsSUFBSSxrQkFBa0IsWUFBWSxjQUFjO0FBQUEsSUFDaEUsUUFBUSxDQUFDLFFBQVEsSUFBSSxjQUFjLFlBQVk7QUFBQSxJQUMvQyxXQUFXLENBQUMsQ0FBQyxRQUFRLElBQUk7QUFBQSxJQUN6QixhQUFhO0FBQUEsRUFDZDtBQUFBO0FBQUEsRUFFQSxNQUFNO0FBQUEsSUFDTCxTQUFTLENBQUMsNkRBQTZEO0FBQUEsRUFDeEU7QUFDRCxDQUFDOyIsCiAgIm5hbWVzIjogW10KfQo=
