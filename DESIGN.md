---
name: quickshare-direct GUI
description: Quiet Linux workbench for phone-to-laptop Quick Share transfers.
colors:
  mint-wash: "#f0fdf4"
  mint-card: "#dcfce7"
  mint-check: "#bbf7d0"
  paper: "#ffffff"
  ink: "#000000"
  progress-track: "#dddddd"
  toast-error: "#fee2e2"
  toast-info: "#dbeafe"
  signal-error: "#f87171"
  body-ink: "#1f2937"
  hover-gray: "#6b7280"
typography:
  display:
    fontFamily: "Montserrat, sans-serif"
    fontSize: "1.5rem"
    fontWeight: 500
    lineHeight: 1.33
  headline:
    fontFamily: "Montserrat, sans-serif"
    fontSize: "1.25rem"
    fontWeight: 500
    lineHeight: 1.4
  title:
    fontFamily: "Montserrat, sans-serif"
    fontSize: "1rem"
    fontWeight: 500
    lineHeight: 1.5
  body:
    fontFamily: "Montserrat, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.5
  label:
    fontFamily: "Montserrat, sans-serif"
    fontSize: "0.75rem"
    fontWeight: 400
    lineHeight: 1.5
rounded:
  sm: "8px"
  md: "12px"
  lg: "16px"
  xl: "24px"
  scoop: "48px"
  full: "9999px"
spacing:
  sm: "12px"
  md: "16px"
  lg: "24px"
  xl: "48px"
components:
  button-default:
    backgroundColor: "transparent"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: "0 16px"
    height: "48px"
  button-default-hover:
    backgroundColor: "{colors.hover-gray}"
    textColor: "{colors.ink}"
    rounded: "{rounded.md}"
    padding: "0 16px"
    height: "48px"
  card-transfer:
    backgroundColor: "{colors.mint-card}"
    textColor: "{colors.body-ink}"
    rounded: "{rounded.xl}"
    padding: "16px"
  toast-success:
    backgroundColor: "{colors.mint-card}"
    textColor: "{colors.body-ink}"
    rounded: "{rounded.md}"
    padding: "16px"
  toast-error:
    backgroundColor: "{colors.toast-error}"
    textColor: "{colors.body-ink}"
    rounded: "{rounded.md}"
    padding: "16px"
  toast-info:
    backgroundColor: "{colors.toast-info}"
    textColor: "{colors.body-ink}"
    rounded: "{rounded.md}"
    padding: "16px"
---

# Design System: quickshare-direct GUI

## Overview

**Creative North Star: "The Quiet Workbench"**

This is an operate-mode desktop utility, not a marketing surface. The window sits idle as a background receiver and interrupts only for consent (Accept/Decline with PIN), progress, and completion (Open/Copy/Clear). The aesthetic is calm, factual, and Linux-native: a pale mint-tinted shell, a white work area with one large scooped corner, Montserrat throughout, and soft fully-rounded geometry.

Density is airy: a fixed 288px side rail, a generous 48px content pad, one transfer card per row. Nothing glows, gradients, or sells. All GUI adaptation lives in `app/main/` (Tauri 2.2 + Vue 3.5 + Tailwind); `core_lib/` protocol truth is never restyled.

**Key Characteristics:**
- Quiet until needed; consent and completion are the only interruptions.
- Flat mint-tinted layering, never marketing gloss or dark mode.
- Soft and restrained controls with press-only feedback.
- Factual README/CLI voice carried into labels and empty states.

## Colors

Calm mint-tinted utility palette: one mint family on white, ink text, pastel toast semantics.

### Primary
- **Pale Mint Wash** (#f0fdf4): App shell background (`bg-green-50` on `HomePage.vue` root). Signals idle/receiving backdrop.
- **Mint Card** (#dcfce7): Transfer cards and success toasts (`bg-green-100`). The work surface for anything actionable.
- **Mint Check** (#bbf7d0): Checked checkbox fill and card borders (`border-green-200`, `.checkbox:checked`). Confirmation accent only.

### Neutral
- **Paper** (#ffffff): Content panel, avatar discs, file tile, settings modal, drop-zone highlight.
- **Ink** (#000000): Pulsing discovery dots, progress ring stroke, icon fills, button text.
- **Body Ink** (#1f2937): Toast and card body copy (`text-gray-800`).
- **Hover Gray** (#6b7280): Button hover wash at 10% opacity (`hover:bg-gray-500/10`).
- **Progress Track** (#dddddd): Circular progress background ring.

### Named Rules (optional, powerful)
**The Mint Rarity Rule.** Mint tints mark shell, actionable cards, and success only. Body copy stays ink-on-white; never flood a screen with green.
**The Pastel Semantics Rule.** Success is Mint Card (#dcfce7), error is pale red (#fee2e2), info is pale blue (#dbeafe). Error progress strokes switch to red-400 (#f87171).

## Typography

**Display Font:** Montserrat (with sans-serif fallback)
**Body Font:** Montserrat (with sans-serif fallback)

**Character:** Geometric-humanist workhorse in weights 400/500 only; friendly but factual, never editorial orMono. Loaded at 100–900 (`main.postcss`) but only 400 body and 500 headings are used.

### Hierarchy
- **Display** (500, 1.5rem/24px, 1.33): Device hostname only (`Heading.vue` `text-2xl`).
- **Headline** (500, 1.25rem/20px, 1.4): Section state — "Ready to receive" / "Nearby devices" / "Settings".
- **Title** (500, 1rem/16px, 1.5): Transfer item name (`h4.text-base`).
- **Body** (400, 0.875rem/14px, 1.5): Card copy, file rows, button labels (`.btn` inherits). Ellipsized single-line file names (`overflow-hidden whitespace-nowrap text-ellipsis`).
- **Label** (400, 0.75rem/12px, 1.5): Explainer copy under visibility toggle, download-path line, "Visibility state" eyebrow. Not uppercased, no extra tracking.

### Named Rules (optional)
**The Two-Weight Rule.** Headings are 500, everything else is 400. No bold marketing headlines, no light display type.

## Layout

Fixed workbench grid: header row, then side rail + scooped content. Root is `w-screen h-screen select-none cursor-default`, column flex. Header is a space-between row (`px-6 py-4`): device name left, version/update + settings gear right. Body is a `flex-1 flex-row`: fixed `w-72` (288px) rail (`SideMenu.vue`, `p-3`/`p-6`) beside a flexible content column (`flex-1 bg-white rounded-tl-[3rem] p-12`, vertical scroll, `min-w-0` for ellipsis).

Rhythm is `12 / 16 / 24 / 48px` (p-3, p-4/gap-4, p-6/gap-6, p-12). Transfer cards stack full-width with `mb-4` and internal `p-4 gap-6`. Actions right-align (`flex-row justify-end gap-4`). Empty state centers: pulsing dots (`m-auto`) above a bordered drop zone (`rounded-2xl p-6`, `mt-auto`) that scales to 105% on drag hover.

Responsive: desktop fixed-width window only. No breakpoints ship in `tailwind.config.cjs` (content globs + typography plugin only). Long names clip with ellipsis; layout never wraps into multi-column.

## Elevation & Depth

Flat by default with tonal layering. Depth comes from mint-vs-white tints, not shadows.

### Shadow Vocabulary (if applicable)
- **Modal lift** (`box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25)` via `shadow-xl`): Settings modal only, over a black 25% scrim.
- **Toast lift** (`box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1)` via `shadow-md`): Top-right toast stack only.
- **None elsewhere**: Transfer cards, buttons, side rail, and drop zone are explicitly `shadow-none`/flat.

### Named Rules (optional)
**The Flat-By-Default Rule.** Surfaces are flat at rest. Shadow appears only as a response to overlay state (modal open, toast raised), never on hover.

## Shapes

Soft, friendly, heavily rounded. Buttons, toasts, and modal use gently curved edges (12px radius, `rounded-xl`). Drop zone and file tile step up to 16px (`rounded-2xl`). Transfer cards are pill-soft at 24px (`rounded-3xl`). Avatars, progress rings, and status dots are fully circular (`rounded-full` / 100px). The signature silhouette is the content scoop: the white work area rounds only its top-left corner at 48px (`rounded-tl-[3rem]`) where it meets the mint shell. Checkbox is small-soft at 8px (`--rounded-btn: 0.5rem`). Borders are 1px functional only (drop zone, checkbox at 20% ink, card `border-green-200`); no decorative outlines.

## Components

### Buttons
Soft and quiet. Shape: gently curved edges (12px radius, `rounded-xl`), fixed height (48px, `h-12 min-h-12`), horizontal padding (16px, `px-4`), inline-flex with 8px gap.
- **Primary:** There is no colored primary. Every action (Accept/Decline/Cancel/Open/Copy/Clear/Select/settings gear) uses the same `.btn`: transparent fill, ink medium-weight Montserrat, pointer cursor.
- **Hover / Focus:** Gray wash only (`hover:bg-gray-500/10`), no shadow or color shift. Focus ring suppressed on checkbox (`focus:outline-none`); card buttons carry `shadow-none`.
- **Press:** Shrinks to 95% over 150ms ease-in-out (`active:scale-95 transition duration-150`).

### Cards / Containers
- **Corner Style:** Pill-soft (24px radius, `rounded-3xl`).
- **Background:** Mint Card (#dcfce7) on every transfer state (consent, sending, finished, cancelled).
- **Shadow Strategy:** Flat, per Elevation section — no card shadow.
- **Border:** None on cards; drop zone uses 1px default border that tints green on drag (`border-green-200 bg-green-100 scale-105`).
- **Internal Padding:** Card scale (16px, `p-4`); content panel scale (48px, `p-12`); drop zone scale (24px, `p-6`).

### Inputs / Fields
- **Style:** Custom checkbox only (no text inputs ship): 24px box, 1px ink-20% border, small-soft corners (8px radius), pointer cursor.
- **Focus:** Outline removed; state is fill-only.
- **Error / Disabled:** Checked fill is Mint Check (#bbf7d0) with dark checkmark gradient; no disabled treatment ships.

### Navigation
Side rail is the nav: fixed 288px (`w-72`), two modes. Idle mode shows a "Visibility state" label (12px) above a full-width `.btn` toggle (Always visible / Hidden / Temporarily) with a chevron that rotates 180° when invisible, plus 12px explainer copy. Send mode swaps to a file-count line, a 128px white rounded tile (`w-32 h-32 rounded-2xl`) with file icon, ellipsized file list, 12px Bluetooth hint, and a text-like Cancel `.btn`. Typography inherits body/label; states are press-scale only.

### Settings Modal
Centered overlay: black 25% scrim, white card (`rounded-xl shadow-xl p-4 w-[24rem]`), header row (20px medium "Settings" + Close `.btn`). Rows are `form-control` blocks (`hover:bg-gray-500/10 rounded-xl p-3`): Start on boot / Keep running on close / Start minimized toggles (checkbox rows), plus download-folder picker with ellipsized 12px path line.

### Toasts
Top-right fixed stack (`fixed top-4 right-4 flex-col gap-4 z-50`). Shape 12px radius, 16px padding, medium shadow. Success is Mint Card (#dcfce7), error pale red (#fee2e2), info pale blue (#dbeafe); text 14px Body Ink (#1f2937).

### Discovery Pulse
Signature empty-state indicator (`.status-indicator--xl`, 72px): three concentric 18px dots — Ink core, 10%-ink middle, 40%-ink outer — running 4s infinite pulsate keyframes (`pulsate-main-infinite`, `pulsate-secondary-infinite`, `pulsate-tertiary` scale-to-3 fade). Rendered only when no items and no endpoints.

### Transfer Progress Ring
62px SVG ring (`.circular-progress`, 12px stroke, round caps): light-gray track (#dddddd), Ink foreground arc rotated -90°, dash driven by `--progress`, 0.3s linear transition. Error state recolors both rings to red-400. Center holds a 56px white disc with device-type icon (Laptop/Phone/Tablet) or finished checkmark; consent state appends a centered PIN row with lock icon.

## Do's and Don'ts

Concrete visual guardrails grounded in the incumbent implementation.

### Do:
- **Do** keep every action on the single `.btn` pattern (48px, 12px radius, gray-10% hover, 95% press scale).
- **Do** use Mint Card (#dcfce7) for transfer cards and success, pale red/blue only for error/info toasts.
- **Do** keep the content scoop (48px top-left radius) where the white panel meets the mint shell.
- **Do** ellipsize file names and paths single-line; preserve `min-w-0` so flex columns can clip.
- **Do** show the PIN with lock icon whenever `WaitingForUserConsent` or `SentIntroduction` carries a `pin_code`.

### Don't:
- **Don't** introduce a colored primary button, gradient, or dark mode — the system is flat mint-on-white by decision.
- **Don't** add card or button shadows; shadow is reserved for modal (`shadow-xl`) and toasts (`shadow-md`).
- **Don't** add text inputs, new font families, or weights beyond Montserrat 400/500 without a brief.
- **Don't** rename or recolor toast semantics; success/error/info mappings are fixed.
- **Don't** break the fixed workbench layout (288px rail + scooped content) for responsive multi-column experiments.
