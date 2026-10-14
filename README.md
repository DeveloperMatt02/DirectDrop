# quickshare-direct — with Desktop GUI (fork)

**Quick Share (Nearby Share) for Linux that doesn't need both devices on the same Wi-Fi — now with a reactivated desktop GUI.**

This is a fork of upstream `quickshare-direct`. All of the upstream headless receiver / sender stack is preserved unchanged (BLE advertisement/GATT first contact, Wi-Fi Direct GO/client upgrade, LAN fallback, polkit helpers, headless CLI daemon). What this fork adds is a redesigned Tauri desktop GUI in `app/main/` with full behavioral parity to the CLI.

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
| Packages: Debian/Ubuntu, Fedora, Arch, generic (x86_64, aarch64) | one-line install, see below |

---

## What this fork introduces: reactivated & redesigned Desktop GUI (`app/main/`)

The upstream repository keeps rQuickShare's old Tauri app under `app/` but does not build, ship, or test it (its sending path is LAN/mDNS-only, without the Bluetooth / Wi-Fi Direct path). This fork reactivates that idea as a new, purpose-built GUI in `app/main/` that drives the **same** `core_lib` (`rqs_lib`) engine the CLI uses — no parallel protocol implementation.

### Stack and look

- **Tauri 2.2, Vue 3.5, Pinia 2.x, Tailwind CSS 3.4.** Frontend entry is `app/main/src/App.vue`; state lives in Pinia stores (`src/stores/session.ts`, `devices.ts`, `transfers.ts`).
- **Technical blueprint aesthetic:** dark operate-mode console, tabular monospace metrics, live network-flow diagram between laptop and phone (`FlowDiagram.vue` / `FlowHero.vue`), 1px hairlines, no marketing bloat. Layout: `StatusHeader` → `FlowDiagram` → `SendPane` + `DeviceBoard` → `TransferQueue`, with a thin receiver-details footer (download path, port).

### Non-invasive in-process Rust bridge

The GUI runs `rqs_lib` in-process. The boundary is exactly two new files plus a versioned contract — **zero changes to the core protocol engine**:

- `app/main/src-tauri/src/dto.rs` — the only wire format. Core domain types in `core_lib` deliberately do not derive `Serialize`/`Deserialize`; every Tauri `invoke`/`emit` payload goes through these serde DTOs with explicit `From` mappings. `DTO_VERSION = 1`, enforced by the `dto_version_matches_ts` Rust test against the TypeScript mirror.
- `app/main/src-tauri/src/bridge.rs` — subscribes to the core `broadcast`/`watch` channels (`message_sender`, `dch_sender`, `visibility_sender`, `ble_receiver`) and re-emits throttled/deduped Tauri events. Progress events are throttled (emit on state change, or ≥256 KiB delta, or ≥500 ms; terminal states always emitted). `Lagged` broadcast overflows are skipped with a debug log, never a crash — same policy as the CLI.
- `app/main/src/types/qsd.ts` — strict TypeScript mirror of `dto.rs` (event names, `VisibilityDto`, `TransferStateDto` with all 18 core states, `PayloadKindDto`, `TransferDto`, `EndpointDto`, `SendPayloadDto`, `ConsentDto`, `SnapshotDto`, error codes). Includes the `humanSize` / `describeTransfer` (port of the CLI `describe()`), `baseName`, and `validateSendFiles` helpers, plus `TERMINAL_STATES` / `ACTIVE_PROGRESS_STATES` / `HANDSHAKE_STATES` sets.

Tauri commands exposed (`src-tauri/src/cmds/`, wired in `main.rs`): `qsd_snapshot`, `qsd_set_visibility`, `qsd_set_download_path`, `qsd_set_device_name`, `qsd_start_discovery`, `qsd_stop_discovery`, `qsd_consent`, `qsd_send_lan`, `qsd_get_hostname` (plus legacy `get_hostname` / `change_download_path` shims). Events backend → frontend: `qsd://transfer`, `qsd://endpoints`, `qsd://visibility`, `qsd://ble-presence`.

### Full CLI behavioral parity

- **Instant UKEY2 PIN verification card** (`ConsentCard.vue`): sender name, device type, PIN, payload summary, and file list appear as soon as the core reaches `WaitingForUserConsent`. A **60-second countdown** ring (`QSD_CONSENT_TIMEOUT_MS`, honoring the `QSD_CONSENT_TIMEOUT` env override read in `dto.rs`) expires the card; expiry triggers the same **auto-decline** (`ConsentDecline` on the channel) the CLI performs. Accept/Decline/Cancel map 1:1 to core `TransferAction`s via `ConsentDto::into_channel_message`.
- **3-state visibility selector** (`StatusHeader.vue` → `qsd_set_visibility`): `Visible`, `Temporarily` (60 s, with amber pulse dot and remaining-time display), `Hidden` (`Invisible`). The Temporarily→Invisible timer lives in core `mdns.rs`; the GUI only reflects the `qsd://visibility` event and persists it via the Tauri store. No GUI-side timer reimplementation.
- **Passive BLE presence alert banner:** core BLE listener sightings arrive as throttled (120 s) `qsd://ble-presence` events. When hidden (`Invisible`), the bridge additionally fires a system notification prompting a temporary visibility window; the frontend surfaces an info toast ("A phone nearby is sharing — check visibility"). Honors `QSD_BLE_SCAN` / `QSD_NO_BLE_SCAN` core semantics.
- **Inline text/URL transfer handling without forced disk writes** (`TransferCard.vue`): `Text`, `Url`, and `WiFi` payloads render inline with preview plus clipboard copy (Tauri clipboard-manager plugin); no file is written to disk to display them. File payloads show RX file count, names, and byte-accurate progress (`ack_bytes` / `total_bytes`).
- **Outbound LAN/mDNS file sending with regular-file validation** (`SendPane.vue` + `qsd_send_lan`): stage files via dialog or drag-and-drop, pick a discovered mDNS endpoint (`EndpointDto`, `addr = ip:port`), and send through the core `sender_file` channel as `SendInfo { ob: Files }`. Validation matches the CLI: empty selections and directories rejected with `VALIDATION` errors (`{f}: not a regular file (folders aren't supported)`); the frontend fails fast on obvious cases before invoking the backend.

### Running the GUI from source

```bash
cd app/main
pnpm install
pnpm tauri dev     # live GUI against local core_lib (path dep: ../../core_lib)
pnpm ts-check       # vue-tsc --noEmit contract check
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

Releases: CI (`.github/workflows/packages.yml`) builds and install-tests the packages on every
push; bumping `VERSION` on `main` publishes the release `qsd-v<VERSION>`.

Repository layout: `core_lib` (library + the `quickshare-direct` binary) and `packaging/`; that's
all the packages, the NixOS module and CI build. `app/main/` is this fork's reactivated desktop
GUI (Tauri + Vue): it wraps `core_lib` in-process via `src-tauri/src/dto.rs` + `bridge.rs` and the
`src/types/qsd.ts` contract, without touching core. Other upstream leftovers (`snap/`, `BUILD.md`,
`CHANGELOG.md`, release-please, the `build`/`lint` workflows) still refer to upstream's `master`.

Development (headless): `nix develop`, then in `core_lib`: `cargo run --bin quickshare-direct`
(env: `QSD_DIR`, `QSD_NAME`, `QSD_PORT`, `QSD_BWU_PORT`, `QSD_BWU=lan|hotspot`, `RUST_LOG`).
Development (GUI): see "Running the GUI from source" above.
