# GUI_ARCHITECTURE.md — quickshare-direct

> Progetto GUI per `quickshare-direct` **senza modificare `core_lib/`**.
> Base fattuale: `REPO_SUMMARY.md` + verifica diretta di `core_lib/src/{lib,channel,manager,hdl/*,bin/quickshare-direct}.rs`,
> `app/main/src-tauri/src/{main,cmds/*,store}.rs`, `app/main/src/{components/HomePage.vue,vue_lib/*}`.
> Branch di lavoro: `DeveloperMatt02/plan-gui-arch` (identico a `main`, nessun commit GUI).

> **Aggiornamento (GUI attuale, dopo `feat/ui-redesign`):** il documento descrive il progetto originale; dove diverge, vale il codice. Layout attuale: barra laterale (`StatusHeader.vue`: radar `FlowHero.vue`, nome del PC, selettore Visible / Temporarily / Hidden, cartella download e porta) + area **Activity** (`ConsentCard` in alto, `SendPane` + `DeviceBoard`, `TransferQueue` raggruppata in NOW / TODAY / EARLIER). `FlowDiagram.vue` è stato rimosso. Il tema chiaro/scuro segue il sistema (`composables/systemTheme.ts`, token `--qsd-*`). Le viste `ReceiveView`/`SendView` e `SettingsModal` citate sotto non esistono: c'è un'unica `App.vue` e `composables/SettingsDrawer.vue`.

---

## 0. Vincoli e decisioni quadro

| # | Vincolo / fatto | Conseguenza |
|---|---|---|
| V1 | Il core espone **solo API Rust in-process** (`RQS` + canali tokio: `broadcast::Sender<ChannelMessage>`, `broadcast::Sender<EndpointInfo>` via discovery, `mpsc::Sender<SendInfo>`, `watch::Sender<Visibility>`). Nessun REST/RPC/socket/DBus. | La GUI **deve girare nello stesso processo del core**. Qualsiasi GUI fuori processo (web app separata, Electron con sidecar, TUI su altro binario) richiederebbe un bridge IPC nuovo = modifica al perimetro core. Scartate. |
| V2 | `core_lib` dipende già da `serde` ma **nessun tipo di dominio deriva `Serialize/Deserialize`** (verificato con grep: zero match su `serde` in `core_lib/src`). | Il wire format GUI↔core **non può riusare i tipi core tal quali** via JSON. Serve un **DTO layer in `app/main/src-tauri`** (serde lì già presente) con `From` mapping. Zero modifiche a `core_lib/`. |
| V3 | `app/main` (Tauri 2 + Vue 3) **non compila** contro il core attuale: `RQS::new` a 4 args (GUI ne passa 3, `main.rs:130`), `rqs_lib::State` e `channel::ChannelDirection` non esistono più, `ChannelMessage` è `{id, msg}` ma la GUI legge `direction/info.state/info.meta`, bindings `@martichou/core_lib` stale (ts-rs rimosso). | Non si butta `app/main`: si **riattiva** con un adapter. Il lavoro è meccanico e confinato in `app/main/src-tauri/src` + `app/main/src`. |
| V4 | Invio BLE (`hdl::send_files_ble`, `discover_receivers`, `BleReceiver`) non passa per `RQS`/`TcpServer`: è un driver autonomo con `bluer::Adapter` + `broadcast::Sender<ChannelMessage>` proprio + `CancellationToken`. | L'invio BLE in GUI è **fase 2**: nuovo comando Tauri che esegue il driver in un task dedicato. Nessuna modifica al driver. |
| V5 | CLI headless resta il riferimento UX (consenso con PIN + timeout `QSD_CONSENT_TIMEOUT=60s`, notifiche, `ShowItems` via FileManager1, copia testo, open link). | La GUI deve replicare **almeno** quella semantica: consenso con PIN e timeout, idempotenza (il core ripete gli update di stato), dedup per `id`. |

**Decisione quadro: riattivare `app/main` (Tauri 2 + Vue 3) con un DTO/bridge layer. Nessun file sotto `core_lib/`, `packaging/`, `flake.nix`, `Makefile` viene toccato.**

---

## 1. Stack consigliato

**Confermato: Tauri 2.2 + Vue 3.5 + Pinia + Tailwind + Vite 5 + Vitest + pnpm 9.7 — cioè lo stack già presente in `app/main/`.**

| Livello | Scelta | Perché (e alternative scartate) |
|---|---|---|
| Shell desktop | **Tauri 2** (`app/main/src-tauri`, crate `rquickshare`) | Solo modo per ospitare `rqs_lib` in-process senza bridge (V1). Plugin già dichiarati: `store, dialog, notification, shell, autostart, clipboard, single-instance, tray-icon`. Scartati: Electron (bundle ~150 MB, servirebbe comunque sidecar Rust), .NET/GTK nativo (rewrite UI, perdita Vite/Tailwind), web server embedded (nuova superficie di rete + auth da progettare, modifica al perimetro). |
| Frontend | **Vue 3.5 + Pinia + Tailwind** (già in `app/main/package.json`) | La logica di presentazione esiste già (`HomePage.vue` 379 righe, `SideMenu/ContentStatus/SettingsModal/ItemSide/Heading`). Si refactorizza, non si riscrive. Scartati: React/Svelte (rewrite immotivato), TUI ratatui (perde drag&drop file, notifiche, tray — core UX del prodotto). |
| Tipi condivisi | **DTO hand-written versionati**, non ts-rs | ts-rs è stato rimosso dalle dipendenze e i `core_lib/bindings/*.ts` sono stale. Reintrodurre derive macro nel core violerebbe il vincolo. I DTO vivono in `app/main/src-tauri/src/dto.rs` (Rust/serde) + `app/main/src/types/qsd.ts` (TS), con `DTO_VERSION` e test di snapshot (vedi §5). `tauri-specta` valutabile in seguito, non ora (aggiungerebbe codegen al build). |
| Package manager | **pnpm 9.7** (già pinnato) | La dipendenza `link:../../core_lib` va **rimossa** (era solo per i bindings stale). |
| Test | **Vitest** (esistente) + `cargo test` sul bridge | Nessun test di business nel core da toccare; si testa mapping DTO e store. |

### 1.1 Albero dei file (solo aggiunte/modifiche sotto `app/main/`)

```
app/main/src-tauri/src/
  main.rs                 # MOD: fix RQS::new a 4 args, AppState invariato, event names qsd://*
  dto.rs                  # NEW: tutti i DTO serde + From<tipi core> (unica fonte del wire format)
  bridge.rs               # NEW: spawn_receiver_tasks riscritto su DTO (broadcast→emit, lag-skip, throttle)
  cmds/
    mod.rs                # MOD: registra i comandi §3.1
    discovery.rs          # MOD: invariato nella firma, usa dch_sender di DTO-convertiti
    change_visibility.rs  # MOD: accetta VisibilityDto
    change_download_path.rs # MOD: invariato
    send_payload.rs       # MOD: accetta SendPayloadDto {endpoint_id,name,addr,files}
    send_to_rs.rs         # MOD: accetta ConsentDto/CancelDto → ChannelMessage core
    settings.rs           # NEW: get/set device_name, get download_path, get version/snapshot
    ble_send.rs           # NEW (fase 2): ble_devices + ble_send + ble_cancel
  store.rs                # MOD: chiavi esistenti + device_name
app/main/src/
  types/qsd.ts            # NEW: tipi TS speculari a dto.rs (sostituisce @martichou/core_lib/bindings)
  stores/
    transfers.ts          # NEW: Pinia store trasferimenti (inbound+outbound)
    devices.ts            # NEW: endpoint mDNS + ricevitori BLE
    session.ts            # NEW: visibility, device_name, download_path, discoveryRunning
    transferGroups.ts     # NEW: funzioni pure per l'elenco Activity (NOW / TODAY / EARLIER, velocità media AVG)
    toasts.ts             # KEEP: esistente useToastStore
  views/ ReceiveView.vue  # NEW (refactor di HomePage: sezione ricezione)
  views/ SendView.vue     # NEW (refactor di HomePage: sezione invio)
  components/ StatusHeader.vue (barra laterale), FlowHero.vue (radar), SendPane.vue, DeviceBoard.vue, DeviceRow.vue,
             TransferQueue.vue (Activity), TransferCard.vue, ConsentCard.vue, TransportBadge.vue, ProgressBar.vue  # NEW
  composables/            # SettingsDrawer.vue, systemTheme.ts (tema di sistema)
  vue_lib/                # MOD: utils.ts riscritto sui nuovi invoke/event; types.ts deprecato
```

**Regola di confine:** `app/main/src-tauri` può importare da `rqs_lib` solo: `RQS`, `Visibility`, `EndpointInfo`, `SendInfo`, `TransferState`, `TransferKind`, `TransferAction`, `ChannelMessage`, `Message`, `MessageClient`, `OutboundPayload`, `TransferMetadata`, `TransferPayload`, `RemoteDeviceInfo`, `DeviceType`, `hdl::{discover_receivers, send_files_ble, BleReceiver}`. Tutto il resto del core è off-limits alla GUI.

---

## 2. Layer di comunicazione GUI ↔ core

### 2.1 Principio: Tauri invoke + eventi, DTO JSON, core intoccato

```
┌─ Vue (Pinia stores) ─── invoke(cmd, dto-json) ──→ Tauri backend ── From<Dto> ──→ rqs_lib (canali tokio)
└─ Vue (listen qsd://*) ←── emit(dto-json) ─────── Tauri bridge ←── map(core→Dto) ── rqs_lib (broadcast)
```

- **FE→BE: `invoke()`** (`@tauri-apps/api/core`). Ogni comando prende/ritorna DTO serde. Errori come `Result<T, GuiError>` con `GuiError { code, message }` serializzato (mai `panic!`, mai `unwrap` su lock oltre `map_err`).
- **BE→FE: `app.emit()`** con nomi **`qsd://transfer`**, **`qsd://endpoints`**, **`qsd://visibility`**, **`qsd://ble-presence`** (rinominano `rs2js_channelmessage`/`rs2js_endpointinfo`/`visibility_updated`; i vecchi nomi vanno rimossi perché legavano al wire format defunto).
- **Serializzazione:** solo `dto.rs` deriva `Serialize/Deserialize`. I tipi core non vengono mai emessi direttamente.
- **Robustezza broadcast:** `broadcast` ha capacità 50 (message) / 10 (discovery). Il bridge su `Lagged(n)` fa **skip + debug log** (come già fa la CLI, `quickshare-direct.rs:483`), mai crash. Progress ad alta frequenza throttled (§2.4).
- **Idempotenza:** il core **ripete** gli update di stato per lo stesso `id`. Il frontend fa upsert per `id` e ignora update con `(kind,state,ack_bytes)` invariati.

### 2.2 Semantica degli `id` (contratto)

| Direzione | `id` | Ciclo di vita |
|---|---|---|
| Inbound (ricezione via `TcpServer`) | `remote_addr` (`ip:port` TCP) assegnato dal core | Appare con `WaitingForUserConsent`, poi `ReceivingFiles`, poi terminale (`Finished/Rejected/Cancelled/Disconnected`). Dopo un terminale l'`id` può essere rimosso dalla store (pulsante Clear). |
| Outbound LAN (via `SendInfo`) | scelto dal frontend (UUID v4) e passato in `send_payload` | Stati `SentIntroduction → SendingFiles → Finished/Rejected/Cancelled/Disconnected`. |
| Outbound BLE (fase 2, via `send_files_ble`) | `send_id` scelto dal frontend (stesso UUID) | Identico al LAN, ma il task vive nel backend finché non termina/cancella. |

### 2.3 DTO e contratti JSON (fonte: `dto.rs` ↔ `types/qsd.ts`)

> Notazione: enum Rust serializzate come **stringhe** (`#[serde(rename_all="...")]` indicato). `u64` come number. Mai `null` dove il core usa `Option`: assente o `null` accettati in ingresso, sempre presenti con default in uscita verso FE per semplificare Vue.

**`DTO_VERSION = 1`** — costante duplicata in `dto.rs` e `qsd.ts`; comando `qsd_snapshot` la espone; test CI fallisce se divergono.

```ts
// types/qsd.ts (speculare a dto.rs)
export const DTO_VERSION = 1;
export type VisibilityDto = "Visible" | "Invisible" | "Temporarily";
export type TransferKindDto = "Inbound" | "Outbound";            // TransferKind
export type TransferActionDto = "ConsentAccept" | "ConsentDecline" | "TransferCancel";
export type TransferStateDto =                                  // TransferState, 18 varianti (hdl/mod.rs:64)
  | "Initial" | "ReceivedConnectionRequest" | "SentUkeyServerInit" | "SentUkeyClientInit"
  | "SentUkeyClientFinish" | "SentPairedKeyEncryption" | "ReceivedUkeyClientFinish"
  | "SentConnectionResponse" | "SentPairedKeyResult" | "SentIntroduction"
  | "ReceivedPairedKeyResult" | "WaitingForUserConsent" | "ReceivingFiles" | "SendingFiles"
  | "Disconnected" | "Rejected" | "Cancelled" | "Finished";
export type PayloadKindDto = "Files" | "Text" | "Url" | "WiFi"; // TransferPayloadKind
export interface RemoteDeviceDto { name: string; device_type: string; }  // RemoteDeviceInfo+DeviceType (Phone/Tablet/Laptop/Unknown)
export interface TransferMetadataDto {
  id: string; source: RemoteDeviceDto | null; pin_code: string | null;
  payload_kind: PayloadKindDto; payload_preview: string | null;
  files: string[] | null; text: string | null; url: string | null; wifi_ssid: string | null;
  total_bytes: number; ack_bytes: number;
}
export interface TransferDto {  // evento qsd://transfer (da Message::Client)
  id: string; kind: TransferKindDto; state: TransferStateDto; metadata: TransferMetadataDto | null;
}
export interface EndpointDto {  // evento qsd://endpoints (da EndpointInfo)
  id: string; fullname: string; name: string | null; ip: string | null; port: string | null;
  device_type: string | null; present: boolean;
}
export interface SendPayloadDto { id: string; name: string; addr: string; files: string[]; } // → SendInfo{ob: Files}
export interface ConsentDto { id: string; action: TransferActionDto; }
export interface BleReceiverDto { address: string; name: string | null; endpoint_id: string; }
export interface SnapshotDto {
  dto_version: number; visibility: VisibilityDto; device_name: string;
  download_path: string | null; port: number | null; discovery_running: boolean;
}
export interface GuiError { code: string; message: string; }  // code: CORE|VALIDATION|NOT_RUNNING|IO|TIMEOUT
```

Mapping note (da implementare in `dto.rs`, mai nel core):
- `Message::Lib` in uscita dal core non esiste (il core emette solo `Message::Client`); `Message::Lib` si costruisce solo in ingresso da `ConsentDto`.
- `TransferMetadata.payload: Option<TransferPayload>` → appiattito in `files/text/url/wifi_ssid` (+ `payload_preview` per i testi). `payload_kind` è sempre noto anche a payload assente (il core lo garantisce per far decidere al client prima del completamento).
- `DeviceType` numerico → stringa (`Unknown/Phone/Tablet/Laptop`).
- `Visibility` ↔ `u8` store (`Visible=0, Invisible=1, Temporarily=2`, come `store.rs` esistente).

### 2.4 Throttling e dedup del bridge (`bridge.rs`)

- Emit `qsd://transfer` solo se: cambio di `state`, oppure `ack_bytes` delta ≥ 256 KiB, oppure ≥ 500 ms dall'ultimo emit per quell'`id`. Ultimo stato terminale sempre emesso.
- `qsd://endpoints`: emit immediato (eventi rari: resolve/remove mDNS).
- `qsd://ble-presence` (esistente `ble_receiver: broadcast::Receiver<()>`, oggi usato solo per toast "temporarily"): inoltrato come evento vuoto throttled a 120 s (stessa finestra già in `main.rs:256`).
- Dedup FE: `transfers.ts` upsert per `id`; scarta update con stessa tripla `(state, ack_bytes, total_bytes)`.

### 2.5 Perché non REST / WebSocket / DBus

| Alternativa | Perché scartata |
|---|---|
| REST/WS su localhost nel processo Tauri | Richiederebbe un `axum`/`warp` embedded + auth token + CORS + porta: nuova superficie d'attacco e codice da mantenere, **zero benefici** rispetto a invoke/event che è già tipizzato, senza rete. |
| Unix socket / DBus verso daemon `quickshare-direct` | Il daemon non espone alcun protocollo (V1); definirne uno = progettare versioning, auth peer, framing — fuori scope e modifica al perimetro core. Rivalutare solo se in futuro si vorrà una GUI fuori processo che parli col daemon systemd. |
| Sidecar (`tauri-plugin-shell` + binario CLI) | Doppio processo, parsing stdout fragile, niente eventi tipizzati, permission shell extra. |

---

## 3. Contratti API (comandi invoke + eventi)

### 3.1 Comandi FE→BE (`invoke`)

| Comando | Input | Output | Effetto core | Note |
|---|---|---|---|---|
| `qsd_snapshot` | — | `SnapshotDto` | Legge `RQS.port_number`, `get_device_name()`, store, flag discovery | Chiamato al mount; sostituisce i 4 getter sparsi attuali. |
| `qsd_set_visibility` | `{ visibility: VisibilityDto }` | `void` | `rqs.change_visibility(v)` + persist store | Sostituisce `change_visibility`. |
| `qsd_set_download_path` | `{ path: string \| null }` | `void` | `rqs.set_download_path()` + persist | `null` = default XDG Downloads. Sostituisce `change_download_path`. |
| `qsd_set_device_name` | `{ name: string }` (1–255 char, validato) | `void` | `rqs.set_device_name()` + persist | NEW. Effettivo dopo restart del service (documentato in `lib.rs:369`); la GUI mostra hint "si applica ai nuovi annunci". |
| `qsd_start_discovery` | — | `void` | `rqs.discovery(dch_sender)` | Idempotente: se già running → ok. Sostituisce `start_discovery`. |
| `qsd_stop_discovery` | — | `void` | `rqs.stop_discovery()` | Sostituisce `stop_discovery`. |
| `qsd_consent` | `ConsentDto` | `void` | `message_sender.send(ChannelMessage{id, msg: Lib{action}})` | Sostituisce `send_to_rs`. Accept/Decline solo su inbound in `WaitingForUserConsent`; `TransferCancel` su qualsiasi transfer attivo. Dopo l'invio il backend non attende ack (fire-and-forget come la CLI). |
| `qsd_send_lan` | `SendPayloadDto` | `void` | `sender_file.send(SendInfo{id,name,addr,ob:Files})` | Sostituisce `send_payload`. `addr` = `ip:port` da `EndpointDto`. Solo file regolari (cartelle rifiutate con `VALIDATION`, come la CLI). |
| `qsd_get_hostname` | — | `string` | `sys_metrics::host::get_hostname()` | KEEP (`get_hostname`). |
| `qsd_open_path` | `{ name: string \| null, reveal: boolean }` | `void` | Nessuno (solo FS/desktop): risolve `name` nella cartella download (rifiuta percorsi assoluti o che escono dalla cartella) e lo apre o lo mostra nel file manager | Usato da OPEN FILE / OPEN FOLDER (`TransferCard.vue`), solo per le ricezioni. |
| `qsd_system_theme` | — | `"dark" \| "light" \| null` | Nessuno: legge `org.freedesktop.appearance color-scheme` dal portale (`ashpd`) | `null` = nessuna preferenza; la GUI ripiega su `theme()` di Tauri e sulla media query. |
| `qsd_ble_devices` (fase 2) | `{ timeout_secs: number }` (default 8, max 30) | `BleReceiverDto[]` | `hdl::discover_receivers(adapter, None, …)` one-shot | Non passa per `RQS`; apre `bluer::Adapter` on-demand. Errore `Bluetooth off` mappato in `GuiError{code:IO}`. |
| `qsd_ble_send` (fase 2) | `{ send_id: string; address: string; files: string[] }` | `void` (ritorna subito) | `tokio::spawn(send_files_ble(adapter, receiver, files, send_id, message_sender.clone(), ctk))` con `ctk` registrato per `send_id` | Progress via `qsd://transfer` con `kind: Outbound`. |
| `qsd_ble_cancel` (fase 2) | `{ send_id: string }` | `void` | `ctk.cancel()` per quel `send_id` | Il driver ritorna `Cancelled`; il core emette lo stato terminale. |

Firma Rust di riferimento (da scrivere in `cmds/`):

```rust
#[tauri::command] pub async fn qsd_snapshot(s: tauri::State<'_, AppState>) -> Result<SnapshotDto, GuiError>;
#[tauri::command] pub fn qsd_set_visibility(v: VisibilityDto, s: tauri::State<'_, AppState>) -> Result<(), GuiError>;
#[tauri::command] pub fn qsd_set_download_path(p: Option<String>, s: tauri::State<'_, AppState>) -> Result<(), GuiError>;
#[tauri::command] pub fn qsd_set_device_name(n: String, s: tauri::State<'_, AppState>) -> Result<(), GuiError>;
#[tauri::command] pub async fn qsd_start_discovery(s: tauri::State<'_, AppState>) -> Result<(), GuiError>;
#[tauri::command] pub fn qsd_stop_discovery(s: tauri::State<'_, AppState>) -> Result<(), GuiError>;
#[tauri::command] pub fn qsd_consent(m: ConsentDto, s: tauri::State<'_, AppState>) -> Result<(), GuiError>;
#[tauri::command] pub async fn qsd_send_lan(m: SendPayloadDto, s: tauri::State<'_, AppState>) -> Result<(), GuiError>;
```

Vecchi comandi (`start/stop_discovery`, `change_visibility`, `change_download_path`, `send_payload`, `send_to_rs`, `get_hostname`) rimossi dopo la migrazione FE (un solo set attivo alla volta, niente alias).

### 3.2 Eventi BE→FE (`listen`)

| Evento | Payload | Produttore | Consumo |
|---|---|---|---|
| `qsd://transfer` | `TransferDto` | bridge su `message_sender.subscribe()` (ricezione LAN + invio LAN; fase 2 anche BLE) | `transfers.ts` upsert; `ConsentCard` su `WaitingForUserConsent`; toast+notifica su terminali |
| `qsd://endpoints` | `EndpointDto` | bridge su `dch_sender.subscribe()` | `devices.ts`; `present:false` rimuove per `id` |
| `qsd://visibility` | `{ visibility: VisibilityDto }` | bridge su `visibility_sender` watch + emit dopo `Temporarily`→`Invisible` (60 s, logica già nel core `mdns.rs:110`) | `session.ts`; la GUI non implementa mai il timer da sé |
| `qsd://ble-presence` | `{ at: number }` | bridge su `ble_receiver` (throttle 120 s) | toast "un telefono sta condividendo ma sei Invisible" (parità con `send_temporarily_notification`) |
| `qsd://system-theme` | `{ theme: "dark" \| "light" \| null }` | `cmds/system_theme.rs` (osserva il portale) | `composables/systemTheme.ts` aggiorna il tema |

### 3.3 Esempi wire (JSON effettivo)

```jsonc
// qsd://transfer — consenso in arrivo (PIN mostrato sul telefono)
{"id":"192.168.1.5:51234","kind":"Inbound","state":"WaitingForUserConsent",
 "metadata":{"id":"192.168.1.5:51234","source":{"name":"Pixel 10","device_type":"Phone"},
 "pin_code":"4821","payload_kind":"Files","payload_preview":null,"files":["IMG_001.jpg","video.mp4"],
 "text":null,"url":null,"wifi_ssid":null,"total_bytes":38210432,"ack_bytes":0}}
// qsd_consent — accept
{"id":"192.168.1.5:51234","action":"ConsentAccept"}
// qsd_send_lan — invio verso endpoint mDNS
{"id":"3f2a…","name":"Pixel 10","addr":"192.168.1.20:46257","files":["/home/u/Docs/report.pdf"]}
// qsd://endpoints — sparizione
{"id":"192.168.1.20:46257","fullname":"…","name":null,"ip":null,"port":null,"device_type":null,"present":false}
```

---

## 4. Struttura viste e stato (Pinia + componenti)

### 4.1 Store

```ts
// stores/session.ts
{ visibility: VisibilityDto; device_name: string; download_path: string|null;
  port: number|null; discovery_running: boolean; ble_presence_at: number|null; }
// stores/devices.ts
{ endpoints: Map<string,EndpointDto>; ble_receivers: Map<string,BleReceiverDto>;
  actions: { startDiscovery(), stopDiscovery(), refreshBle(timeout) } }
// stores/transfers.ts
{ items: Map<string, TransferView>;   // TransferView = TransferDto + ui: {consent_deadline, expired}
  timing: Map<string, {startedAt, endedAt|null}>;  // timestamp lato client (non nel DTO; in `timing`, non in `ui`)
  actions: { upsert(dto), consent(id,action), cancel(id), clear(id), sendLan(endpointId) } }
```

`TransferView.ui.consent_deadline = now + QSD_CONSENT_TIMEOUT*1000` (default 60 s, letto da env documentato in UI). Allo scadere senza risposta lo store invia `ConsentDecline` (parità CLI §`quickshare-direct.rs:504-518`) e marca `expired:true`.

### 4.2 Mappa stati core → UI

| `TransferStateDto` | UI | Azioni |
|---|---|---|
| `WaitingForUserConsent` | `ConsentCard` (nome, descrizione file/testo/url/wifi come `describe()` CLI, **PIN grande**, countdown) | Accept / Decline |
| `ReceivingFiles` / `SendingFiles` / `SentIntroduction` | `TransferCard` attiva + `ProgressBar` (`ack_bytes/total_bytes`, % e rate) | Cancel |
| `Finished` | Card completata: file ricevuti→OPEN FILE (solo 1 file) / OPEN FOLDER (`qsd_open_path`; con più file solo la cartella), testo→Copy, url→Open; velocità media AVG; nessun OPEN per gli invii | Open / Copy / Clear |
| `Rejected` / `Cancelled` / `Disconnected` | Card errore con messaggio | Clear |
| Tutti gli altri (`Initial…ReceivedPairedKeyResult`) | Riga compatta "Connessione…" (no azioni) — sono transienti di handshake | — |

Filtro display esistente (`stateToDisplay` in `vue_lib/types.ts`) riusato come default, esteso con `SentUkey*` nascosti.

### 4.3 Viste

> Struttura attuale: vedi la nota di aggiornamento in testa (barra laterale + Activity). Il seguito è il progetto originale.

- **`ReceiveView`** (default): visibilità toggle (Visible/Invisible/Temporarily con hint auto-off 60 s), lista inbound (`transfers` filtrati `kind==Inbound`), empty-state "In attesa — visibile come `<device_name>`".
- **`SendView`**: picker file (dialog + drag&drop esistente `onDragDropEvent`), lista `devices.endpoints` (mDNS, click→`sendLan`), sezione BLE fase 2 (scan→`ble_receivers`, click→`ble_send`), lista outbound con progress.
- **`SettingsModal`** (esistente, esteso): device_name (con hint restart-annuncio), download_path (dialog cartella), autostart, realclose, startminimized, porta (read-only da snapshot).
- **Tray + notifiche**: tray menu Show/Quit esistente; notifiche via `tauri-plugin-notification` (consenso anche a finestra chiusa → click apre `ReceiveView`); single-instance focus esistente.

### 4.4 Parità CLI (checklist di accettazione GUI)

- [ ] PIN mostrato su Accept/Decline = `pin_code` del metadata (4 cifre, cfr. `utils::to_four_digit_string`).
- [ ] Timeout consenso 60 s default con Decline automatico.
- [ ] File duplicati/progress ad alta frequenza non freezano la UI (throttle §2.4 + upsert).
- [ ] Solo file regolari inviabili; cartelle → errore `VALIDATION` con toast.
- [ ] `Show in folder` preferisce FileManager1, fallback `shell.open` cartella (parità `show_items`).
- [ ] Testo/url ricevuti: Copy/Open senza salvare file.

---

## 5. Fasi, test, rischi

**F0 — Compila (1–2 gg):** `main.rs` fix `RQS::new(v,port,path,Some(device_name))`, `dto.rs` + `qsd.ts` + `bridge.rs`, 8 comandi §3.1 (senza `ble_*`), rimozione `link:../../core_lib`, `HomePage.vue` adattata ai nuovi nomi (logica invariata). Criterio: `cargo check -p rquickshare` + `pnpm dev` verdi.
**F1 — Ricezione (parità CLI):** store/view §4, consenso con countdown, notifiche, test Vitest su upsert/dedup/timeout + `cargo test` snapshot DTO.
**F2 — Invio LAN:** `qsd_send_lan` + `SendView` mDNS (parità `app/main` originale).
**F3 — Invio BLE (chiude il gap col fork):** `ble_send.rs` + task registry `HashMap<send_id, CancellationToken>` in `AppState`; `SendView` sezione BLE. Nessuna modifica a `send.rs/join.rs/hotspot.rs`.
**F4 — Polish/packaging:** icone/tray, update-check esistente, `.desktop` opzionale GUI, docs utente. Il daemon systemd resta il default installato; la GUI è opt-in (non si toccano `Makefile`/`packaging/` in F0–F3).

**Test:** `cargo test -p rquickshare` (mapping DTO↔fixture JSON, `DTO_VERSION` sync con `qsd.ts`); `pnpm test` (store: upsert idempotente, Lagged-skip, countdown auto-decline, present:false removal); manuale con `tools/dev/qs-*.sh` come da REPO_SUMMARY §9 (service user stoppato, porte 46257/46258 libere).

**Rischi:** (1) `broadcast` lag sotto transfer veloci (~35 MB/s) → mitigato da throttle+skip; (2) `device_name` richiede restart annunci → hint UI, mai restart automatico durante transfer; (3) BLE scan richiede adapter BlueZ + permessi → errori mappati in toast, mai crash; (4) divergenza futura core → i DTO isolano la GUI: se il core cambia, si aggiorna solo `dto.rs`+snapshot.

---

## 6. File da creare/modificare (riepilogo operativo)

1. `app/main/src-tauri/src/dto.rs` (NEW, ~250 righe) — DTO §2.3 + `From` + `DTO_VERSION`.
2. `app/main/src-tauri/src/bridge.rs` (NEW) — receiver tasks con throttle/dedup §2.4, nomi eventi §3.2.
3. `app/main/src-tauri/src/cmds/*` (MOD+NEW) — comandi §3.1, `GuiError`, registro BLE task (F3).
4. `app/main/src-tauri/src/main.rs` (MOD) — 4-arg `RQS::new`, `AppState` + `ble_tasks`, handler aggiornato.
5. `app/main/src/types/qsd.ts` (NEW) — tipi §2.3, `DTO_VERSION`.
6. `app/main/src/stores/{session,devices,transfers}.ts` + `views/` + `components/` (NEW/refactor) — §4.
7. `app/main/src/vue_lib/{utils.ts}` (MOD, nuovi invoke/event), `types.ts` deprecato; `package.json` senza `link:../../core_lib`.
8. `app/main/src-tauri/src/dto_snapshot.json` + test sync (NEW) — garanzia DTO↔TS.

*Non toccare: `core_lib/**`, `app/legacy/**`, `packaging/**`, `Makefile`, `flake.nix`, `install.sh`, workflow CI.*
