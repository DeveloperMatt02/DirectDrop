---
version: 1
slug: "app-main-src-app-vue"
primary_target: "app/main/src/App.vue"
related_targets: []
---

# Surface brief: app/main/src/App.vue (receiver/sender console)

Scope: Tauri `app/main/` main window. Mode: Operate.
Audience/job: Linux laptop owner moving files with own Android phone in same room. Task: stay receivable, send files fast, consent with PIN in 60s, track queue. Frequency: few times daily, seconds per run. Constraints: CLI parity (PIN, 60s auto-decline, idempotent upserts), in-process bridge only, stores/types/src-tauri untouched.

Direction: user-pinned blueprint schematic (replaces the earlier quiet console; user pin beats the roll c788c598). Dark slate-950 dotted-grid schematic: laptop ↔ phone flow diagram with animated dashed link + particles while live, sketch-outline consent card with giant orange PIN and depleting ring, block-grid progress with live speed/bytes/ETA, dashed send dropzone, 1px device cards with LAN badges and SEND buttons, arc scan radar. Zero shadows, mono-first, reduced-motion fallbacks.

## Direction contract

THESIS: The transfer drawn as its own wiring diagram. The window owns one job — receive fast, send fast, consent once — and proves the link visually instead of listing it. Telemetry is the decoration.

OWN-WORLD: Slate-950 dotted ground, 1px hairline panels, sky link accent plus orange consent signal, semantic green/amber/red for state. Mono-first labels; tabular figures for PIN, bytes, speed, countdowns, ETA. The link animates only while live.

STORY: Owner glances at the header and knows visibility in one second. The flow diagram shows laptop, phone, and the live link; consent interrupts centered on the link with PIN and ring; progress shows blocks, speed, and ETA. Send pane stages files, device cards fire sends, the log keeps history.

FIRST VIEWPORT: 56px header (Linux mark, wordmark, live dot, visibility blueprint buttons with Temporary timer, settings). Below: full-width flow diagram (laptop | dashed link + card | phone), then send/devices grid with scan radar, then handshake/completed log.

FORM: User-pinned blueprint schematic from brief; roll c788c598 (assigned 4, code-led Operate) set aside by pin. Seed key c788c598.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance
