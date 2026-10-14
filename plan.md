# Plan: BLE send in the GUI (parity with `quickshare-direct send`)

Goal: let the DirectDrop GUI (`app/main/`) discover offline phones/tablets over
BLE and send files to them through the phone's Wi-Fi Direct group — exactly
what `quickshare-direct send` does today — instead of LAN-only sending.

Today the GUI can only send to mDNS/LAN endpoints (`qsd_send_lan` +
`SendInfo{addr: ip:port}`); an offline device never appears in the DeviceBoard.
The CLI path (`core_lib/src/bin/quickshare-direct.rs:247-360`) already does
BLE discovery (`discover_receivers`) + `send_files_ble` with progress on the
same broadcast channel the GUI bridge forwards. This plan wires that path
into the GUI **without touching `core_lib`** (public API only).

## 0. Read first (exact entry points)

- CLI reference: `core_lib/src/bin/quickshare-direct.rs:240-360`
  (`receiver_label`, `cmd_devices`, send flow, `--to` matching, CancellationToken usage).
- Core API (DO NOT MODIFY, only call):
  - `core_lib/src/hdl/gatt_client.rs:38-160` — `BleReceiver{address, endpoint_id,
    name, bt_mac, psm}`, `discover_receiver`, `discover_receivers(adapter,
    name, timeout, settle)` (strongest-signal-first, one entry per endpoint).
  - `core_lib/src/hdl/send.rs:1-70` — `send_files_ble(adapter, receiver, files,
    send_id, sender: broadcast::Sender<ChannelMessage>, ctk)`; progress
    arrives as `ChannelMessage`s with `kind: Outbound`.
  - `core_lib/src/manager.rs:14` — `SendInfo` (LAN only, NOT for this task).
- GUI boundary (extend here):
  - `app/main/src-tauri/src/dto.rs` — `BleReceiverDto` (display-only today),
    `SendPayloadDto`, `ConsentDto`, `GuiError::validation`.
  - `app/main/src-tauri/src/bridge.rs` — transfer task already forwards
    **all** `Message::Client` to `qsd://transfer`, Outbound included.
  - `app/main/src-tauri/src/cmds/send_payload.rs` — file validation pattern
    to reuse (`is_file`, VALIDATION errors).
  - `app/main/src-tauri/src/cmds/discovery.rs` — `qsd_start/stop_discovery`
    (mDNS browse; BLE scan is separate — see §2).
  - `app/main/src/types/qsd.ts`, `app/main/src/stores/devices.ts`,
    `app/main/src/stores/transfers.ts:103-106` (Outbound already separated),
    `app/main/src/components/DeviceBoard.vue`, `SendPane.vue`.

## 1. Design decisions (locked)

1. **One-shot scan, not streaming.** New command `qsd_scan_ble{name?,
   timeout_s?} -> Vec<BleReceiverDto>` that opens its own
   `bluer::Adapter` handle (mirror how the CLI obtains it in
   `quickshare-direct.rs:251-347`), runs `discover_receivers` with
   `timeout` (default 10 s) + `settle` (default 3 s), returns display DTOs.
   No new persistent watch task; the frontend re-scans on demand.
2. **Opaque receiver tokens, no wire-format churn.** `BleReceiver` carries
   `psm`/`bt_mac`/`endpoint_id` which must NOT be re-typed by the frontend.
   Backend keeps `Mutex<HashMap<String, BleReceiver>>` (id = endpoint_id hex
   + address) and `qsd_send_ble{receiver_id, files, send_id?}` resolves the
   token server-side. `BleReceiverDto` stays display-only; **no
   `DTO_VERSION` bump**.
3. **Progress for free.** `send_files_ble` emits Outbound `ChannelMessage`s
   on `message_sender`; the existing bridge task forwards them to
   `qsd://transfer`, and `transfers.ts` already splits Outbound. Only the
   *waiting-for-phone-accept* state needs UI work (see §3): for Outbound
   there is no local Accept — the PIN is verified on the phone.
4. **Cancel.** Store the send task's `CancellationToken` per `send_id`;
   reuse the existing `ConsentDto{TransferCancel}` path if core honors it
   for outbound, else add `qsd_cancel_send{send_id}` that fires the token.
   (Check core behavior first; CLI cancels via ctk on Ctrl-C.)
5. **Prerequisites are environmental, not code:** BlueZ experimental mode
   (`/etc/bluetooth/main.conf: Experimental = true`, simultaneous
   central+peripheral for advertise+scan on one adapter), `quickshare-join`
   helper installed (daemon package, service may stay disabled), phone in
   Receive/Everyone. Surface these as UI hints + VALIDATION/GUI errors,
   don't reimplement them.

## 2. Backend work (`app/main/src-tauri/src/`)

- `cmds/scan_ble.rs` (new): `qsd_scan_ble` — validate `timeout_s` range
  (e.g. 3–30 s); open adapter; `discover_receivers(adapter, name, timeout,
  settle)`; insert results into the token map (cap size, e.g. 32, evict
  oldest); return `Vec<BleReceiverDto>` mapped from `&BleReceiver`
  (address string, name, endpoint_id hex — same mapping style as
  `EndpointDto::from`). Errors: adapter missing / bluetoothd down /
  timeout-with-zero-results returns `[]`, NOT an error (CLI `devices`
  prints empty too).
- `cmds/send_ble.rs` (new): `qsd_send_ble{receiver_id, files, send_id?}` —
  (a) resolve token or VALIDATION error; (b) regular-file validation
  identical to `send_payload.rs:21-29`; (c) generate `send_id`
  (`ble-<nanos>` or frontend-provided) and return it immediately;
  (d) `tauri::async_runtime::spawn` the `send_files_ble` future with
  `state.message_sender.clone()` + stored token; log terminal state.
- `cmds/mod.rs` + `main.rs` invoke handler: register both commands.
- `dto.rs`: add `ScanBleInput{ name: Option<String>, timeout_s: Option<u64> }`
  and `SendBleInput{ receiver_id: String, files: Vec<String>, send_id:
  Option<String> }` (+ round-trip tests in the existing `mod tests`).
  Touch nothing else in `dto.rs`; `qsd.ts` gains the two input interfaces.
- Token map location: `AppState` (new field
  `ble_receivers: Mutex<HashMap<String, BleReceiver>>`) so scan and send
  share it; requires `BleReceiver: Clone` (it derives Clone ✓).

## 3. Frontend work (`app/main/src/`)

- `types/qsd.ts`: `ScanBleInput`, `SendBleInput`; helper
  `bleReceiverLabel(r: BleReceiverDto): string` mirroring CLI
  `receiver_label` (`quickshare-direct.rs:247`) — name + address suffix.
- `stores/devices.ts`: add `bleReceivers: Map<string, BleReceiverDto>`,
  `scanning: boolean`, `lastScanAt: number|null`,
  `scanBle(name?)` (invoke `qsd_scan_ble`, replace map contents),
  `clearBle()`. Keep mDNS `endpoints` untouched.
- `DeviceBoard.vue`: new section above/below LAN list — "Nearby over
  Bluetooth" with `[Scan]` button, spinner + `lastScanAt`, rows from
  `bleReceivers` (name, address, "no shared network needed" hint), empty
  state distinguishing "not scanned yet" vs "none found (is the phone on
  its Receive screen?)". Keep the existing `blePresenceAt` banner as-is.
- `SendPane.vue`: target picker gains the BLE list; selecting one calls
  `qsd_send_ble` (not `qsd_send_lan`); staged-file validation stays
  (`validateSendFiles`).
- `TransferQueue.vue` / `TransferCard.vue`: Outbound rows already render;
  add the outbound-waiting state ("Waiting for accept on <phone> — check
  the PIN there, no button here") for pre-`SendingFiles` Outbound states,
  reusing `HANDSHAKE_STATES`/`WaitingForUserConsent` display logic but
  WITHOUT Accept/Decline buttons when `kind === 'Outbound'`.
- `tests/`: vitest for `scanBle` store action (mocked invoke), label helper,
  and Outbound-waiting rendering.

## 4. Validation (must all be green, no heavy local Tauri build needed)

- `pnpm ts-check`, `pnpm test`, `pnpm lint` in `app/main`.
- `cargo check` + `cargo test` in `app/main/src-tauri` (covers new DTO tests).
- Manual E2E (needs BlueZ experimental + join helper + offline phone):
  Scan finds the phone → send completes → progress bar moves → phone-side
  accept works → file arrives; cancel mid-handshake works; re-scan after
  phone leaves Receive mode empties the list.

## 5. Explicit non-goals

- No changes to `core_lib/` (not even `Cargo.toml`/`Cargo.lock`).
- No streaming BLE presence list (throttled `qsd://ble-presence` stays).
- No multi-file/folder sends (regular files only, CLI parity).
- No Windows/macOS packaging changes (`release.yml` untouched; x86_64 Linux).

## 6. Risks / notes for the implementer

- Two D-Bus adapter handles (GUI advertiser + scan task) need BlueZ
  experimental; without it, scan returns [] — report `NOT_RUNNING`-style
  hint, don't crash.
- `discover_receivers` runs active discovery; concurrent GUI advertising is
  the same pattern the CLI uses (advertise-then-send), known good.
- Token map entries go stale (phone leaves); `send_files_ble` failure
  surfaces as `Disconnected` transfer state — map that to a toast with a
  "re-scan" action.
- `send_id` uniqueness: prefix `ble-` to never collide with LAN `id`s
  (`ip:port`) or inbound `ble-1`-style core ids — check core's own id
  scheme first (`send.rs` uses `"send"` in CLI; GUI needs per-transfer ids).
