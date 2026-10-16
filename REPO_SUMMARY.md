# REPO_SUMMARY.md — quickshare-direct

Analisi della struttura del repository (branch corrente: `DeveloperMatt02/recon-gui`).
Fonti: `README.md`, `CLAUDE.md`, `BUILD.md`, `docs/DEV_NOTES.md`, manifest di dipendenza, entrypoint.

---

## 1. Identità e stato del progetto

- **quickshare-direct**: ricevitore Quick Share / Nearby Share per Linux che **non richiede una Wi-Fi
  condivisa**: primo contatto via Bluetooth LE, poi il trasferimento passa su un gruppo Wi-Fi Direct
  ospitato dal laptop (fallback: hotspot, LAN condivisa).
- GPL-3.0, fork di **rQuickShare** (github.com/papshmeare/quickshare-direct, fork del branch BLE
  receiver di martinalderson).
- Funzionante end-to-end con Pixel 10 (packet ~33–37 MB/s in ricezione, ~36 MB/s in invio).
- `VERSION` = `0.1.5` (release = bump di `VERSION` su `main`, CI `packages.yml` pubblica `qsd-v<VERSION>`).

---

## 2. Architettura generale

```
                    ┌──────────────────────────────────────────────┐
                    │  Telefonino Android (Quick Share)            │
                    └───────────────┬──────────────────────────────┘
                 BLE / GATT / RFCOMM│(primo contatto, handshake UKEY2 + PIN)
                                    ▼
┌─────────────────────────────  core_lib (crate `rqs_lib`)  ─────────────────────────────┐
│ lib.rs  → RQS: orchestratore (TaskTracker + CancellationToken)                          │
│ manager.rs → TcpServer (accept connessioni LAN/BWU, router verso inbound/outbound)      │
│ channel.rs → messaggi di controllo (ChannelMessage/Message/TransferAction)              │
│ hdl/    → macchine a stati e bearer (vedi §6)                                           │
│ proto_src/ + build.rs → protobuf Nearby (prost) compilati in OUT_DIR                    │
└──────┬──────────────────────────────┬───────────────────────────────┬──────────────────┘
       │ libreria diretta (Rust)      │ libreria diretta (Rust)       │ libreria diretta (Rust)
       ▼                              ▼                               ▼
┌─────────────────┐        ┌──────────────────────┐        ┌──────────────────────────┐
│ core_lib/src/   │        │ app/main (Tauri 2 +  │        │ app/legacy (Tauri 1 +    │
│ bin/quickshare- │        │ Vue 3) — GUI desktop │        │ Vue 3) — GUI upstream    │
│ direct.rs       │        │ IPC: invoke + event  │        │ NON compilata/usa­ta     │
│ (daemon + CLI)  │        └──────────────────────┘        └──────────────────────────┘
└─────────────────┘
       ▲
       │ systemd user unit / Makefile / flake.nix / .desktop (Thunar sendto)
       │
packaging/linux/* → helper root (quickshare-ap, quickshare-join, quickshare-bt-setup, ...)
```

Non esistono API REST, RPC di rete o un server web: **tutto passa per chiamate di libreria in-process
(Rust)**, più un livello IPC Tauri (invoke/event) se si usa la GUI, più notifiche desktop
(`notify-send`) nel caso headless.

---

## 3. File di configurazione delle dipendenze

| File | Ruolo |
|---|---|
| `core_lib/Cargo.toml` | crate **`rqs_lib`** (edition 2024), 2 bin: `core_bin` (`src/bin.rs`) e `quickshare-direct` (`src/bin/quickshare-direct.rs`). Dipendenze: tokio, bluer (feature `experimental`, default on), mdns-sd (git fork), prost, p256, libaes, sys_metrics (git), once_cell, tokio-util. Build-dep: `prost-build`. |
| `core_lib/package.json` | npm package **`@martichou/core_lib`**: solo tipi TypeScript per la Vue app (`dist/`, `esm/`, `bindings/`), build con `tsc`. Nessun runtime. |
| `core_lib/bindings/*.ts` | tipi generati in passato da **ts-rs** (`State.ts`, `ChannelMessage.ts`, `Visibility.ts`, ...). *ts-rs oggi non è più nelle dipendenze: i file sono stale (vedi §7).* |
| `app/main/package.json` | GUI frontend: Vue 3.5, Pinia, Tailwind, Vite 5, Vitest, `@tauri-apps/api` v2, plugin tauri (store, dialog, notification, shell, autostart, clipboard), dipendenza `@martichou/core_lib: link:../../core_lib`. Package manager **pnpm 9.7**. |
| `app/main/src-tauri/Cargo.toml` | crate `rquickshare` (Tauri **2.2**, tray-icon, single-instance...), dipende da `rqs_lib = { path = "../../../core_lib", features = ["experimental"] }`. |
| `app/legacy/package.json` + `app/legacy/src-tauri/Cargo.toml` | stessa app upstream su **Tauri 1.8** + `@tauri-apps/api` 1.6 (cartella "legacy" di rQuickShare). |
| `Makefile` | `make` → `cargo build --release --locked --bin quickshare-direct`; `make install DESTDIR= PREFIX=` installa binario, helper, unit systemd, polkit, firewalld/ufw, NM, .desktop. |
| `flake.nix` | packaging Nix + modulo NixOS `services.quickshare-direct` + `devShells` (`nix develop`). |
| `install.sh`, `packaging/build-packages.sh` | installer one-line (scelta pacchetto da `/etc/os-release`) e build `.deb/.rpm/.pkg.tar.zst/.tar.gz` con fpm. |
| `.pre-commit-config.yaml`, `fmt.sh`, `rustfmt.toml` | lint/format (pre-commit + `cargo fmt`). |
| `release-please-config.json`, `.github/workflows/*` | release automation; `packages.yml` è la CI attiva di questo fork, `build.yml`/`lint.yml` sono upstream (master). |

Nessun `pyproject.toml`, `CMakeLists.txt`, `go.mod`: il progetto è **Rust + TypeScript/Vue**.

---

## 4. Entry point

### 4.1 Binari Rust (`core_lib`)

1. **`quickshare-direct`** — `core_lib/src/bin/quickshare-direct.rs` (587 righe), l'entrypoint
   principale del progetto:
   - *nessun argomento* → **daemon ricevitore**: `RQS::new(Visibility::Visible, port, dir, nome)` →
     `rqs.run()`, poi un task consuma `rqs.message_sender` (`broadcast::Receiver<ChannelMessage>`)
     e mostra notifiche `notify-send` con azioni **Accept/Decline (con PIN)**,
     "Show in folder" (`org.freedesktop.FileManager1.ShowItems`), "Open" (link), "Copy" (testo).
   - `quickshare-direct send [--to NAME] FILE...` → trova il telefono in ricezione (BLE), mostra il
     PIN, invia; upgrade sul gruppo Wi-Fi Direct del telefono tramite `quickshare-join`.
   - `quickshare-direct devices` → elenca i ricevitori vicini.
   - `help/-h` → usage; codici di uscita: 0 ok, 1 errore/rifiuto, 2 usage, 130 annullato.
   - Config via env: `QSD_DIR`, `QSD_NAME`, `QSD_PORT`, `QSD_BWU_PORT`, `QSD_BWU`, `QSD_CONSENT_TIMEOUT`,
     `QSD_BLE_SCAN`, `QSD_BT_CLASSIC`, `RUST_LOG` (default `info`).
2. **`core_bin`** — `core_lib/src/bin.rs`: minimo demo upstream (RQS default + discovery, stop su
   Ctrl+C). Non usato dalla packaging.
3. `core_lib/examples/` — `l2cap_probe.rs`, `rx_advert.rs`, `rx_service.rs` (strumenti di test).

### 4.2 GUI (Tauri)

- `app/main/src-tauri/src/main.rs` → `tauri::Builder` + `invoke_handler` + tray + `AppState`
  (`message_sender`, `dch_sender`, `visibility_sender`, `sender_file: mpsc::Sender<SendInfo>`,
  `ble_receiver`, `rqs: Mutex<RQS>`).
- `app/main/index.html` + `src/main.ts` → Vue app (`App.vue`: barra laterale `StatusHeader.vue` + area Activity con `SendPane`/`DeviceBoard`/`TransferQueue`/`TransferCard`).

### 4.3 Servizi / sistema

- `packaging/linux/systemd/quickshare-direct.service` (unit utente, `ExecStart=@BINDIR@/quickshare-direct`,
  `EnvironmentFile=-%h/.config/quickshare-direct/env`), template `quickshare-ap@USER` /
  `quickshare-join@USER` (root), `quickshare-bt-connectable.service`.
- Helper root: `packaging/linux/quickshare-ap` (Wi-Fi Direct GO con wpa_supplicant D-Bus o hostapd +
  dnsmasq + nft), `quickshare-join`, `quickshare-bt-setup`, `quickshare-reconnect`.
- `.desktop`: `quickshare-direct-send.desktop` e sendto Thunar → invocano la CLI `send`.
- NixOS: `flake.nix` → `services.quickshare-direct`.

---

## 5. GUI esistente? Branch/cartelle dedicati?

**Sì, esiste una GUI desktop (Tauri + Vue), ma è upstream e non è più allineata al core.**

| Dove | Cosa | Stato |
|---|---|---|
| `app/main/` | Tauri **2** + Vue 3 + Tailwind (crate `directdrop`), tray icon, notifiche, store persistente, discovery, send; layout a barra laterale (radar `FlowHero.vue`, nome del PC, visibilità, cartella e porta) + area Activity (consenso, pannello SEND, elenco NOW / TODAY / EARLIER); tema chiaro/scuro automatico | GUI attuale del fork (`DTO_VERSION` 1, bridge `dto.rs`/`bridge.rs`); vedi `GUI_ARCHITECTURE.md` |
| `app/legacy/` | stessa app su Tauri **1** | legacy upstream, stessa situazione |
| `app/main/src/composables/` | `SettingsDrawer.vue`, `systemTheme.ts` (tema di sistema: portale `org.freedesktop.appearance color-scheme` via `qsd_system_theme`/`qsd://system-theme`, riserva su `theme()` di Tauri e sulla media query) | UI (nessuna logica di business) |
| `app/main/src/stores/` | `session.ts`, `devices.ts`, `transfers.ts` (con `timing`: `startedAt`/`endedAt` lato client), `transferGroups.ts` (raggruppamento NOW / TODAY / EARLIER, velocità media) | stato Pinia |
| `app/main/src/vue_lib/` | `utils.ts` (wrapper su `invoke`), `types.ts` (tipi da `@martichou/core_lib/bindings`) | adapter UI→IPC (ereditato da upstream; la GUI attuale usa `src/types/qsd.ts`) |

Nessuna **GUI web server-side**, nessuna **TUI** (l'unico "interfaccia a riga" è la CLI + notifiche).

**Branch**: 
- `DeveloperMatt02/recon-gui` (**corrente**) e `DeveloperMatt02/plan-gui-arch` → **entrambi identici a
  `main`** (nessun commit di GUI ancora; sono i branch di lavoro per questo task).
- `origin/main` (= HEAD), `origin/master` (upstream rQuickShare), `origin/wip/refactor`
  ("rewrite the interface for the core_lib part 2/3" — riscrittura interfaccia core), `origin/arm64`,
  vari `dependabot/*`.
- Non esistono cartelle/branch come `gui/`, `tui/`, `web/`.

---

## 6. Come viene invocata la logica core

### 6.1 Libreria diretta (unico modo reale)

Il cuore è la struct **`rqs_lib::RQS`** (`core_lib/src/lib.rs`):

```rust
let mut rqs = RQS::new(visibility, port_number, download_path, device_name);
let (sender_file, ble_receiver) = rqs.run().await?;   // avvia TcpServer + mDNS + BLE (+ GATT/RFCOMM)
rqs.discovery(dch_sender)?;                            // discovery mDNS + BleAdvertiser
rqs.stop_discovery(); rqs.change_visibility(v); rqs.set_download_path(p); rqs.stop().await;
```

Canali (nessuna callback registrata dall'esterno, solo canali tokio):

- `rqs.message_sender: broadcast::Sender<ChannelMessage>` → **stato dei trasferimenti in uscita dalla
  lib** (`Message::Client(MessageClient{ kind, state, metadata })`) e **comandi in ingresso alla lib**
  (`Message::Lib { action: ConsentAccept | ConsentDecline | TransferCancel }`).
  Il consenso Accept/Decline si esprime inviando `ChannelMessage { id, msg: Message::Lib { .. } }`
  su questo sender.
- `EndpointInfo` su `dch_sender` → dispositivi scoperti (usato dalla GUI).
- `mpsc::Sender<SendInfo>` (`manager::SendInfo { id, name, addr, ob: OutboundPayload }`) → avvia un invio.
- `watch::Sender<Visibility>` → visibilità (Visible/Invisible/Temporarily).

### 6.2 CLI (headless)

`quickshare-direct [send|devices]` (§4.1): consuma gli stessi canali e traduce lo stato in
**notifiche desktop con azioni**; per l'invio chiama direttamente le funzioni `hdl`:
`discover_receiver(s)`, `send_files_ble(...)`, `join_network(...)`, `parse_receiver_advertisement(...)`.

### 6.3 GUI Tauri (IPC in-process, non REST/RPC)

Frontend → backend: `invoke()` da `@tauri-apps/api/core` su 7 comandi registrati in `main.rs`:

| Comando (`#[tauri::command]`) | File | Effetto |
|---|---|---|
| `start_discovery` / `stop_discovery` | `cmds/discovery.rs` | `rqs.discovery(dch_sender)` / `rqs.stop_discovery()` |
| `change_visibility` | `cmds/change_visibility.rs` | `rqs.change_visibility(v)` |
| `change_download_path` | `cmds/change_download_path.rs` | `rqs.set_download_path(Option<PathBuf>)` |
| `send_payload` | `cmds/send_payload.rs` | `sender_file.send(SendInfo)` |
| `send_to_rs` | `cmds/send_to_rs.rs` | `message_sender.send(ChannelMessage)` (consenso, annulla) |
| `get_hostname` | `cmds/get_hostname.rs` | `sys_metrics::host::get_hostname()` |

Backend → frontend: `app.emit(...)` con eventi **`rs2js_channelmessage`** e **`rs2js_endpointinfo`**
(assegnati in `spawn_receiver_tasks`, consumati in `HomePage.vue` con `listen(...)`).

### 6.4 Contratto di tipo condiviso

I tipi Rust arrivano al frontend come TypeScript generato (`core_lib/bindings/*.ts`, pacchettizzato
come `@martichou/core_lib` e linkato con `link:../../core_lib`). Origine storica: **ts-rs**, oggi
**non più presente** nelle dipendenze → le generazioni non vengono più rigenerate.

---

## 7. Moduli con la logica di business (`core_lib/src`)

| Modulo | Righe | Responsabilità |
|---|---|---|
| `lib.rs` | 382 | orchestrazione `RQS`: lifecycle, task, canali, visibilità, nome dispositivo, protobuf Nearby inclusi (`sharing_nearby`, `securemessage`, `securegcm`, `location_nearby_connections`) |
| `manager.rs` | 180 | `TcpServer`: accept connessioni, routing a `InboundRequest`/`OutboundRequest`, rilevamento `CLIENT_INTRODUCTION` (BWU) |
| `channel.rs` | 43 | contratto messaggi lib↔UI (`ChannelMessage`, `Message`, `TransferAction`, `TransferKind`, `MessageClient`) |
| `hdl/mod.rs` | 142 | re-export dei moduli `hdl` + enum **`TransferState`** (stato transfer, 18 varianti) + `InnerState` |
| `hdl/inbound.rs` | 2046 | **stato ricezione** + bandwidth upgrade (`do_bwu`, `hotspot_task`, gate consenso `take_bwu_pending`), letti frame cancel-safe |
| `hdl/outbound.rs` | 1690 | **stato invio** (BWU come rispondente) |
| `hdl/hotspot.rs` | 362 | avvia unit helper root, legge `/run/quickshare/credentials` (p2p → WIFI_DIRECT, ap → WIFI_HOTSPOT), fallback NetworkManager |
| `hdl/gatt.rs` / `gatt_client.rs` | 344 / 421 | server GATT + weave (lato ricevitore); discovery e client weave (lato invio) |
| `hdl/att.rs` | 324 | bearer ATT proprietario su socket L2CAP (MTU exchange) |
| `hdl/l2cap.rs` | 183 | canale L2CAP al PSM annunciato dal telefono (preferito su weave) |
| `hdl/ble.rs` / `blea.rs` | 145 / 200 | scan (BLE listener passivo) / advertisement BLE |
| `hdl/rfcomm.rs` | 129 | Bluetooth Classic RFCOMM (opt-in `QSD_BT_CLASSIC=1`) |
| `hdl/send.rs` | 139 | driver dell'invio (`send_files_ble`) |
| `hdl/join.rs` | 120 | join del gruppo Wi-Fi Direct del telefono (via helper `quickshare-join`) |
| `hdl/mdns.rs` / `mdns_discovery.rs` | 156 / 126 | server mDNS (visibilità) / discovery mDNS |
| `hdl/info.rs`, `migratable.rs` | 68 / 57 | metadata transfer (`TransferMetadata`, payload file/text/url/wifi), migrazione stato |
| `utils.rs`, `errors.rs` | 261 / 12 | `DeviceType`, `RemoteDeviceInfo`, helper, `AppError` |
| `bin/quickshare-direct.rs` | 587 | CLI/daemon (presentazione: notifiche, clipboard, xdg-open) — **non** logica di protocollo |

Frontend (`app/*/src`): solo presentazione + adapter IPC; nessuna logica di business.
`packaging/linux/*`: logica di sistema (Wi-Fi Direct/hotspot/firewall), non del protocollo.

---

## 8. Stato della GUI rispetto al core (importante per un nuovo lavoro GUI)

`README.md` e `CLAUDE.md` dichiarano esplicitamente che **`app/` è il Tauri app di rQuickShare, non
buildato, non spedito, non testato in questo fork**, e che il suo invio funziona solo su LAN
condivisa (mDNS + TCP), senza il percorso Bluetooth/Wi-Fi Direct della CLI `send`.

Verifica statica: **`app/main` non compila contro `core_lib` attuale**. Divergenze trovate:

1. `RQS::new(visibility, port, download_path)` — l'attuale firma ha **4 parametri** (aggiunto
   `device_name`) → `app/main/src-tauri/src/main.rs:114`, idem `app/legacy:99`.
2. `use rqs_lib::State` — il tipo `State` **non esiste più** in `rqs_lib` (rinominato `TransferState`
   in `hdl::mod`) → `main.rs:6`, `app/legacy:13`.
3. `use rqs_lib::channel::ChannelDirection` — **rimosso** da `channel.rs` → `main.rs:5`, `app/legacy:11`.
4. `ChannelMessage` attuale è `{ id, msg: Message }`; la GUI usa ancora `message.direction`,
   `info.state`, `info.meta` (`main.rs:194, 294`) e `HomePage.vue` legge `cm.state`/`cm.meta` →
   il wire format UI↔core è cambiato.
5. `core_lib/bindings/*.ts` (e quindi `@martichou/core_lib`) sono **stale**: generati con ts-rs che
   non è più una dipendenza; `State.ts` descrive la vecchia enum.
6. `app/main` usa il send-only LAN (nessun BLE send, nessun `quickshare-direct send`).

Quindi per una GUI c'è da scegliere tra:
- **riattivare `app/main`** aggiornando il Tauri `AppState`/comandi al wire format attuale e
  rigenerando i bindings TS (servirebbe rimettere ts-rs o uno script di generazione equivalente);
- **costruire una nuova UI** (web/TUI) che parli con un nuovo lato server del core usando gli stessi
  canali tokio descritti in §6.1 — occhio: oggi il core espone canali in-process, **nessun protocollo
  di rete/IPC già pronto** (niente REST/RPC/Unix socket), quindi un'GUI fuori processo richiede di
  aggiungere un bridge (socket/DBus) o di restare in-process (Tauri/tauri-sidecar).

---

## 9. Comandi di sviluppo

```bash
nix develop                                    # shell con cargo, protoc, libdbus
cd core_lib && cargo build --release --bin quickshare-direct
cd core_lib && cargo run --bin quickshare-direct        # receiver dev
core_lib/target/release/quickshare-direct send FILE...  # invio
make                                            # build del binario (solo quickshare-direct)
make install DESTDIR=... PREFIX=...             # stage/install (no GUI)
cd app/main && pnpm install && pnpm dev         # GUI Tauri 2 (ATTENZIONE: non compila, §8)
cd app/main && pnpm test                        # vitest (solo example.test.ts)
```

Il service installato va fermato prima di un dev build (`systemctl --user stop quickshare-direct`;
porta TCP 46257/46258 + advertising BLE). Test su telefono: `tools/dev/qs-auto.sh`, `qs-accept.sh`,
`qs-receive-mode.sh` (USB adb).

## 10. Documentazione rilevante

- `README.md` — stato, install, layout ("Repository layout" a riga 120).
- `CLAUDE.md` — note operative per agenti: layout, dev loop, stato.
- `docs/DEV_NOTES.md` — findings, evidenze da log telefono, misure, piano aperto.
- `docs/BLE_RECEIVER_DISCOVERY.md` — protocollo BLE/weave.
- `BUILD.md` — istruzioni di build upstream per `core_lib` e `app/{main,legacy}`.
