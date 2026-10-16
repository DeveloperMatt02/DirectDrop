<div align="center">
  <img src="https://img.shields.io/badge/English-EN-blue?style=flat-square" alt="English" />
  <a href="README.it.md"><img src="https://img.shields.io/badge/Italiano-IT-555?style=flat-square" alt="Italiano" /></a>
  <a href="README.es.md"><img src="https://img.shields.io/badge/Español-ES-555?style=flat-square" alt="Español" /></a>
  <a href="README.fr.md"><img src="https://img.shields.io/badge/Français-FR-555?style=flat-square" alt="Français" /></a>
  <a href="README.de.md"><img src="https://img.shields.io/badge/Deutsch-DE-555?style=flat-square" alt="Deutsch" /></a>
  <a href="README.pt.md"><img src="https://img.shields.io/badge/Português-PT-555?style=flat-square" alt="Português" /></a>
  <a href="README.ru.md"><img src="https://img.shields.io/badge/Русский-RU-555?style=flat-square" alt="Русский" /></a>
  <a href="README.zh.md"><img src="https://img.shields.io/badge/中文-ZH-555?style=flat-square" alt="中文" /></a>
  <a href="README.ja.md"><img src="https://img.shields.io/badge/日本語-JA-555?style=flat-square" alt="日本語" /></a>
</div>

<div align="center">
  <img src="app/main/src-tauri/icons/128x128.png" width="128" alt="DirectDrop icon" />
  <h1>DirectDrop</h1>
  <p><strong>Quick Share (Nearby Share) for Linux that doesn't need both devices on the same Wi-Fi — now with a desktop GUI.</strong></p>

  <a href="https://github.com/DeveloperMatt02/DirectDrop/releases/latest"><img src="https://img.shields.io/github/v/release/DeveloperMatt02/DirectDrop?style=flat-square" alt="latest release" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=flat-square" alt="stars" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/releases/latest"><img src="https://img.shields.io/github/downloads/DeveloperMatt02/DirectDrop/total?style=flat-square" alt="downloads" /></a>
  <img src="https://img.shields.io/badge/License-GPLv3-blue?style=flat-square" alt="license GPL-3.0" />
  <img src="https://img.shields.io/badge/platform-Linux-blue?style=flat-square" alt="platform Linux" />
  <img src="https://img.shields.io/badge/Tauri-2.2-24C8DB?style=flat-square&logo=tauri" alt="Tauri 2.2" />
  <img src="https://img.shields.io/badge/Vue-3.5-4FC08D?style=flat-square&logo=vue.js" alt="Vue 3.5" />
  <a href="https://github.com/DeveloperMatt02/DirectDrop/actions/workflows/release.yml"><img src="https://github.com/DeveloperMatt02/DirectDrop/actions/workflows/release.yml/badge.svg" alt="release CI" /></a>

  <br />
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=social" alt="star this repo" /></a>

  <br /><br />
  <a href="https://github.com/DeveloperMatt02/DirectDrop/releases/latest"><img src="https://img.shields.io/badge/⬇_Download_Latest_Release-38bdf8?style=for-the-badge" alt="download latest release" /></a>
  <a href="#quick-install-terminal"><img src="https://img.shields.io/badge/🐧_Quick_Install-22c55e?style=for-the-badge" alt="quick install" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/issues/new"><img src="https://img.shields.io/badge/🐛_Report_Bug-f59e0b?style=for-the-badge" alt="report bug" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/badge/⭐_Star-facc15?style=for-the-badge&logo=github" alt="star" /></a>
</div>

> A fork of upstream `quickshare-direct`. The headless receiver / sender stack is preserved unchanged (BLE advertisement/GATT first contact, Wi-Fi Direct GO/client upgrade, LAN fallback, polkit helpers, headless CLI daemon). What this fork adds is a redesigned Tauri desktop GUI in `app/main/` with full behavioral parity to the CLI.

## Quick Install (Terminal)

Universal one-liner — detects the distro and installs the matching bundle from the
latest `DeveloperMatt02/DirectDrop` release (`.rpm` on Fedora/RHEL, `.deb` on
Debian/Ubuntu, `.AppImage` into `~/.local/bin` everywhere else):

```bash
curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash
```

The installer asks which edition to install (GUI if available, else daemon).
Unattended: `DIRECTDROP_EDITION=cli curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash`,
or `curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash -s -- --cli`
(`--gui` GUI-only, `--auto` explicit default).

Direct commands per distro (same source, no helper script):

```bash
# Fedora / RHEL (.rpm)
URL=$(curl -fsSL https://api.github.com/repos/DeveloperMatt02/DirectDrop/releases/latest \
  | grep -o 'https://[^"]*\.rpm' | grep x86_64 | head -n 1)  # x86_64 (aarch64 on ARM)
sudo dnf install -y "$URL"

# Debian / Ubuntu (.deb)
URL=$(curl -fsSL https://api.github.com/repos/DeveloperMatt02/DirectDrop/releases/latest \
  | grep -o 'https://[^"]*\.deb' | grep amd64 | head -n 1)  # amd64 (arm64 on ARM)
curl -fL -o /tmp/directdrop.deb "$URL"
sudo apt-get install -y /tmp/directdrop.deb
```

Or pick the asset by hand from the [latest release](https://github.com/DeveloperMatt02/DirectDrop/releases/latest).

## GUI + headless daemon: pick one

The GUI bundles (`.deb` / `.rpm` / `.AppImage`) embed their own receiver; the
`quickshare-direct` daemon package is the same receiver headless. Running both
at once advertises the laptop twice under the same device name and makes them
fight over the per-user Wi-Fi helpers (`quickshare-ap@USER`,
`quickshare-join@USER`), so `get-directdrop.sh` disables the daemon
(`systemctl --user disable --now quickshare-direct`) whenever it installs a
GUI bundle. The `.deb`/`.rpm` GUI also adds DirectDrop to the app menu (search
"DirectDrop"); the `.AppImage` doesn't until integrated (Gear Lever,
appimaged, or a manual `.desktop` file). To go back to headless: quit the GUI
and run `systemctl --user enable --now quickshare-direct`.

Upstream quick-start still applies:

1. **Discovery + first contact over Bluetooth LE** (works with the phone off Wi-Fi).
2. **Encrypted handshake** (UKEY2, PIN shown on both sides) over that Bluetooth link.
3. **Upgrade to Wi-Fi speed**: the laptop creates a **Wi-Fi Direct group** next to its normal Wi-Fi connection (which keeps working) and hands the credentials to the phone; or, if the phone is on the same Wi-Fi, the transfer moves to the local network. A plain hotspot is the fallback.
4. **Desktop notifications**: Accept/Decline (with the PIN), then "Open folder".

| | Status |
|---|---|
| Receive from Android (tested upstream: Pixel 10, Android 16) | works |
| Phone off Wi-Fi → Wi-Fi Direct (75 MB, ~5 MB/s) | works |
| Phone on the same Wi-Fi → LAN | works |
| Bluetooth Classic first contact | experimental (`QSD_BT_CLASSIC=1`) |
| Send to Android → phone's Wi-Fi Direct (300 MB, ~36 MB/s) | works (`quickshare-direct send`, file manager, GUI send pane) |
| Send over Bluetooth only (no shared Wi-Fi) to a OnePlus Nord4 | not working yet: the phone asks to pair, then doesn't answer; use the same Wi-Fi (LAN) |
| Packages: Debian/Ubuntu, Fedora, Arch, generic (x86_64, aarch64) | one-line install, see below |

---

## What this fork introduces: reactivated & redesigned Desktop GUI (`app/main/`)

The upstream repository keeps rQuickShare's old Tauri app under `app/` but does not build, ship, or test it (its sending path is LAN/mDNS-only, without the Bluetooth / Wi-Fi Direct path). This fork reactivates that idea as a new, purpose-built GUI in `app/main/` that drives the **same** `core_lib` (`rqs_lib`) engine the CLI uses — no parallel protocol implementation.

### Stack and look

- **Tauri 2.2, Vue 3.5, Pinia 2.x, Tailwind CSS 3.4.** Frontend entry is `app/main/src/App.vue`; state lives in Pinia stores (`src/stores/session.ts`, `devices.ts`, `transfers.ts`, `transferGroups.ts`).
- **Sidebar + activity layout:** the left sidebar (`StatusHeader.vue`) holds the radar (`FlowHero.vue`, with waves only while the PC is discoverable), the PC name, the **Visible / Temporarily / Hidden** selector, and the download folder and port. The main area is **Activity**: the incoming-request (consent) card on top, the compact send panel (`SendPane.vue` + `DeviceBoard.vue`), then the transfer list (`TransferQueue.vue`) grouped into **NOW / TODAY / EARLIER**, newest first. UI strings are uppercase English.
- **Light and dark theme, automatic:** follows the desktop setting. On Linux it reads `org.freedesktop.appearance color-scheme` from the settings portal (`ashpd`, command `qsd_system_theme`, event `qsd://system-theme`), falling back to Tauri's `theme()` and then to the `prefers-color-scheme` media query. Colors are `--qsd-*` tokens.
- **Transfer rows:** direction, a badge for the link in use (Bluetooth LE, BLE L2CAP, Wi-Fi LAN, Wi-Fi Direct, hotspot), a one-time toast per transfer plus an inline notice when the transfer falls back to a slower link, live speed while running and the average speed (AVG) once finished. **OPEN FILE** / **OPEN FOLDER** appear only on received transfers (one file: both buttons; several files: the folder only).
- **More reliable receiver (core):** the mDNS and BLE announcement tasks restart themselves with a backoff, so the PC no longer drops off nearby devices after an error; after a failed or cancelled receive the partial files are deleted and the completed ones are kept.

### Non-invasive in-process Rust bridge

The GUI runs `rqs_lib` in-process. The boundary is exactly two new files plus a versioned contract — **zero changes to the core protocol engine**:

- `app/main/src-tauri/src/dto.rs` — the only wire format. Core domain types in `core_lib` deliberately do not derive `Serialize`/`Deserialize`; every Tauri `invoke`/`emit` payload goes through these serde DTOs with explicit `From` mappings. `DTO_VERSION = 1`, enforced by the `dto_version_matches_ts` Rust test against the TypeScript mirror.
- `app/main/src-tauri/src/bridge.rs` — subscribes to the core `broadcast`/`watch` channels (`message_sender`, `dch_sender`, `visibility_sender`, `ble_receiver`) and re-emits throttled/deduped Tauri events. Progress events are throttled (emit on state change, or ≥256 KiB delta, or ≥500 ms; terminal states always emitted). `Lagged` broadcast overflows are skipped with a debug log, never a crash — same policy as the CLI.
- `app/main/src/types/qsd.ts` — strict TypeScript mirror of `dto.rs` (event names, `VisibilityDto`, `TransferStateDto` with all 18 core states, `PayloadKindDto`, `TransferDto`, `EndpointDto`, `SendPayloadDto`, `ConsentDto`, `SnapshotDto`, error codes). Includes the `humanSize` / `describeTransfer` (port of the CLI `describe()`), `baseName`, and `validateSendFiles` helpers, plus `TERMINAL_STATES` / `ACTIVE_PROGRESS_STATES` / `HANDSHAKE_STATES` sets.

Tauri commands exposed (`src-tauri/src/cmds/`, wired in `main.rs`): `qsd_snapshot`, `qsd_set_visibility`, `qsd_set_download_path`, `qsd_set_device_name`, `qsd_start_discovery`, `qsd_stop_discovery`, `qsd_consent`, `qsd_send_lan`, `qsd_scan_ble`, `qsd_send_ble`, `qsd_cancel_send`, `qsd_open_path`, `qsd_system_theme`, `qsd_get_hostname` (plus legacy `get_hostname` / `change_download_path` shims). Events backend → frontend: `qsd://transfer`, `qsd://endpoints`, `qsd://visibility`, `qsd://ble-presence`, `qsd://system-theme`.

### Full CLI behavioral parity

- **Instant UKEY2 PIN verification card** (`ConsentCard.vue`): sender name, device type, PIN, payload summary, and file list appear as soon as the core reaches `WaitingForUserConsent`. A **60-second countdown** ring (`QSD_CONSENT_TIMEOUT_MS`, honoring the `QSD_CONSENT_TIMEOUT` env override read in `dto.rs`) expires the card; expiry triggers the same **auto-decline** (`ConsentDecline` on the channel) the CLI performs. Accept/Decline/Cancel map 1:1 to core `TransferAction`s via `ConsentDto::into_channel_message`.
- **3-state visibility selector** (sidebar, `StatusHeader.vue` → `qsd_set_visibility`): `Visible`, `Temporarily` (60 s, with the remaining time shown as `VISIBLE FOR 0:xx`), `Hidden` (`Invisible`). The Temporarily→Invisible timer lives in core `mdns.rs`; the GUI only reflects the `qsd://visibility` event and persists it via the Tauri store. No GUI-side timer reimplementation.
- **Passive BLE presence alert banner:** core BLE listener sightings arrive as throttled (120 s) `qsd://ble-presence` events. When hidden (`Invisible`), the bridge additionally fires a system notification prompting a temporary visibility window; the frontend surfaces an info toast ("A phone nearby is sharing — check visibility"). Honors `QSD_BLE_SCAN` / `QSD_NO_BLE_SCAN` core semantics.
- **Inline text/URL transfer handling without forced disk writes** (`TransferCard.vue`): `Text`, `Url`, and `WiFi` payloads are listed in the row (`TEXT`, `LINK`, `WI-FI · SSID`) with COPY / COPY LINK (Tauri clipboard-manager plugin) and OPEN for links; no file is written to disk to display them. File payloads show RX file count, names, and byte-accurate progress (`ack_bytes` / `total_bytes`).
- **Outbound LAN/mDNS file sending with regular-file validation** (`SendPane.vue` + `qsd_send_lan`): stage files via dialog or drag-and-drop, pick a discovered mDNS endpoint (`EndpointDto`, `addr = ip:port`), and send through the core `sender_file` channel as `SendInfo { ob: Files }`. Validation matches the CLI: empty selections and directories rejected with `VALIDATION` errors (`{f}: not a regular file (folders aren't supported)`); the frontend fails fast on obvious cases before invoking the backend.

### Running the GUI from source

```bash
cd app/main
pnpm install
pnpm tauri dev     # live GUI against local core_lib (path dep: ../../core_lib)
pnpm ts-check       # vue-tsc --noEmit contract check
pnpm lint           # eslint
pnpm test           # vitest suite (tests/)
pnpm build          # production Tauri bundle
```

Rust side: `cd app/main/src-tauri && cargo check` / `cargo test` (includes the DTO wire-format tests). System prerequisites are the same as the daemon's (BlueZ, NetworkManager, libdbus headers, protoc, cargo ≥ 1.85) plus Node/pnpm and Tauri 2.x Linux dependencies.

---

## Architecture & Genealogy / Credits

Layered view of this fork:

```
Android phone ──BLE adv / GATT──▶ core_lib (rqs_lib) ◀──Tauri bridge──▶ Vue GUI
                                        │  (unchanged engine)
                                        ▼
                               Wi-Fi Direct GO / client,
                               LAN, hotspot fallback,
                               polkit Wi-Fi helpers
```

- **Upstream `quickshare-direct` author(s):** the core off-network transport this fork depends on — BLE advertisement monitoring and GATT server for discovery/first contact, Wi-Fi Direct group-owner / client upgrade alongside the normal Wi-Fi connection (with single-channel fallback handling), the `quickshare-ap@USER` / `quickshare-join@USER` on-demand root helpers with their polkit rule, and the headless `quickshare-direct` CLI daemon with desktop notifications. All protocol, Bluetooth, and Wi-Fi engineering credit belongs here; this fork modifies none of it.
- **`rQuickShare` by Martichou (and contributors such as nozwock and martinalderson):** the foundational Rust/Tauri implementation of the Nearby Share / Quick Share protocol on Linux that both upstream and this fork build on — including the `rqs_lib` core, the original Tauri app structure, and the Bluetooth discovery work carried forward into `core_lib`.
- **Google Nearby (Apache-2.0):** the protocol specification and protobuf definitions that define the UKEY2 handshake, connection, introduction, and payload frames. License of this repository remains GPL-3.0, like rQuickShare.

---

### Install (daemon + CLI packages)

Debian 12+ / Ubuntu 22.04+ (and Mint, Pop!_OS, ...), Fedora, Arch (and Manjaro, EndeavourOS, ...),
others via a generic build; x86_64 and aarch64:

```
curl -fsSL https://raw.githubusercontent.com/papshmeare/quickshare-direct/main/install.sh | sh
```

It installs the latest [release](https://github.com/papshmeare/quickshare-direct/releases) with
your package manager (`.deb`, `.rpm`, `.pkg.tar.zst`; dependencies come from your distribution),
opens the Quick Share ports if firewalld or ufw is active, and starts the receiver. Run the same
line again to update; `curl -fsSL .../install.sh | sh -s -- --uninstall` removes it. Prefer to
see what happens first? Download `install.sh` and read it, or install the package from the
release page by hand (`sudo apt install ./quickshare-direct_amd64.deb`,
`sudo dnf install ./quickshare-direct.x86_64.rpm`,
`sudo pacman -U quickshare-direct-x86_64.pkg.tar.zst`).

What gets installed:
- `quickshare-direct`: the receiver (a user service in every graphical session, with
  notifications) and the `send` / `devices` commands;
- two small root helpers, started on demand through systemd (`quickshare-ap@USER` creates the
  direct Wi-Fi link when receiving, `quickshare-join@USER` joins the phone's when sending), and a
  polkit rule (`packaging/linux/polkit/50-quickshare-direct.rules`) that lets an active local
  user start/stop only their own instances (`start`/`stop`/`restart` on
  `quickshare-ap@USER.service` / `quickshare-join@USER.service`);
- a NetworkManager drop-in that leaves the temporary link interfaces alone, a unit that keeps
  Bluetooth connectable, firewalld/ufw service definitions, and file manager entries.

Needs: systemd, BlueZ, NetworkManager with wpa_supplicant (the default on these distributions;
not iwd), and a Wi-Fi card that can run a second interface next to the normal connection (most
Intel/MediaTek/Qualcomm cards). Notifications need a daemon that shows action buttons (GNOME,
KDE, swaync, mako, ...). Ubuntu 22.04's older polkit can't let users start the helpers, so there
only Bluetooth and same-network transfers work. Set your Wi-Fi regulatory domain
(`sudo iw reg set XX`): with the default world domain Linux won't start a group owner on 5 GHz.
Settings: `~/.config/quickshare-direct/env` (`QSD_NAME=...`, `QSD_DIR=...`) for the receiver,
`/etc/default/quickshare-direct` for the helpers. Many Wi-Fi chips can't host the direct link on a
second channel next to your normal connection, which matters when your router uses a channel a
laptop may not host on (radar/DFS channels such as 100): then the laptop leaves your Wi-Fi network
for the few seconds of the transfer and reconnects afterwards (`QS_SINGLE_CHANNEL=auto`; `always`
for full speed when sending too, `never` to keep the connection and fall back to Bluetooth).

From source: `make && sudo make install` (needs cargo ≥ 1.85, protoc, libdbus headers), then
`systemctl --user enable --now quickshare-direct`.

### NixOS

```nix
# flake.nix inputs
quickshare-direct = { url = "github:papshmeare/quickshare-direct"; inputs.nixpkgs.follows = "nixpkgs"; };

# a module
{ inputs, ... }: {
  imports = [ inputs.quickshare-direct.nixosModules.default ];
  services.quickshare-direct = {
    enable = true;
    user = "alice";               # whose desktop session runs it
    wifiInterface = "wlp2s0";     # ip -br link
  };
}
```
This sets up the receiver (user service in the graphical session), the root helper that creates
the direct Wi-Fi link on demand (`quickshare-ap.service`, startable by that user via polkit),
the helper that joins a phone's Wi-Fi Direct group when sending (`quickshare-join.service`),
firewall ports, Bluetooth "connectable", and keeps NetworkManager off the link interfaces.
Set your Wi-Fi regulatory domain (`iw reg set XX`): with the default world domain Linux won't
start an access point / group owner on 5 GHz.

### Sending (CLI)

On the phone, open Quick Share and tap **Receive** (or make it visible to everyone), then:

```
quickshare-direct send photo.jpg notes.pdf      # to the one phone in receive mode nearby
quickshare-direct send --to Pixel video.mp4     # pick by name when several are around
quickshare-direct devices                       # list phones in receive mode
```
It finds the phone over Bluetooth, shows the PIN, and once you accept on the phone the files go
over the phone's Wi-Fi Direct group (joined on a second interface, your Wi-Fi stays connected).
In the file manager: Thunar's **Send To → Phone (Quick Share)** (any file), or Open With →
"Send with Quick Share" (common file types); it reports through notifications. The GUI
`SendPane` performs the same LAN/mDNS send through `qsd_send_lan` with identical PIN and
regular-file semantics.

### Environment variables (receiver, sender, helpers)

Core behavior is configured via environment (see `core_lib/src/bin/quickshare-direct.rs`,
`core_lib/src/hdl/*.rs`):

| Variable | Default | Effect |
|---|---|---|
| `QSD_DIR` | XDG Downloads, else `~/Downloads` | download folder |
| `QSD_NAME` | hostname | device name shown on the phone |
| `QSD_PORT` | random | TCP port for incoming transfers |
| `QSD_BWU_PORT` | random | TCP port for the Wi-Fi upgrade listener |
| `QSD_CONSENT_TIMEOUT` | `60` (s) | seconds to answer Accept/Decline before auto-decline; also drives the GUI countdown |
| `QSD_BWU` | `auto` | inbound upgrade preference: `lan` / `hotspot` forces one |
| `QSD_BLE_SCAN` | passive BlueZ advertisement monitor | `discovery` = permanent LE discovery (may block bonded LE mice/keyboards reconnecting); `off` disables the listener |
| `QSD_NO_BLE_SCAN` | unset | when set, disables sender BLE detection entirely |
| `QSD_BT_CLASSIC` | unset | when set (`=1`), enables experimental Bluetooth Classic first contact |
| `QSD_BLE_L2CAP` / `QSD_L2CAP_CHUNK` | on / default | send-path BLE L2CAP tuning (`=0` forces weave socket) |
| `QSD_WEAVE_ATT` / `QSD_WEAVE_PACKET` | on / MTU default | GATT/weave transport tuning for experiments |
| `QSD_SEND_MEDIUMS` | wifi_lan-first default | comma-separated outbound medium preference (`wifi_lan`, ...) |
| `QS_SINGLE_CHANNEL` | `auto` | `always` = full speed when sending (may drop station); `never` = keep connection, fall back to Bluetooth |
| `QSD_AP_IFACE` / `QSD_STA_IFACE` | `ap0` / auto | link interface names for the AP helper |
| `QSD_AP_UNIT` / `QSD_JOIN_UNIT` | `quickshare-ap` / `quickshare-join` | systemd unit names for the root helpers |
| `QSD_SYSTEMCTL` | `systemctl` | helper spawn command override |
| `QSD_AP_CREDENTIALS` | `/run/quickshare/credentials` | credential drop path for the AP helper |
| `QSD_JOIN_REQUEST` / `QSD_JOIN_RESULT` | `/run/quickshare-join/...` | request/result paths for the join helper |
| `RUST_LOG` | `info` (`warn` for send/devices) | log filter |

Receiver file: `~/.config/quickshare-direct/env` (`QSD_NAME`, `QSD_DIR`, ...). Helper file:
`/etc/default/quickshare-direct`.

### How it works / research notes

[docs/DEV_NOTES.md](docs/DEV_NOTES.md) (findings, phone logs, protocol details) and
[docs/BLE_RECEIVER_DISCOVERY.md](docs/BLE_RECEIVER_DISCOVERY.md) (the BLE receiver path).

Releases (this fork): pushing a `v*.*.*` tag runs [`.github/workflows/release.yml`](.github/workflows/release.yml), which builds the Tauri GUI bundles (`.deb`, `.rpm`, `.AppImage`) and the `directdrop-cli-linux-x86_64.tar.gz` daemon tarball and publishes them to the GitHub Release — installable via `get-directdrop.sh` above. Upstream daemon packages keep their own flow: CI (`.github/workflows/packages.yml`) builds and install-tests the packages on every push; bumping `VERSION` on `main` publishes the release `qsd-v<VERSION>`.

Repository layout: `core_lib` (library + the `quickshare-direct` binary) and `packaging/`; that's
all the packages, the NixOS module and CI build. `app/main/` is this fork's reactivated desktop
GUI (Tauri + Vue): it wraps `core_lib` in-process via `src-tauri/src/dto.rs` + `bridge.rs` and the
`src/types/qsd.ts` contract, without touching core. Other upstream leftovers (`snap/`, `BUILD.md`,
`CHANGELOG.md`, release-please, the `build`/`lint` workflows) still refer to upstream's `master`.

Development (headless): `nix develop`, then in `core_lib`: `cargo run --bin quickshare-direct`
(env: `QSD_DIR`, `QSD_NAME`, `QSD_PORT`, `QSD_BWU_PORT`, `QSD_BWU=lan|hotspot`, `RUST_LOG`).
Development (GUI): see "Running the GUI from source" above.

---

<div align="center">

## ⭐ Show your support

If DirectDrop gets your files to your phone without touching the router, give it a star — it helps a lot.

<a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=social" alt="star this repo" /></a>

<a href="https://star-history.com/#DeveloperMatt02/DirectDrop&Date"><img src="https://api.star-history.com/svg?repos=DeveloperMatt02/DirectDrop&type=Date" width="600" alt="star history chart" /></a>

Found a bug? [Open an issue](https://github.com/DeveloperMatt02/DirectDrop/issues/new) — pull requests welcome.

</div>
