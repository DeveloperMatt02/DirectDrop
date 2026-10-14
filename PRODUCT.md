# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Primary: Linux desktop owner (GNOME/KDE, systemd user session, BlueZ, NetworkManager with wpa_supplicant) moving files between their own Android phone (tested: Pixel 10, Android 16) and laptop in the same room. Job: get phone photos/files onto the laptop (and send files back) quickly without cable, cloud upload, or shared Wi-Fi.

Other audiences: none confirmed. macOS/legacy Tauri builds are upstream leftovers, not targets.

## Product Purpose

quickshare-direct is a Quick Share / Nearby Share receiver and sender for Linux that works with the phone off-Wi-Fi. It exists because LAN-only implementations fail when newer Pixels drop Wi-Fi during discovery. Success: prompt ~0.4 s after the PIN, transfers at Wi-Fi speed (~33–37 MB/s receive, ~36 MB/s send), files land in ~/Downloads, install via one-line package install with the receiver running as a user service.

The Tauri GUI (`app/main/`) is an opt-in desktop surface over the same core. The headless daemon + desktop notifications stay the default; the GUI must reach at least CLI parity, not replace the daemon.

## Positioning

Phone-to-phone-style transport on Linux: discovery + first contact over Bluetooth LE, encrypted UKEY2 handshake with a PIN shown on both sides, then upgrade to a laptop-hosted Wi-Fi Direct group alongside the normal Wi-Fi connection (fallbacks: shared-LAN transfer, plain hotspot). No shared network required. A neighboring product that only does mDNS + TCP on a shared LAN (upstream rQuickShare app, KDE Connect, Warpinator class) could not truthfully copy this.

## Operating Context

- Same-room phone + laptop; phone's Quick Share share-sheet / Receive screen drives the flow.
- Laptop runs `quickshare-direct` as a systemd user service in each graphical session; two small root helpers (`quickshare-ap@USER`, `quickshare-join@USER` via polkit) create/join the Wi-Fi link on demand.
- Environment: systemd, BlueZ (connectable), NetworkManager + wpa_supplicant (not iwd), Wi-Fi card capable of a second interface; regulatory domain must allow group-owner mode; firewall ports opened by the package.
- Config via `~/.config/quickshare-direct/env` (`QSD_NAME`, `QSD_DIR`, `QSD_PORT`, `QSD_BWU_PORT`, `QSD_BWU`, `QSD_CONSENT_TIMEOUT` default 60 s, `QSD_BLE_SCAN`, `QSD_BT_CLASSIC`, `RUST_LOG`).
- Notifications require a daemon with action buttons (GNOME, KDE, swaync, mako); headless flow uses Accept/Decline with PIN, "Show in folder", Open (link), Copy (text).
- Dev/test: `nix develop`, USB adb (wireless adb dies mid-test because the Pixel drops Wi-Fi during discovery), ports 46257/46258 + BLE advertising must be free (stop installed service first).

## Capabilities and Constraints

Confirmed functionality: receive files/text/URL/WiFi payloads from Android; send files to a phone in receive mode (`quickshare-direct send`, `--to NAME` pick, `devices` list, Thunar Send-To entry); device visibility (Visible/Invisible/Temporarily with 60 s auto-off in core); mDNS discovery + BLE presence hint; tray icon, single-instance, autostart, persistent settings store, update check.

Durable technical constraints (GUI work):
- Never modify `core_lib/`, `packaging/`, `Makefile`, `flake.nix`, `install.sh`, CI release flow.
- GUI must stay in-process: Tauri 2.2 + Vue 3.5 + Pinia + Tailwind + Vite 5 + Vitest + pnpm 9.7 (existing `app/main/` stack). Core exposes only in-process Rust APIs (`RQS` + tokio channels); no REST/RPC/socket/DBus. DTO layer lives in `app/main/src-tauri` (serde) + `app/main/src/types/qsd.ts`; core types never cross the wire directly.
- Must replicate CLI semantics: PIN shown with consent, 60 s consent timeout with automatic decline, idempotent per-`id` upserts (core repeats state updates), dedup, broadcast-lag skip + progress throttle, regular-files-only sends (folders rejected with validation error), FileManager1 `ShowItems` preferred for "Show in folder" with `shell.open` fallback, received text/URL via Copy/Open without saving.
- `device_name` change takes effect on new announcements (restart of service); GUI shows a hint, never auto-restarts mid-transfer.
- BLE send is phase 2 (dedicated backend task per `send_id` with cancellation); LAN send/receive first.
- Linux-only, x86_64 + aarch64 (.deb/.rpm/.pkg.tar.zst/.tar.gz).

Undecided: final GUI packaging/install story (still opt-in, `.desktop` optional); update-check UX in GUI; exact empty-state / first-run copy.

## Brand Commitments

Name: `quickshare-direct` (binary + package names `qsd-v<VERSION>`; current `VERSION` = `0.1.5`). GUI crate is `rquickshare` (historical). Voice: factual README/CLI tone, no marketing claims. Upstream credit: fork of rQuickShare (Martichou, contributors; Bluetooth work nozwock/martinalderson); protocol reference Google Nearby (Apache-2.0). License: GPL-3.0. No logo, palette, or type commitments recorded — do not invent any.

## Evidence on Hand

- `README.md` (status table, install, layout; below "Repository layout" is upstream text about `app/`).
- `CLAUDE.md` (agent ops, layout, dev loop, Pixel 10 results 2026-10-05).
- `docs/DEV_NOTES.md` (findings, phone logs, measurements, open plan), `docs/BLE_RECEIVER_DISCOVERY.md` (BLE/weave protocol).
- `GUI_ARCHITECTURE.md` (GUI reactivation plan: DTO/bridge layer, `qsd://*` events, F0–F4 phases) + `REPO_SUMMARY.md` (branch/recon state).
- Incumbent UI: `app/main/src/components/HomePage.vue` (379 lines), `app/main/src/composables/` (SideMenu, ContentStatus, SettingsModal, ItemSide, Heading), `app/main/src/vue_lib/` (invoke wrappers, stale `@martichou/core_lib` bindings), `app/main/src-tauri/src/main.rs` (currently non-compiling against core: 4-arg `RQS::new`, stale `State`/`ChannelDirection`/`ChannelMessage` shape).
- CLI reference: `core_lib/src/bin/quickshare-direct.rs` (587 lines); test harness `tools/dev/qs-*.sh`.
- Absences future work must not fabricate: no testimonials, benchmarks beyond the measured ~35 MB/s lab numbers, pricing, or shipped GUI screenshots.

## Product Principles

1. Direct link first — no shared network, no cloud round-trip, no cable for the core job.
2. Parity before polish — the GUI earns its place only by matching daemon semantics (PIN, timeout, idempotence) exactly.
3. Core untouched — all GUI adaptation lives in the Tauri bridge/DTO layer; protocol and system helpers stay as-is.
4. Quiet until needed — background receiver; interrupt only for consent and completion.
5. Linux-native humility — fit systemd/NetworkManager/BlueZ/desktop conventions instead of inventing new ones.
