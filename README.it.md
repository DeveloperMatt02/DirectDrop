<div align="center">
  <a href="README.md"><img src="https://img.shields.io/badge/English-EN-555?style=flat-square" alt="English" /></a>
  <img src="https://img.shields.io/badge/Italiano-IT-blue?style=flat-square" alt="Italiano" />
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
  <p><strong>Quick Share (Nearby Share) per Linux che non richiede entrambi i dispositivi sulla stessa rete Wi-Fi — ora con una GUI desktop.</strong></p>

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
  <a href="#installazione-rapida-terminale"><img src="https://img.shields.io/badge/🐧_Quick_Install-22c55e?style=for-the-badge" alt="quick install" /></a>
  <a href="#schermate"><img src="https://img.shields.io/badge/🖥️_Screenshots-a855f7?style=for-the-badge" alt="screenshots" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/issues/new"><img src="https://img.shields.io/badge/🐛_Report_Bug-f59e0b?style=for-the-badge" alt="report bug" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/badge/⭐_Star-facc15?style=for-the-badge&logo=github" alt="star" /></a>
</div>

> Un fork di `quickshare-direct` upstream. Lo stack headless di ricezione/invio è conservato invariato (primo contatto via advertisement BLE/GATT, upgrade a Wi-Fi Direct GO/client, fallback LAN, helper polkit, demone CLI headless). Ciò che questo fork aggiunge è una GUI desktop Tauri riprogettata in `app/main/` con piena parità comportamentale rispetto alla CLI.

## Installazione rapida (Terminale)

One-liner universale — rileva la distribuzione e installa il pacchetto corrispondente
dall'ultima release di `DeveloperMatt02/DirectDrop` (`.rpm` su Fedora/RHEL, `.deb` su
Debian/Ubuntu, `.AppImage` in `~/.local/bin` in tutti gli altri casi):

```bash
curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash
```

Comandi diretti per distribuzione (stessa origine, senza script di supporto):

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

Oppure scegli manualmente il file dalla [ultima release](https://github.com/DeveloperMatt02/DirectDrop/releases/latest).

## 🖥️ Schermate

Finestra principale (console in modalità operativa: intestazione di stato, flusso live laptop ⇄ telefono, pannello di invio, scheda dei dispositivi, coda dei trasferimenti):

![DirectDrop main window](docs/screenshots/gui-main.svg)

Trasferimento in entrata — scheda di verifica PIN UKEY2 con conto alla rovescia di 60 s e rifiuto automatico:

![DirectDrop consent card](docs/screenshots/gui-consent.svg)

> [!NOTE]
> Si tratta di segnaposto schematici nello stile blueprint dell'app. Sostituisci `docs/screenshots/gui-main.svg` e `docs/screenshots/gui-consent.svg` con acquisizioni reali (`pnpm tauri dev`, fotografa la finestra, salva come `.png` accanto a essi e aggiorna i collegamenti qui sopra).

La guida rapida upstream resta valida:

1. **Rilevamento + primo contatto via Bluetooth LE** (funziona con il telefono fuori dal Wi-Fi).
2. **Handshake cifrato** (UKEY2, PIN mostrato su entrambi i lati) su quel collegamento Bluetooth.
3. **Passaggio alla velocità Wi-Fi**: il laptop crea un **gruppo Wi-Fi Direct** accanto alla normale connessione Wi-Fi (che continua a funzionare) e passa le credenziali al telefono; oppure, se il telefono è sulla stessa rete Wi-Fi, il trasferimento passa alla rete locale. Un semplice hotspot è il ripiego.
4. **Notifiche desktop**: Accetta/Rifiuta (con il PIN), poi «Apri cartella».

| | Stato |
|---|---|
| Ricezione da Android (testato upstream: Pixel 10, Android 16) | funzionante |
| Telefono fuori dal Wi-Fi → Wi-Fi Direct (75 MB, ~5 MB/s) | funzionante |
| Telefono sulla stessa rete Wi-Fi → LAN | funzionante |
| Primo contatto via Bluetooth Classic | sperimentale (`QSD_BT_CLASSIC=1`) |
| Invio ad Android → Wi-Fi Direct del telefono (300 MB, ~36 MB/s) | funzionante (`quickshare-direct send`, file manager, pannello di invio della GUI) |
| Pacchetti: Debian/Ubuntu, Fedora, Arch, generico (x86_64, aarch64) | installazione con una riga, vedi sotto |

---

## Cosa introduce questo fork: GUI desktop riprogettata e riattivata (`app/main/`)

Il repository upstream conserva la vecchia app Tauri di rQuickShare sotto `app/`, ma non la compila, non la distribuisce e non la testa (il suo percorso di invio è solo LAN/mDNS, senza il percorso Bluetooth / Wi-Fi Direct). Questo fork riattiva quell'idea come una nuova GUI dedicata in `app/main/`, che pilota lo **stesso** motore `core_lib` (`rqs_lib`) usato dalla CLI — nessuna implementazione parallela del protocollo.

### Stack e aspetto

- **Tauri 2.2, Vue 3.5, Pinia 2.x, Tailwind CSS 3.4.** Il punto d'ingresso del frontend è `app/main/src/App.vue`; lo stato vive negli store Pinia (`src/stores/session.ts`, `devices.ts`, `transfers.ts`).
- **Estetica blueprint tecnica:** console scura in modalità operativa, metriche monospaziate tabellari, diagramma live del flusso di rete tra laptop e telefono (`FlowDiagram.vue` / `FlowHero.vue`), linee sottili da 1px, nessun orpello commerciale. Layout: `StatusHeader` → `FlowDiagram` → `SendPane` + `DeviceBoard` → `TransferQueue`, con un sottile piè di pagina con i dettagli del ricevitore (percorso di download, porta).

### Bridge Rust in-process non invasivo

La GUI esegue `rqs_lib` in-process. Il confine è esattamente costituito da due nuovi file più un contratto versionato — **zero modifiche al motore del protocollo centrale**:

- `app/main/src-tauri/src/dto.rs` — l'unico formato wire. I tipi di dominio centrali in `core_lib` deliberatamente non derivano `Serialize`/`Deserialize`; ogni payload Tauri `invoke`/`emit` passa attraverso questi DTO serde con mappature `From` esplicite. `DTO_VERSION = 1`, verificato dal test Rust `dto_version_matches_ts` rispetto allo specchio TypeScript.
- `app/main/src-tauri/src/bridge.rs` — si iscrive ai canali `broadcast`/`watch` del core (`message_sender`, `dch_sender`, `visibility_sender`, `ble_receiver`) e riemette eventi Tauri limitati e deduplicati. Gli eventi di avanzamento sono limitati (emessi al cambio di stato, oppure con delta ≥256 KiB, oppure ≥500 ms; gli stati terminali sono sempre emessi). Gli overflow `Lagged` del broadcast vengono saltati con un debug log, mai con un crash — stessa politica della CLI.
- `app/main/src/types/qsd.ts` — specchio TypeScript rigoroso di `dto.rs` (nomi degli eventi, `VisibilityDto`, `TransferStateDto` con tutti i 18 stati del core, `PayloadKindDto`, `TransferDto`, `EndpointDto`, `SendPayloadDto`, `ConsentDto`, `SnapshotDto`, codici di errore). Include gli helper `humanSize` / `describeTransfer` (port del `describe()` della CLI), `baseName` e `validateSendFiles`, più gli insiemi `TERMINAL_STATES` / `ACTIVE_PROGRESS_STATES` / `HANDSHAKE_STATES`.

Comandi Tauri esposti (`src-tauri/src/cmds/`, collegati in `main.rs`): `qsd_snapshot`, `qsd_set_visibility`, `qsd_set_download_path`, `qsd_set_device_name`, `qsd_start_discovery`, `qsd_stop_discovery`, `qsd_consent`, `qsd_send_lan`, `qsd_get_hostname` (più gli shim legacy `get_hostname` / `change_download_path`). Eventi dal backend al frontend: `qsd://transfer`, `qsd://endpoints`, `qsd://visibility`, `qsd://ble-presence`.

### Piena parità comportamentale con la CLI

- **Scheda istantanea di verifica PIN UKEY2** (`ConsentCard.vue`): nome del mittente, tipo di dispositivo, PIN, riepilogo del payload ed elenco dei file compaiono non appena il core raggiunge `WaitingForUserConsent`. Un anello di **conto alla rovescia di 60 secondi** (`QSD_CONSENT_TIMEOUT_MS`, nel rispetto dell'override d'ambiente `QSD_CONSENT_TIMEOUT` letto in `dto.rs`) fa scadere la scheda; la scadenza attiva lo stesso **rifiuto automatico** (`ConsentDecline` sul canale) eseguito dalla CLI. Accetta/Rifiuta/Annulla corrispondono 1:1 alle `TransferAction` del core tramite `ConsentDto::into_channel_message`.
- **Selettore di visibilità a 3 stati** (`StatusHeader.vue` → `qsd_set_visibility`): `Visible`, `Temporarily` (60 s, con punto pulsante ambra e visualizzazione del tempo residuo), `Hidden` (`Invisible`). Il timer Temporarily→Invisible vive nel `mdns.rs` del core; la GUI riflette soltanto l'evento `qsd://visibility` e lo rende persistente tramite lo store Tauri. Nessuna reimplementazione del timer lato GUI.
- **Banner passivo di avviso di presenza BLE:** gli avvistamenti del listener BLE del core arrivano come eventi `qsd://ble-presence` limitati (120 s). Quando nascosta (`Invisible`), la bridge attiva inoltre una notifica di sistema che suggerisce una finestra di visibilità temporanea; il frontend mostra un toast informativo («Un telefono vicino sta condividendo — controlla la visibilità»). Rispetta la semantica del core `QSD_BLE_SCAN` / `QSD_NO_BLE_SCAN`.
- **Gestione inline dei trasferimenti di testo/URL senza scritture forzate su disco** (`TransferCard.vue`): i payload `Text`, `Url` e `WiFi` sono resi inline con anteprima più copia negli appunti (plugin clipboard-manager di Tauri); nessun file viene scritto su disco per visualizzarli. I payload di file mostrano conteggio dei file RX, nomi e avanzamento preciso al byte (`ack_bytes` / `total_bytes`).
- **Invio di file LAN/mDNS in uscita con validazione dei file regolari** (`SendPane.vue` + `qsd_send_lan`): prepara i file tramite finestra di dialogo o trascinamento, scegli un endpoint mDNS rilevato (`EndpointDto`, `addr = ip:port`) e invia attraverso il canale `sender_file` del core come `SendInfo { ob: Files }`. La validazione corrisponde a quella della CLI: selezioni vuote e directory rifiutate con errori `VALIDATION` (`{f}: not a regular file (folders aren't supported)`); il frontend fallisce in anticipo sui casi evidenti prima di invocare il backend.

### Eseguire la GUI dai sorgenti

```bash
cd app/main
pnpm install
pnpm tauri dev     # live GUI against local core_lib (path dep: ../../core_lib)
pnpm ts-check       # vue-tsc --noEmit contract check
pnpm test           # vitest suite (tests/)
pnpm build          # production Tauri bundle
```

Lato Rust: `cd app/main/src-tauri && cargo check` / `cargo test` (include i test del formato wire DTO). I prerequisiti di sistema sono gli stessi del demone (BlueZ, NetworkManager, header libdbus, protoc, cargo ≥ 1.85) più Node/pnpm e le dipendenze Linux di Tauri 2.x.

---

## Architettura e genealogia / Crediti

Vista a strati di questo fork:

```
Android phone ──BLE adv / GATT──▶ core_lib (rqs_lib) ◀──Tauri bridge──▶ Vue GUI
                                         │  (unchanged engine)
                                         ▼
                                Wi-Fi Direct GO / client,
                                LAN, hotspot fallback,
                                polkit Wi-Fi helpers
```

- **Autore/i di `quickshare-direct` upstream:** il trasporto fuori rete centrale da cui questo fork dipende — monitoraggio degli advertisement BLE e server GATT per rilevamento/primo contatto, upgrade a gruppo proprietario / client Wi-Fi Direct accanto alla normale connessione Wi-Fi (con gestione del fallback a singolo canale), gli helper root on-demand `quickshare-ap@USER` / `quickshare-join@USER` con la loro regola polkit, e il demone CLI headless `quickshare-direct` con notifiche desktop. Tutto il merito ingegneristico di protocollo, Bluetooth e Wi-Fi appartiene a loro; questo fork non ne modifica nulla.
- **`rQuickShare` di Martichou (e collaboratori come nozwock e martinalderson):** l'implementazione Rust/Tauri fondamentale del protocollo Nearby Share / Quick Share su Linux su cui si basano sia upstream che questo fork — inclusi il core `rqs_lib`, la struttura originaria dell'app Tauri e il lavoro di rilevamento Bluetooth proseguito in `core_lib`.
- **Google Nearby (Apache-2.0):** la specifica del protocollo e le definizioni protobuf che definiscono i frame UKEY2 di handshake, connessione, introduzione e payload. La licenza di questo repository resta GPL-3.0, come rQuickShare.

---

### Installazione (demone + pacchetti CLI)

Debian 12+ / Ubuntu 22.04+ (e Mint, Pop!_OS, ...), Fedora, Arch (e Manjaro, EndeavourOS, ...),
altri tramite build generica; x86_64 e aarch64:

```
curl -fsSL https://raw.githubusercontent.com/papshmeare/quickshare-direct/main/install.sh | sh
```

Installa l'ultima [release](https://github.com/papshmeare/quickshare-direct/releases) con
il tuo gestore di pacchetti (`.deb`, `.rpm`, `.pkg.tar.zst`; le dipendenze provengono dalla tua distribuzione),
apre le porte Quick Share se firewalld o ufw è attivo e avvia il ricevitore. Esegui di nuovo la stessa
riga per aggiornare; `curl -fsSL .../install.sh | sh -s -- --uninstall` lo rimuove. Preferisci
vedere prima cosa succede? Scarica `install.sh` e leggilo, oppure installa manualmente il pacchetto dalla
pagina della release (`sudo apt install ./quickshare-direct_amd64.deb`,
`sudo dnf install ./quickshare-direct.x86_64.rpm`,
`sudo pacman -U quickshare-direct-x86_64.pkg.tar.zst`).

Cosa viene installato:
- `quickshare-direct`: il ricevitore (un servizio utente in ogni sessione grafica, con
  notifiche) e i comandi `send` / `devices`;
- due piccoli helper root, avviati su richiesta tramite systemd (`quickshare-ap@USER` crea il
  collegamento Wi-Fi diretto in ricezione, `quickshare-join@USER` si unisce a quello del telefono in invio), e una
  regola polkit (`packaging/linux/polkit/50-quickshare-direct.rules`) che consente a un utente locale attivo
  di avviare/fermare solo le proprie istanze (`start`/`stop`/`restart` su
  `quickshare-ap@USER.service` / `quickshare-join@USER.service`);
- un drop-in di NetworkManager che lascia in pace le interfacce di collegamento temporanee, un'unità che mantiene
  il Bluetooth collegabile, definizioni di servizi firewalld/ufw e voci del file manager.

Richiede: systemd, BlueZ, NetworkManager con wpa_supplicant (predefinito su queste distribuzioni;
non iwd) e una scheda Wi-Fi in grado di gestire una seconda interfaccia accanto alla normale connessione (la maggior parte
delle schede Intel/MediaTek/Qualcomm). Le notifiche richiedono un demone che mostri pulsanti di azione (GNOME,
KDE, swaync, mako, ...). Il polkit più vecchio di Ubuntu 22.04 non consente agli utenti di avviare gli helper, quindi lì
funzionano solo Bluetooth e i trasferimenti sulla stessa rete. Imposta il dominio regolatorio Wi-Fi
(`sudo iw reg set XX`): con il dominio world predefinito Linux non avvierà un group owner a 5 GHz.
Impostazioni: `~/.config/quickshare-direct/env` (`QSD_NAME=...`, `QSD_DIR=...`) per il ricevitore,
`/etc/default/quickshare-direct` per gli helper. Molti chip Wi-Fi non possono ospitare il collegamento diretto su un
secondo canale accanto alla normale connessione, il che conta quando il router usa un canale che un
laptop potrebbe non poter ospitare (canali radar/DFS come il 100): allora il laptop lascia la rete Wi-Fi
per i pochi secondi del trasferimento e si riconnette dopo (`QS_SINGLE_CHANNEL=auto`; `always`
per la massima velocità anche in invio, `never` per mantenere la connessione e ripiegare sul Bluetooth).

Dai sorgenti: `make && sudo make install` (richiede cargo ≥ 1.85, protoc, header libdbus), poi
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
Questo configura il ricevitore (servizio utente nella sessione grafica), l'helper root che crea
il collegamento Wi-Fi diretto su richiesta (`quickshare-ap.service`, avviabile da quell'utente via polkit),
l'helper che si unisce al gruppo Wi-Fi Direct di un telefono in invio (`quickshare-join.service`),
le porte del firewall, il Bluetooth «collegabile» e tiene NetworkManager lontano dalle interfacce di collegamento.
Imposta il dominio regolatorio Wi-Fi (`iw reg set XX`): con il dominio world predefinito Linux non
avvierà un access point / group owner a 5 GHz.

### Invio (CLI)

Sul telefono, apri Quick Share e tocca **Ricevi** (o rendilo visibile a tutti), poi:

```
quickshare-direct send photo.jpg notes.pdf      # to the one phone in receive mode nearby
quickshare-direct send --to Pixel video.mp4     # pick by name when several are around
quickshare-direct devices                       # list phones in receive mode
```
Trova il telefono via Bluetooth, mostra il PIN e, una volta accettato sul telefono, i file viaggiano
sul gruppo Wi-Fi Direct del telefono (unito su una seconda interfaccia, il tuo Wi-Fi resta connesso).
Nel file manager: **Invia a → Telefono (Quick Share)** di Thunar (qualsiasi file), oppure Apri con →
«Invia con Quick Share» (tipi di file comuni); riporta tramite notifiche. Il
`SendPane` della GUI esegue lo stesso invio LAN/mDNS tramite `qsd_send_lan` con identici PIN e
semantica dei file regolari.

### Variabili d'ambiente (ricevitore, mittente, helper)

Il comportamento del core è configurato tramite ambiente (vedi `core_lib/src/bin/quickshare-direct.rs`,
`core_lib/src/hdl/*.rs`):

| Variable | Default | Effect |
|---|---|---|
| `QSD_DIR` | XDG Downloads, else `~/Downloads` | cartella di download |
| `QSD_NAME` | hostname | nome del dispositivo mostrato sul telefono |
| `QSD_PORT` | random | porta TCP per i trasferimenti in entrata |
| `QSD_BWU_PORT` | random | porta TCP per il listener di upgrade Wi-Fi |
| `QSD_CONSENT_TIMEOUT` | `60` (s) | secondi per rispondere Accetta/Rifiuta prima del rifiuto automatico; guida anche il conto alla rovescia della GUI |
| `QSD_BWU` | `auto` | preferenza di upgrade in entrata: `lan` / `hotspot` ne forza uno |
| `QSD_BLE_SCAN` | passive BlueZ advertisement monitor | `discovery` = discovery LE permanente (potrebbe impedire la riconnessione di mouse/tastiere LE associati); `off` disabilita il listener |
| `QSD_NO_BLE_SCAN` | unset | quando impostata, disabilita completamente il rilevamento BLE del mittente |
| `QSD_BT_CLASSIC` | unset | quando impostata (`=1`), abilita il primo contatto sperimentale via Bluetooth Classic |
| `QSD_BLE_L2CAP` / `QSD_L2CAP_CHUNK` | on / default | ottimizzazione BLE L2CAP del percorso di invio (`=0` forza il socket weave) |
| `QSD_WEAVE_ATT` / `QSD_WEAVE_PACKET` | on / MTU default | ottimizzazione del trasporto GATT/weave per esperimenti |
| `QSD_SEND_MEDIUMS` | wifi_lan-first default | preferenza del medium in uscita separata da virgole (`wifi_lan`, ...) |
| `QS_SINGLE_CHANNEL` | `auto` | `always` = massima velocità in invio (potrebbe interrompere la stazione); `never` = mantiene la connessione, ripiega sul Bluetooth |
| `QSD_AP_IFACE` / `QSD_STA_IFACE` | `ap0` / auto | nomi delle interfacce di collegamento per l'helper AP |
| `QSD_AP_UNIT` / `QSD_JOIN_UNIT` | `quickshare-ap` / `quickshare-join` | nomi delle unità systemd per gli helper root |
| `QSD_SYSTEMCTL` | `systemctl` | override del comando di avvio degli helper |
| `QSD_AP_CREDENTIALS` | `/run/quickshare/credentials` | percorso di deposito delle credenziali per l'helper AP |
| `QSD_JOIN_REQUEST` / `QSD_JOIN_RESULT` | `/run/quickshare-join/...` | percorsi di richiesta/risultato per l'helper di join |
| `RUST_LOG` | `info` (`warn` for send/devices) | filtro dei log |

File del ricevitore: `~/.config/quickshare-direct/env` (`QSD_NAME`, `QSD_DIR`, ...). File degli helper:
`/etc/default/quickshare-direct`.

### Come funziona / note di ricerca

[docs/DEV_NOTES.md](docs/DEV_NOTES.md) (scoperte, log dei telefoni, dettagli del protocollo) e
[docs/BLE_RECEIVER_DISCOVERY.md](docs/BLE_RECEIVER_DISCOVERY.md) (il percorso di rilevamento del ricevitore BLE).

Release (questo fork): spingere un tag `v*.*.*` esegue [`.github/workflows/release.yml`](.github/workflows/release.yml), che compila i bundle della GUI Tauri (`.deb`, `.rpm`, `.AppImage`) e il tarball del demone `directdrop-cli-linux-x86_64.tar.gz` e li pubblica nella GitHub Release — installabili tramite `get-directdrop.sh` qui sopra. I pacchetti del demone upstream mantengono il proprio flusso: CI (`.github/workflows/packages.yml`) compila e installa-testa i pacchetti a ogni push; incrementare `VERSION` su `main` pubblica la release `qsd-v<VERSION>`.

Layout del repository: `core_lib` (libreria + binario `quickshare-direct`) e `packaging/`; questo è tutto
ciò che i pacchetti, il modulo NixOS e la CI compilano. `app/main/` è la GUI desktop riattivata di questo fork
(Tauri + Vue): avvolge `core_lib` in-process tramite `src-tauri/src/dto.rs` + `bridge.rs` e il
contratto `src/types/qsd.ts`, senza toccare il core. Altri resti upstream (`snap/`, `BUILD.md`,
`CHANGELOG.md`, release-please, i workflow `build`/`lint`) si riferiscono ancora al `master` di upstream.

Sviluppo (headless): `nix develop`, poi in `core_lib`: `cargo run --bin quickshare-direct`
(env: `QSD_DIR`, `QSD_NAME`, `QSD_PORT`, `QSD_BWU_PORT`, `QSD_BWU=lan|hotspot`, `RUST_LOG`).
Sviluppo (GUI): vedi «Eseguire la GUI dai sorgenti» qui sopra.

---

<div align="center">

## ⭐ Mostra il tuo supporto

Se DirectDrop porta i tuoi file al telefono senza toccare il router, dagli una stella — aiuta molto.

<a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=social" alt="star this repo" /></a>

<a href="https://star-history.com/#DeveloperMatt02/DirectDrop&Date"><img src="https://api.star-history.com/svg?repos=DeveloperMatt02/DirectDrop&type=Date" width="600" alt="star history chart" /></a>

Hai trovato un bug? [Apri una issue](https://github.com/DeveloperMatt02/DirectDrop/issues/new) — le pull request sono benvenute.

</div>
