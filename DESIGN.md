---
name: DirectDrop GUI
description: Dark blueprint schematic console for same-room phone-to-laptop transfer.
colors:
  ground: "#020617"
  panel: "#0f172a"
  panel-raise: "#1e293b"
  ink: "#e2e8f0"
  ink-dim: "#94a3b8"
  line: "#1e293b"
  accent: "#38bdf8"
  accent-ink: "#020617"
  signal: "#fb923c"
  ok: "#34d399"
  warn: "#fbbf24"
  danger: "#f87171"
typography:
  title:
    fontFamily: "ui-monospace, 'Cascadia Mono', 'JetBrains Mono', Menlo, Consolas, monospace"
    fontSize: "15px"
    fontWeight: 700
    lineHeight: 1.3
    letterSpacing: "0.1em"
  body:
    fontFamily: "system-ui, -apple-system, 'Segoe UI', Ubuntu, Cantarell, sans-serif"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: 1.5
  label:
    fontFamily: "system-ui, -apple-system, 'Segoe UI', Ubuntu, Cantarell, sans-serif"
    fontSize: "11px"
    fontWeight: 600
    lineHeight: 1.4
    letterSpacing: "0.06em"
  mono:
    fontFamily: "ui-monospace, 'Cascadia Mono', 'JetBrains Mono', Menlo, Consolas, monospace"
    fontSize: "13px"
    fontWeight: 500
    lineHeight: 1.4
    fontFeature: "'tnum' 1, 'zero' 1"
rounded:
  sm: "6px"
  md: "10px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "16px"
  xl: "24px"
components:
  button-primary:
    backgroundColor: "{colors.signal}"
    textColor: "{colors.accent-ink}"
    rounded: "{rounded.sm}"
    padding: "8px 16px"
  button-send:
    backgroundColor: "{colors.panel-raise}"
    textColor: "{colors.accent}"
    rounded: "{rounded.sm}"
    padding: "6px 16px"
  button-ghost:
    backgroundColor: "{colors.panel-raise}"
    textColor: "{colors.ink}"
    rounded: "{rounded.sm}"
    padding: "8px 16px"
  consent-pin:
    textColor: "{colors.ink}"
    typography: "{typography.mono}"
    size: "32px"
---

# Design System: DirectDrop GUI

## Overview

**Creative North Star: "The Bench Instrument"**

A dark blueprint schematic of the transfer itself: laptop on the left, phone on the right, a dashed animated link between them that goes live only when bytes move. Dotted-grid ground, 1px hairline panels, zero shadows, mono-first labels. One sky accent carries link/selection, one orange signal carries consent urgency; semantic green/amber/red carry state. Tabular mono makes PINs, byte counts, speeds, countdowns, and ETAs comparable at a glance. Motion is the system's own telemetry — flowing dashes, a depleting consent ring, a sweeping scan radar — never decoration for its own sake.

The window reads as a wiring diagram that works: the flow section proves the link, the send pane stages files, device cards fire sends, the log keeps history.

**Key Characteristics:**
- Dark-first adaptive console (light follows system, same seams and density).
- High-density rows over cards; tables and queues, not icon grids.
- Tabular mono for every number that must be compared or confirmed.
- One accent; state colors earn their pixels.

## Colors

Restrained schematic palette: near-black slate ground, hairline panels, one sky link accent, one orange consent signal, three state colors.

### Primary
- **Link Sky** (#38bdf8): live flow path, particles, selection, SEND buttons, focus rings, progress blocks. Animates only while a transfer is live.

### Signal
- **Consent Orange** (#fb923c): UKEY2 PIN digits, consent ring, countdown urgency, primary Accept. Reserved for the consent moment.

### Neutral
- **Ground** (#020617): app ground with a dotted grid (`radial-gradient(#1e293b 1px)` at 16px).
- **Panel** (#0f172a): flow section, send pane, device board, log rows.
- **Raised** (#1e293b): hover states, empty blocks, dropzone active.
- **Primary Ink** (#e2e8f0): headings, body, PIN digits.
- **Dim Ink** (#94a3b8): captions, addresses, timestamps.
- **Hairline Seam** (#1e293b): 1px dividers and outlines; depth comes from seams, never shadows.

### Named Rules
**The Seam-Not-Shadow Rule.** Depth is a 1px seam (#2e333d) or one tonal step up. No drop shadows, no glows, no blur-as-decoration.
**The One-Accent Rule.** Blue means actionable-or-selected. Success, warning, and danger never borrow it and never decorate inactive states.

Light mode (adaptive): ground paper (#f4f5f7), panel white (#ffffff), ink (#1a1d23), dim ink (#5b6270), seam (#e0e3ea); accent deepens to #0b5bd3 for 4.5:1 contrast. Same density, same components.

## Typography

**Display Font:** none. This surface has no display voice.
**Body Font:** system sans (`system-ui, -apple-system, 'Segoe UI', Ubuntu, Cantarell, sans-serif`) with Ubuntu/Cantarell carrying GNOME/KDE fluency.
**Label/Mono Font:** `ui-monospace, 'Cascadia Mono', 'JetBrains Mono', Menlo, Consolas, monospace` with tabular figures (`tnum`, `zero`) for PINs, byte counts, speeds, countdowns.

**Character:** one workhorse sans plus one honest mono. No pairing, no display face, no fluid sizing.

### Hierarchy
- **Title** (600, 15px, 1.3): panel headers, device names, queue section heads.
- **Body** (400, 13px, 1.5): file names, descriptions, settings rows. Prose caps at 65–75ch; data rows run denser.
- **Label** (600, 11px, 0.06em tracking, uppercase): column heads, section eyebrows-as-labels (functional table headers, never decorative kickers), status captions.
- **Mono Data** (500, 13px base, tabular): byte counts, speeds, countdowns, ports. The UKEY2 PIN sets at 56px/700 with 0.15em tracking.

### Named Rules
**The Numbers-Agree Rule.** Any two numbers the user compares (PIN to phone screen, ack/total, countdown) share the mono face and tabular figures.
**The No-Costume Rule.** Mono is for code, data, and measurement only — never for labels or decoration.

## Layout

Two regions under a 56px header: a full-width flow diagram (laptop schematic | animated dashed link + consent/progress card | phone schematic), then a send/devices grid (dashed dropzone + 1px device cards with LAN badges and SEND buttons above an arc scan radar), then a handshake/completed log. Header holds Linux mark, wordmark, live status dot, visibility blueprint buttons with Temporary countdown, and settings entry. Settings live in a right drawer (360px), never a centered modal.

Density: 8px base grid, 12px panel padding, 8px row gaps, 16–24px between sections. More space above a heading than below it. Responsive is structural: below the `lg` breakpoint (1024px) the panes stack with the transfer queue first so consent stays visible, send second; tables drop the address column before they wrap. No fluid display type.

## Elevation & Depth

Flat by construction. Panels separate with 1px seams and one tonal step; the consent card earns a 2px accent top-rule plus a countdown bar (its urgency is state, not elevation). Drawer and toasts use a single ambient shadow for overlay escape only.

**The Flat-By-Default Rule.** Surfaces are flat at rest. Shadow appears only when a layer leaves its container (drawer, toast, dropdown).

## Shapes

Small technical radii: controls 6px, panels and rows 10px. No pills, no full-round avatars, no 3rem hero corners. Borders are 1px seams; selected rows take a 1px accent outline, never a colored left bar. Progress is a 4px linear bar with square-minimal fill; no rings standing in for content.

## Components

### Buttons
- **Shape:** 6px radius, 13px/500 labels.
- **Primary:** Receiver Blue ground, near-black text, 8px 16px padding. Hover darkens 8%; focus-visible adds 2px accent ring with 2px offset.
- **Ghost:** raised-panel ground, primary ink text. Destructive stays ghost-outline until focused, then takes danger color; it sits apart from neighbors with deliberate space.
- **Disabled:** 40% opacity, no hover. Every button ships default/hover/focus/active/disabled; async actions add a loading label, never a bare spinner.

### Flow Diagram
- **Style:** laptop schematic (hostname + `:port`) left, phone schematic (name + `ip:port` + link state) right, dashed link with arrowheads and SMIL particles that run only while a transfer is live; idle renders dim and static.
- **State:** consent and active transfers render centered on the link; a mono readout names link id, state, and bytes.

### Device Rows
- **Style:** 1px outline rows, schematic phone glyph, name + mono `ip:port`, LAN badge, sky SEND button (disabled until files are staged).
- **State:** hover raises one step; a BLE-hint line reports recent presence; an arc radar sweeps below the list.

### Consent Card
- **Style:** sketch-arc 2px orange outline, 56px orange mono PIN with `aria-live`, SVG ring countdown depleting over 60s.
- **Behavior:** Accept (solid orange, black text) / Decline (1px outline) side by side; expiry fires backend decline and marks expired. Renders inside the flow diagram, never as a modal.

### Progress Card
- **Style:** 24-block grid (sky filled, seam empty), live `TX/RX` speed from ack deltas, tabular `ack/total`, ETA `m:ss`.
- **State:** skeleton handshake line while connecting; errors name the problem; completed rows keep Open/Copy/Clear.

### Transfers Queue
- **Style:** handshake + completed/terminal log rows; live consent and active transfers live in the flow diagram, not here.
- **State:** empty state points at the flow diagram; errors name the problem and the recovery.

### Inputs / Fields
- **Style:** panel-raise ground, 1px seam border, 6px radius, 13px body text.
- **Focus:** 2px accent outline. Error takes danger text plus recovery hint; disabled dims.

### Navigation
- Header-only: status dot + device name left; visibility segmented control center; discovery caption + settings button right. Active segment is accent-filled; all segments are real radio semantics.

## Do's and Don'ts

### Do:
- **Do** set every PIN, byte count, speed, and countdown in tabular mono (`tnum`), verified against the phone screen at a glance.
- **Do** keep consent in flow atop the queue with countdown and expiry parity (60s auto-decline).
- **Do** theme selection, caret, focus rings, scrollbars, and numerals from this palette.
- **Do** isolate destructive actions with empty space and focus-gating.

### Don't:
- **Don't** ship gradient text, glassmorphism, colored left-bars, hard offset shadows, or eyebrow kickers.
- **Don't** use icon-plus-heading-plus-text cards as page structure or nested cards anywhere.
- **Don't** put product UI in a display face or use mono for labels and decoration.
- **Don't** modal what a drawer or inline row can do; settings and errors stay in flow.
- **Don't** invent benchmarks, testimonials, or screenshots beyond the measured ~35 MB/s lab numbers.
