<div align="center">
  <img src="https://img.shields.io/badge/Deutsch-DE-blue?style=flat-square" alt="Deutsch" />
  <a href="README.md"><img src="https://img.shields.io/badge/English-EN-555?style=flat-square" alt="English" /></a>
  <a href="README.it.md"><img src="https://img.shields.io/badge/Italiano-IT-555?style=flat-square" alt="Italiano" /></a>
  <a href="README.es.md"><img src="https://img.shields.io/badge/Español-ES-555?style=flat-square" alt="Español" /></a>
  <a href="README.fr.md"><img src="https://img.shields.io/badge/Français-FR-555?style=flat-square" alt="Français" /></a>
  <a href="README.pt.md"><img src="https://img.shields.io/badge/Português-PT-555?style=flat-square" alt="Português" /></a>
  <a href="README.ru.md"><img src="https://img.shields.io/badge/Русский-RU-555?style=flat-square" alt="Русский" /></a>
  <a href="README.zh.md"><img src="https://img.shields.io/badge/中文-ZH-555?style=flat-square" alt="中文" /></a>
  <a href="README.ja.md"><img src="https://img.shields.io/badge/日本語-JA-555?style=flat-square" alt="日本語" /></a>
</div>

<div align="center">
  <img src="app/main/src-tauri/icons/128x128.png" width="128" alt="DirectDrop icon" />
  <h1>DirectDrop</h1>
  <p><strong>Quick Share (Nearby Share) für Linux, das nicht erfordert, dass beide Geräte im selben WLAN sind — jetzt mit Desktop-GUI.</strong></p>

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
  <a href="#schnellinstallation-terminal"><img src="https://img.shields.io/badge/🐧_Quick_Install-22c55e?style=for-the-badge" alt="quick install" /></a>
  <a href="#bildschirmfotos"><img src="https://img.shields.io/badge/🖥️_Screenshots-a855f7?style=for-the-badge" alt="screenshots" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/issues/new"><img src="https://img.shields.io/badge/🐛_Report_Bug-f59e0b?style=for-the-badge" alt="report bug" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/badge/⭐_Star-facc15?style=for-the-badge&logo=github" alt="star" /></a>
</div>

> Ein Fork von Upstream-`quickshare-direct`. Der Headless-Empfänger-/Sender-Stack bleibt unverändert erhalten (BLE-Advertisement/GATT-Erstkontakt, Wi-Fi-Direct-GO/Client-Upgrade, LAN-Fallback, Polkit-Helfer, Headless-CLI-Daemon). Was dieser Fork hinzufügt, ist eine neu gestaltete Tauri-Desktop-GUI in `app/main/` mit vollständiger Verhaltensparität zur CLI.

## Schnellinstallation (Terminal)

Universeller Einzeiler — erkennt die Distribution und installiert das passende Paket aus dem
neuesten `DeveloperMatt02/DirectDrop`-Release (`.rpm` auf Fedora/RHEL, `.deb` auf
Debian/Ubuntu, `.AppImage` nach `~/.local/bin` überall sonst):

```bash
curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash
```

Direkte Befehle pro Distribution (gleiche Quelle, ohne Hilfsskript):

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

Oder wähle das Paket manuell aus dem [neuesten Release](https://github.com/DeveloperMatt02/DirectDrop/releases/latest).

## 🖥️ Bildschirmfotos

Hauptfenster (Operate-Mode-Konsole: Statuskopfzeile, Live-Laptop-⇄-Telefon-Fluss, Sendebereich, Geräteboard, Übertragungswarteschlange):

## GUI oder Headless-Daemon: entscheide dich für eines

Die GUI-Pakete (`.deb` / `.rpm` / `.AppImage`) enthalten ihren eigenen Empfänger; das
`quickshare-direct`-Daemon-Paket ist derselbe Empfänger im Headless-Betrieb. Werden beide
gleichzeitig ausgeführt, kündigt sich der Laptop zweimal unter demselben Gerätenamen an und sie
streiten sich um die benutzerspezifischen WLAN-Helfer (`quickshare-ap@USER`,
`quickshare-join@USER`), daher deaktiviert `get-directdrop.sh` den Daemon
(`systemctl --user disable --now quickshare-direct`), wann immer es ein GUI-Paket installiert. Die
`.deb`/`.rpm`-GUI fügt DirectDrop außerdem zum App-Menü hinzu (nach "DirectDrop" suchen); die
`.AppImage` erst nach Integration (Gear Lever, appimaged oder eine manuelle `.desktop`-Datei). Um zum
Headless-Betrieb zurückzukehren: GUI beenden und `systemctl --user enable --now quickshare-direct`
ausführen.

![DirectDrop main window](docs/screenshots/gui-main.svg)

Eingehende Übertragung — UKEY2-PIN-Verifizierungskarte mit 60-s-Countdown und automatischer Ablehnung:

![DirectDrop consent card](docs/screenshots/gui-consent.svg)

> [!NOTE]
> Dies sind schematische Platzhalter im Blueprint-Stil der App. Ersetze `docs/screenshots/gui-main.svg` und `docs/screenshots/gui-consent.svg` durch echte Aufnahmen (`pnpm tauri dev`, Fenster fotografieren, als `.png` daneben speichern und die Links oben aktualisieren).

Der Upstream-Schnellstart gilt weiterhin:

1. **Erkennung + Erstkontakt über Bluetooth LE** (funktioniert, wenn das Telefon nicht im WLAN ist).
2. **Verschlüsselter Handshake** (UKEY2, PIN auf beiden Seiten angezeigt) über diese Bluetooth-Verbindung.
3. **Upgrade auf WLAN-Geschwindigkeit**: Der Laptop erstellt eine **Wi-Fi-Direct-Gruppe** neben seiner normalen WLAN-Verbindung (die weiterhin funktioniert) und übergibt die Zugangsdaten an das Telefon; oder, wenn sich das Telefon im selben WLAN befindet, wechselt die Übertragung ins lokale Netzwerk. Ein einfacher Hotspot ist der Fallback.
4. **Desktop-Benachrichtigungen**: Annehmen/Ablehnen (mit der PIN), dann „Ordner öffnen“.

| | Status |
|---|---|
| Empfangen von Android (Upstream getestet: Pixel 10, Android 16) | funktioniert |
| Telefon ohne WLAN → Wi-Fi Direct (75 MB, ~5 MB/s) | funktioniert |
| Telefon im selben WLAN → LAN | funktioniert |
| Bluetooth-Classic-Erstkontakt | experimentell (`QSD_BT_CLASSIC=1`) |
| Senden an Android → Wi-Fi Direct des Telefons (300 MB, ~36 MB/s) | funktioniert (`quickshare-direct send`, Dateimanager, GUI-Sendebereich) |
| Pakete: Debian/Ubuntu, Fedora, Arch, generisch (x86_64, aarch64) | Einzeilen-Installation, siehe unten |

---

## Was dieser Fork einführt: reaktivierte & neu gestaltete Desktop-GUI (`app/main/`)

Das Upstream-Repository behält rQuickShares alte Tauri-App unter `app/`, baut, versendet oder testet sie aber nicht (ihr Sendepfad ist nur LAN/mDNS, ohne den Bluetooth-/Wi-Fi-Direct-Pfad). Dieser Fork reaktiviert diese Idee als neue, zweckgebaute GUI in `app/main/`, die dieselbe `core_lib`-Engine (`rqs_lib`) antreibt wie die CLI — keine parallele Protokollimplementierung.

### Stack und Aussehen

- **Tauri 2.2, Vue 3.5, Pinia 2.x, Tailwind CSS 3.4.** Frontend-Einstieg ist `app/main/src/App.vue`; der Zustand liegt in Pinia-Stores (`src/stores/session.ts`, `devices.ts`, `transfers.ts`).
- **Technische Blueprint-Ästhetik:** dunkle Operate-Mode-Konsole, tabellarische Monospace-Metriken, Live-Netzwerkflussdiagramm zwischen Laptop und Telefon (`FlowDiagram.vue` / `FlowHero.vue`), 1px-Haarlinien, kein Marketing-Ballast. Layout: `StatusHeader` → `FlowDiagram` → `SendPane` + `DeviceBoard` → `TransferQueue`, mit einer schmalen Empfängerdetails-Fußzeile (Downloadpfad, Port).

### Nicht-invasive In-Prozess-Rust-Brücke

Die GUI lässt `rqs_lib` in-prozess laufen. Die Grenze besteht aus genau zwei neuen Dateien plus einem versionierten Vertrag — **null Änderungen an der Kern-Protokoll-Engine**:

- `app/main/src-tauri/src/dto.rs` — das einzige Übertragungsformat. Kern-Domaintypen in `core_lib` leiten bewusst kein `Serialize`/`Deserialize` ab; jedes Tauri-`invoke`-/`emit`-Payload läuft über diese Serde-DTOs mit expliziten `From`-Abbildungen. `DTO_VERSION = 1`, erzwungen durch den Rust-Test `dto_version_matches_ts` gegen den TypeScript-Spiegel.
- `app/main/src-tauri/src/bridge.rs` — abonniert die Kern-`broadcast`-/`watch`-Kanäle (`message_sender`, `dch_sender`, `visibility_sender`, `ble_receiver`) und sendet gedrosselte/entduplizierte Tauri-Events erneut. Fortschritts-Events werden gedrosselt (Senden bei Zustandsänderung, oder ≥256-KiB-Delta, oder ≥500 ms; Endzustände werden immer gesendet). `Lagged`-Broadcast-Überläufe werden mit einem Debug-Log übersprungen, niemals ein Absturz — dieselbe Richtlinie wie bei der CLI.
- `app/main/src/types/qsd.ts` — strikter TypeScript-Spiegel von `dto.rs` (Eventnamen, `VisibilityDto`, `TransferStateDto` mit allen 18 Kernzuständen, `PayloadKindDto`, `TransferDto`, `EndpointDto`, `SendPayloadDto`, `ConsentDto`, `SnapshotDto`, Fehlercodes). Enthält die `humanSize`- / `describeTransfer`-Helfer (Portierung des CLI-`describe()`), `baseName` und `validateSendFiles`, plus die Mengen `TERMINAL_STATES` / `ACTIVE_PROGRESS_STATES` / `HANDSHAKE_STATES`.

Exponierte Tauri-Befehle (`src-tauri/src/cmds/`, verdrahtet in `main.rs`): `qsd_snapshot`, `qsd_set_visibility`, `qsd_set_download_path`, `qsd_set_device_name`, `qsd_start_discovery`, `qsd_stop_discovery`, `qsd_consent`, `qsd_send_lan`, `qsd_get_hostname` (plus alte `get_hostname`- / `change_download_path`-Shims). Events Backend → Frontend: `qsd://transfer`, `qsd://endpoints`, `qsd://visibility`, `qsd://ble-presence`.

### Vollständige CLI-Verhaltensparität

- **Sofortige UKEY2-PIN-Verifizierungskarte** (`ConsentCard.vue`): Absendername, Gerätetyp, PIN, Payload-Zusammenfassung und Dateiliste erscheinen, sobald der Kern `WaitingForUserConsent` erreicht. Ein **60-Sekunden-Countdown**-Ring (`QSD_CONSENT_TIMEOUT_MS`, unter Beachtung der in `dto.rs` gelesenen `QSD_CONSENT_TIMEOUT`-Env-Überschreibung) lässt die Karte ablaufen; Ablauf löst dieselbe **automatische Ablehnung** (`ConsentDecline` auf dem Kanal) aus, die die CLI durchführt. Annehmen/Ablehnen/Abbrechen entsprechen 1:1 den Kern-`TransferAction`s via `ConsentDto::into_channel_message`.
- **3-stufige Sichtbarkeitsauswahl** (`StatusHeader.vue` → `qsd_set_visibility`): `Visible`, `Temporarily` (60 s, mit bernsteinfarbenem Puls-Punkt und Restzeitanzeige), `Hidden` (`Invisible`). Der Temporarily→Invisible-Timer liegt im Kern-`mdns.rs`; die GUI spiegelt nur das `qsd://visibility`-Event und persistiert es via Tauri-Store. Keine GUI-seitige Timer-Neuimplementierung.
- **Passive BLE-Anwesenheitswarnbanner:** BLE-Sichtungen des Kern-Listeners treffen als gedrosselte (120 s) `qsd://ble-presence`-Events ein. Wenn versteckt (`Invisible`), löst die Brücke zusätzlich eine Systembenachrichtigung aus, die ein temporäres Sichtbarkeitsfenster vorschlägt; das Frontend zeigt einen Info-Toast („Ein Telefon in der Nähe teilt — prüfe die Sichtbarkeit“). Beachtet die Kern-Semantik von `QSD_BLE_SCAN` / `QSD_NO_BLE_SCAN`.
- **Inline-Text-/URL-Übertragungsbehandlung ohne erzwungene Plattenschreibvorgänge** (`TransferCard.vue`): `Text`-, `Url`- und `WiFi`-Payloads werden inline mit Vorschau plus Zwischenablage-Kopie dargestellt (Tauri-Clipboard-Manager-Plugin); es wird keine Datei auf die Platte geschrieben, um sie anzuzeigen. Datei-Payloads zeigen RX-Dateianzahl, Namen und bytegenauen Fortschritt (`ack_bytes` / `total_bytes`).
- **Ausgehendes LAN/mDNS-Datei-Senden mit Reguläre-Datei-Validierung** (`SendPane.vue` + `qsd_send_lan`): Dateien per Dialog oder Drag-and-drop bereitstellen, einen entdeckten mDNS-Endpunkt wählen (`EndpointDto`, `addr = ip:port`) und über den Kern-`sender_file`-Kanal als `SendInfo { ob: Files }` senden. Die Validierung entspricht der CLI: leere Auswahl und Verzeichnisse werden mit `VALIDATION`-Fehlern abgelehnt (`{f}: not a regular file (folders aren't supported)`); das Frontend scheitert bei offensichtlichen Fällen schnell, bevor es das Backend aufruft.

### Die GUI aus dem Quellcode starten

```bash
cd app/main
pnpm install
pnpm tauri dev     # live GUI against local core_lib (path dep: ../../core_lib)
pnpm ts-check       # vue-tsc --noEmit contract check
pnpm test           # vitest suite (tests/)
pnpm build          # production Tauri bundle
```

Rust-Seite: `cd app/main/src-tauri && cargo check` / `cargo test` (einschließlich der DTO-Wire-Format-Tests). Systemvoraussetzungen sind dieselben wie beim Daemon (BlueZ, NetworkManager, libdbus-Header, protoc, cargo ≥ 1.85) plus Node/pnpm und Tauri-2.x-Linux-Abhängigkeiten.

---

## Architektur & Genealogie / Danksagungen

Geschichtete Ansicht dieses Forks:

```
Android phone ──BLE adv / GATT──▶ core_lib (rqs_lib) ◀──Tauri bridge──▶ Vue GUI
                                         │  (unchanged engine)
                                         ▼
                                Wi-Fi Direct GO / client,
                                LAN, hotspot fallback,
                                polkit Wi-Fi helpers
```

- **Upstream-`quickshare-direct`-Autor(en):** der Kern-Off-Netzwerk-Transport, von dem dieser Fork abhängt — BLE-Advertisement-Überwachung und GATT-Server für Erkennung/Erstkontakt, Wi-Fi-Direct-Group-Owner-/Client-Upgrade neben der normalen WLAN-Verbindung (mit Einzelkanal-Fallback-Behandlung), die `quickshare-ap@USER`- / `quickshare-join@USER`-On-Demand-Root-Helfer mit ihrer Polkit-Regel und der Headless-`quickshare-direct`-CLI-Daemon mit Desktop-Benachrichtigungen. Alle Protokoll-, Bluetooth- und WLAN-Ingenieursleistung gebührt hierher; dieser Fork ändert nichts daran.
- **`rQuickShare` von Martichou (und Mitwirkenden wie nozwock und martinalderson):** die grundlegende Rust/Tauri-Implementierung des Nearby-Share-/Quick-Share-Protokolls auf Linux, auf der sowohl Upstream als auch dieser Fork aufbauen — einschließlich des `rqs_lib`-Kerns, der ursprünglichen Tauri-App-Struktur und der Bluetooth-Erkennungsarbeit, die in `core_lib` weitergeführt wird.
- **Google Nearby (Apache-2.0):** die Protokollspezifikation und Protobuf-Definitionen, die den UKEY2-Handshake, Verbindungs-, Vorstellungs- und Payload-Frames definieren. Die Lizenz dieses Repositorys bleibt GPL-3.0, wie rQuickShare.

---

### Installieren (Daemon- + CLI-Pakete)

Debian 12+ / Ubuntu 22.04+ (und Mint, Pop!_OS, ...), Fedora, Arch (und Manjaro, EndeavourOS, ...),
andere via generischem Build; x86_64 und aarch64:

```
curl -fsSL https://raw.githubusercontent.com/papshmeare/quickshare-direct/main/install.sh | sh
```

Es installiert das neueste [Release](https://github.com/papshmeare/quickshare-direct/releases) mit
deinem Paketmanager (`.deb`, `.rpm`, `.pkg.tar.zst`; Abhängigkeiten kommen aus deiner Distribution),
öffnet die Quick-Share-Ports, wenn firewalld oder ufw aktiv ist, und startet den Empfänger. Führe dieselbe
Zeile erneut aus, um zu aktualisieren; `curl -fsSL .../install.sh | sh -s -- --uninstall` entfernt es. Lieber erst
sehen, was passiert? Lade `install.sh` herunter und lies es, oder installiere das Paket von der
Release-Seite von Hand (`sudo apt install ./quickshare-direct_amd64.deb`,
`sudo dnf install ./quickshare-direct.x86_64.rpm`,
`sudo pacman -U quickshare-direct-x86_64.pkg.tar.zst`).

Was installiert wird:
- `quickshare-direct`: der Empfänger (ein Benutzerdienst in jeder grafischen Sitzung, mit
  Benachrichtigungen) und die `send`- / `devices`-Befehle;
- zwei kleine Root-Helfer, bei Bedarf über systemd gestartet (`quickshare-ap@USER` erstellt die
  direkte WLAN-Verbindung beim Empfangen, `quickshare-join@USER` tritt der des Telefons beim Senden bei), und eine
  Polkit-Regel (`packaging/linux/polkit/50-quickshare-direct.rules`), die einem aktiven lokalen
  Benutzer erlaubt, nur seine eigenen Instanzen zu starten/stoppen (`start`/`stop`/`restart` auf
  `quickshare-ap@USER.service` / `quickshare-join@USER.service`);
- ein NetworkManager-Drop-in, das die temporären Link-Schnittstellen in Ruhe lässt, eine Unit, die
  Bluetooth verbindbar hält, firewalld/ufw-Dienstdefinitionen und Dateimanager-Einträge.

Braucht: systemd, BlueZ, NetworkManager mit wpa_supplicant (Standard auf diesen Distributionen;
nicht iwd) und eine WLAN-Karte, die eine zweite Schnittstelle neben der normalen Verbindung betreiben kann (die meisten
Intel/MediaTek/Qualcomm-Karten). Benachrichtigungen brauchen einen Daemon, der Aktionsschaltflächen zeigt (GNOME,
KDE, swaync, mako, ...). Ubuntus 22.04 älteres Polkit kann Benutzer die Helfer nicht starten lassen, daher funktionieren dort
nur Bluetooth- und Gleichnetz-Übertragungen. Setze deine WLAN-Regulierungsdomäne
(`sudo iw reg set XX`): Mit der Standard-Welt-Domäne startet Linux keinen Group Owner auf 5 GHz.
Einstellungen: `~/.config/quickshare-direct/env` (`QSD_NAME=...`, `QSD_DIR=...`) für den Empfänger,
`/etc/default/quickshare-direct` für die Helfer. Viele WLAN-Chips können die Direktverbindung nicht auf einem
zweiten Kanal neben deiner normalen Verbindung hosten, was wichtig wird, wenn dein Router einen Kanal nutzt, den ein
Laptop möglicherweise nicht hosten darf (Radar/DFS-Kanäle wie 100): Dann verlässt der Laptop dein WLAN-Netzwerk
für die wenigen Sekunden der Übertragung und verbindet sich danach erneut (`QS_SINGLE_CHANNEL=auto`; `always`
für volle Geschwindigkeit auch beim Senden, `never`, um die Verbindung zu halten und auf Bluetooth zurückzufallen).

Aus dem Quellcode: `make && sudo make install` (braucht cargo ≥ 1.85, protoc, libdbus-Header), dann
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
Dies richtet den Empfänger ein (Benutzerdienst in der grafischen Sitzung), den Root-Helfer, der die
direkte WLAN-Verbindung bei Bedarf erstellt (`quickshare-ap.service`, startbar durch diesen Benutzer via Polkit),
den Helfer, der der Wi-Fi-Direct-Gruppe eines Telefons beim Senden beitritt (`quickshare-join.service`),
Firewall-Ports, Bluetooth-„verbindbar“ und hält NetworkManager von den Link-Schnittstellen fern.
Setze deine WLAN-Regulierungsdomäne (`iw reg set XX`): Mit der Standard-Welt-Domäne startet Linux keinen
Access Point / Group Owner auf 5 GHz.

### Senden (CLI)

Öffne auf dem Telefon Quick Share und tippe auf **Empfangen** (oder mache es für alle sichtbar), dann:

```
quickshare-direct send photo.jpg notes.pdf      # to the one phone in receive mode nearby
quickshare-direct send --to Pixel video.mp4     # pick by name when several are around
quickshare-direct devices                       # list phones in receive mode
```
Es findet das Telefon über Bluetooth, zeigt die PIN, und sobald du auf dem Telefon annimmst, gehen die Dateien
über die Wi-Fi-Direct-Gruppe des Telefons (beigetreten auf einer zweiten Schnittstelle, dein WLAN bleibt verbunden).
Im Dateimanager: Thunars **Senden an → Telefon (Quick Share)** (jede Datei), oder Öffnen mit →
„Senden mit Quick Share“ (gängige Dateitypen); es meldet über Benachrichtigungen. Die GUI-
`SendPane` führt dasselbe LAN/mDNS-Senden über `qsd_send_lan` mit identischer PIN- und
Reguläre-Datei-Semantik durch.

### Umgebungsvariablen (Empfänger, Sender, Helfer)

Das Kernverhalten wird über Umgebung konfiguriert (siehe `core_lib/src/bin/quickshare-direct.rs`,
`core_lib/src/hdl/*.rs`):

| Variable | Default | Effect |
|---|---|---|
| `QSD_DIR` | XDG Downloads, else `~/Downloads` | Download-Ordner |
| `QSD_NAME` | hostname | Gerätename, der auf dem Telefon angezeigt wird |
| `QSD_PORT` | random | TCP-Port für eingehende Übertragungen |
| `QSD_BWU_PORT` | random | TCP-Port für den WLAN-Upgrade-Listener |
| `QSD_CONSENT_TIMEOUT` | `60` (s) | Sekunden, um Annehmen/Ablehnen zu beantworten, bevor automatisch abgelehnt wird; steuert auch den GUI-Countdown |
| `QSD_BWU` | `auto` | eingehende Upgrade-Präferenz: `lan` / `hotspot` erzwingt eine |
| `QSD_BLE_SCAN` | passive BlueZ advertisement monitor | `discovery` = permanente LE-Erkennung (kann blockieren, dass gekoppelte LE-Mäuse/Tastaturen sich erneut verbinden); `off` deaktiviert den Listener |
| `QSD_NO_BLE_SCAN` | unset | wenn gesetzt, deaktiviert die Sender-BLE-Erkennung vollständig |
| `QSD_BT_CLASSIC` | unset | wenn gesetzt (`=1`), aktiviert experimentellen Bluetooth-Classic-Erstkontakt |
| `QSD_BLE_L2CAP` / `QSD_L2CAP_CHUNK` | on / default | Sendepfad-BLE-L2CAP-Tuning (`=0` erzwingt Weave-Socket) |
| `QSD_WEAVE_ATT` / `QSD_WEAVE_PACKET` | on / MTU default | GATT/Weave-Transport-Tuning für Experimente |
| `QSD_SEND_MEDIUMS` | wifi_lan-first default | kommagetrennte ausgehende Medienpräferenz (`wifi_lan`, ...) |
| `QS_SINGLE_CHANNEL` | `auto` | `always` = volle Geschwindigkeit beim Senden (kann Station abwerfen); `never` = Verbindung halten, auf Bluetooth zurückfallen |
| `QSD_AP_IFACE` / `QSD_STA_IFACE` | `ap0` / auto | Link-Schnittstellennamen für den AP-Helfer |
| `QSD_AP_UNIT` / `QSD_JOIN_UNIT` | `quickshare-ap` / `quickshare-join` | Systemd-Unit-Namen für die Root-Helfer |
| `QSD_SYSTEMCTL` | `systemctl` | Überschreibung des Helfer-Startbefehls |
| `QSD_AP_CREDENTIALS` | `/run/quickshare/credentials` | Credential-Ablagepfad für den AP-Helfer |
| `QSD_JOIN_REQUEST` / `QSD_JOIN_RESULT` | `/run/quickshare-join/...` | Anfrage-/Ergebnispfade für den Join-Helfer |
| `RUST_LOG` | `info` (`warn` for send/devices) | Log-Filter |

Empfängerdatei: `~/.config/quickshare-direct/env` (`QSD_NAME`, `QSD_DIR`, ...). Helferdatei:
`/etc/default/quickshare-direct`.

### Wie es funktioniert / Forschungsnotizen

[docs/DEV_NOTES.md](docs/DEV_NOTES.md) (Erkenntnisse, Telefonlogs, Protokolldetails) und
[docs/BLE_RECEIVER_DISCOVERY.md](docs/BLE_RECEIVER_DISCOVERY.md) (der BLE-Empfängerpfad).

Releases (dieser Fork): Das Pushen eines `v*.*.*`-Tags führt [`.github/workflows/release.yml`](.github/workflows/release.yml) aus, das die Tauri-GUI-Pakete (`.deb`, `.rpm`, `.AppImage`) und den `directdrop-cli-linux-x86_64.tar.gz`-Daemon-Tarball baut und sie im GitHub-Release veröffentlicht — installierbar via `get-directdrop.sh` oben. Upstream-Daemon-Pakete behalten ihren eigenen Ablauf: CI (`.github/workflows/packages.yml`) baut und install-testet die Pakete bei jedem Push; das Erhöhen von `VERSION` auf `main` veröffentlicht das Release `qsd-v<VERSION>`.

Repository-Layout: `core_lib` (Bibliothek + das `quickshare-direct`-Binärprogramm) und `packaging/`; das ist
alles, was die Pakete, das NixOS-Modul und CI baut. `app/main/` ist die reaktivierte Desktop-GUI dieses Forks
(Tauri + Vue): sie umhüllt `core_lib` in-prozess via `src-tauri/src/dto.rs` + `bridge.rs` und den
`src/types/qsd.ts`-Vertrag, ohne den Kern anzufassen. Andere Upstream-Überbleibsel (`snap/`, `BUILD.md`,
`CHANGELOG.md`, release-please, die `build`-/`lint`-Workflows) verweisen noch auf Upstream-`master`.

Entwicklung (headless): `nix develop`, dann in `core_lib`: `cargo run --bin quickshare-direct`
(env: `QSD_DIR`, `QSD_NAME`, `QSD_PORT`, `QSD_BWU_PORT`, `QSD_BWU=lan|hotspot`, `RUST_LOG`).
Entwicklung (GUI): siehe „Die GUI aus dem Quellcode starten“ oben.

---

<div align="center">

## ⭐ Unterstütze uns

Wenn DirectDrop deine Dateien auf dein Telefon bekommt, ohne den Router anzufassen, gib ihm einen Stern — das hilft sehr.

<a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=social" alt="star this repo" /></a>

<a href="https://star-history.com/#DeveloperMatt02/DirectDrop&Date"><img src="https://api.star-history.com/svg?repos=DeveloperMatt02/DirectDrop&type=Date" width="600" alt="star history chart" /></a>

Einen Fehler gefunden? [Eröffne ein Issue](https://github.com/DeveloperMatt02/DirectDrop/issues/new) — Pull Requests willkommen.

</div>
