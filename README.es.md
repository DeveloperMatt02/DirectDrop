<div align="center">
  <a href="README.md"><img src="https://img.shields.io/badge/English-EN-555?style=flat-square" alt="English" /></a>
  <a href="README.it.md"><img src="https://img.shields.io/badge/Italiano-IT-555?style=flat-square" alt="Italiano" /></a>
  <img src="https://img.shields.io/badge/Español-ES-blue?style=flat-square" alt="Español" />
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
  <p><strong>Quick Share (Nearby Share) para Linux que no necesita que ambos dispositivos estén en la misma Wi-Fi — ahora con GUI de escritorio.</strong></p>

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
  <a href="#instalación-rápida-terminal"><img src="https://img.shields.io/badge/🐧_Quick_Install-22c55e?style=for-the-badge" alt="quick install" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/issues/new"><img src="https://img.shields.io/badge/🐛_Report_Bug-f59e0b?style=for-the-badge" alt="report bug" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/badge/⭐_Star-facc15?style=for-the-badge&logo=github" alt="star" /></a>
</div>

> Un fork del `quickshare-direct` original. La pila headless de receptor / remitente se conserva sin cambios (primer contacto por anuncio BLE / GATT, mejora a cliente / GO de Wi-Fi Direct, alternativa por LAN, ayudantes polkit, demonio CLI headless). Lo que añade este fork es una GUI de escritorio Tauri rediseñada en `app/main/` con paridad total de comportamiento con la CLI.

## Instalación rápida (Terminal)

Comando único universal — detecta la distribución e instala el paquete correspondiente desde la
última versión de `DeveloperMatt02/DirectDrop` (`.rpm` en Fedora/RHEL, `.deb` en
Debian/Ubuntu, `.AppImage` en `~/.local/bin` en los demás casos):

```bash
curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash
```

El instalador pregunta qué edición instalar (GUI si está disponible, si no el demonio).
Desatendido: `DIRECTDROP_EDITION=cli curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash`,
o `curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash -s -- --cli`
(`--gui` solo GUI, `--auto` valor predeterminado explícito).

Comandos directos por distribución (misma fuente, sin script auxiliar):

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

O elige el archivo manualmente desde la [última versión](https://github.com/DeveloperMatt02/DirectDrop/releases/latest).

## GUI + demonio headless: elige uno

Los paquetes de la GUI (`.deb` / `.rpm` / `.AppImage`) incluyen su propio receptor; el paquete del demonio `quickshare-direct` es el mismo receptor sin interfaz. Ejecutar ambos a la vez anuncia el portátil dos veces con el mismo nombre de dispositivo y hace que compitan por los ayudantes Wi-Fi por usuario (`quickshare-ap@USER`, `quickshare-join@USER`), por lo que `get-directdrop.sh` desactiva el demonio (`systemctl --user disable --now quickshare-direct`) cuando instala un paquete de la GUI. La GUI `.deb`/`.rpm` también añade DirectDrop al menú de aplicaciones (busca "DirectDrop"); el `.AppImage` no lo hace hasta integrarlo (Gear Lever, appimaged o un archivo `.desktop` manual). Para volver al modo headless: sal de la GUI y ejecuta `systemctl --user enable --now quickshare-direct`.

La guía rápida original sigue siendo válida:

1. **Descubrimiento + primer contacto por Bluetooth LE** (funciona con el teléfono fuera de la Wi-Fi).
2. **Apretón de manos cifrado** (UKEY2, PIN mostrado en ambos lados) sobre ese enlace Bluetooth.
3. **Mejora a velocidad Wi-Fi**: el portátil crea un **grupo de Wi-Fi Direct** junto a su conexión Wi-Fi normal (que sigue funcionando) y entrega las credenciales al teléfono; o, si el teléfono está en la misma Wi-Fi, la transferencia pasa a la red local. Un punto de acceso simple es la alternativa.
4. **Notificaciones de escritorio**: Aceptar/Rechazar (con el PIN) y luego «Abrir carpeta».

| | Estado |
|---|---|
| Recibir desde Android (probado originalmente: Pixel 10, Android 16) | funciona |
| Teléfono fuera de Wi-Fi → Wi-Fi Direct (75 MB, ~5 MB/s) | funciona |
| Teléfono en la misma Wi-Fi → LAN | funciona |
| Primer contacto por Bluetooth Classic | experimental (`QSD_BT_CLASSIC=1`) |
| Enviar a Android → Wi-Fi Direct del teléfono (300 MB, ~36 MB/s) | funciona (`quickshare-direct send`, gestor de archivos, panel de envío de la GUI) |
| Enviar solo por Bluetooth (sin Wi-Fi compartida) a un OnePlus Nord4 (Android 16) | funciona (panel de envío de la GUI, BLE L2CAP, sin solicitud de emparejamiento) |
| Paquetes: Debian/Ubuntu, Fedora, Arch, genérico (x86_64, aarch64) | instalación en una línea, ver abajo |

---

## Lo que presenta este fork: GUI de escritorio reactivada y rediseñada (`app/main/`)

El repositorio original conserva la antigua aplicación Tauri de rQuickShare bajo `app/`, pero no la compila, distribuye ni prueba (su ruta de envío es solo LAN/mDNS, sin la ruta Bluetooth / Wi-Fi Direct). Este fork reactiva esa idea como una GUI nueva y específica en `app/main/` que utiliza el **mismo** motor `core_lib` (`rqs_lib`) que la CLI — sin implementación paralela del protocolo.

### Pila tecnológica y apariencia

- **Tauri 2.2, Vue 3.5, Pinia 2.x, Tailwind CSS 3.4.** El punto de entrada del frontend es `app/main/src/App.vue`; el estado vive en stores de Pinia (`src/stores/session.ts`, `devices.ts`, `transfers.ts`, `transferGroups.ts`).
- **Diseño con barra lateral y actividad:** la barra lateral izquierda (`StatusHeader.vue`) contiene el radar (`FlowHero.vue`, con ondas solo mientras el PC es visible), el nombre del PC, el selector **Visible / Temporarily / Hidden**, la carpeta de descargas y el puerto. El área principal es **Activity**: arriba la tarjeta de solicitud de consentimiento, luego el panel de envío compacto (`SendPane.vue` + `DeviceBoard.vue`) y la lista de transferencias (`TransferQueue.vue`) agrupada en **NOW / TODAY / EARLIER**, de la más reciente a la más antigua. Las cadenas de la interfaz están en inglés en mayúsculas.
- **Tema claro y oscuro automático:** sigue la configuración del escritorio. En Linux lee `org.freedesktop.appearance color-scheme` del portal de ajustes (`ashpd`, comando `qsd_system_theme`, evento `qsd://system-theme`), con respaldo en `theme()` de Tauri y después en la media query `prefers-color-scheme`. Los colores son tokens `--qsd-*`.
- **Filas de transferencia:** dirección, una insignia del medio en uso (Bluetooth LE, BLE L2CAP, Wi-Fi LAN, Wi-Fi Direct, hotspot), un único toast por transferencia más un aviso en la fila cuando se recurre a un medio más lento, velocidad en tiempo real y velocidad media (AVG) al terminar. **OPEN FILE** / **OPEN FOLDER** solo aparecen en las recepciones (1 archivo: ambos; varios archivos: solo la carpeta).
- **Receptor más fiable (core):** las tareas de anuncio mDNS y BLE se reinician solas con backoff, así que el PC ya no desaparece de los dispositivos cercanos tras un error; tras un error o una cancelación en la recepción se borran los archivos parciales y se conservan los completados.

### Puente Rust en proceso no invasivo

La GUI ejecuta `rqs_lib` en proceso. El límite son exactamente dos archivos nuevos más un contrato versionado — **cero cambios en el motor central del protocolo**:

- `app/main/src-tauri/src/dto.rs` — el único formato de intercambio. Los tipos de dominio centrales en `core_lib` deliberadamente no derivan `Serialize`/`Deserialize`; cada payload `invoke`/`emit` de Tauri pasa por estos DTO serde con mapeos `From` explícitos. `DTO_VERSION = 1`, verificado por la prueba Rust `dto_version_matches_ts` contra el espejo TypeScript.
- `app/main/src-tauri/src/bridge.rs` — se suscribe a los canales `broadcast`/`watch` centrales (`message_sender`, `dch_sender`, `visibility_sender`, `ble_receiver`) y reemite eventos Tauri limitados y sin duplicados. Los eventos de progreso se limitan (emisión al cambiar de estado, o con delta ≥256 KiB, o ≥500 ms; los estados terminales siempre se emiten). Los desbordamientos `Lagged` del broadcast se omiten con un registro de depuración, nunca con un fallo — la misma política que la CLI.
- `app/main/src/types/qsd.ts` — espejo TypeScript estricto de `dto.rs` (nombres de eventos, `VisibilityDto`, `TransferStateDto` con los 18 estados centrales, `PayloadKindDto`, `TransferDto`, `EndpointDto`, `SendPayloadDto`, `ConsentDto`, `SnapshotDto`, códigos de error). Incluye los ayudantes `humanSize` / `describeTransfer` (versión del `describe()` de la CLI), `baseName` y `validateSendFiles`, además de los conjuntos `TERMINAL_STATES` / `ACTIVE_PROGRESS_STATES` / `HANDSHAKE_STATES`.

Comandos Tauri expuestos (`src-tauri/src/cmds/`, conectados en `main.rs`): `qsd_snapshot`, `qsd_set_visibility`, `qsd_set_download_path`, `qsd_set_device_name`, `qsd_start_discovery`, `qsd_stop_discovery`, `qsd_consent`, `qsd_send_lan`, `qsd_scan_ble`, `qsd_send_ble`, `qsd_cancel_send`, `qsd_open_path`, `qsd_system_theme`, `qsd_get_hostname` (más los adaptadores heredados `get_hostname` / `change_download_path`). Eventos del backend → frontend: `qsd://transfer`, `qsd://endpoints`, `qsd://visibility`, `qsd://ble-presence`, `qsd://system-theme`.

### Paridad total de comportamiento con la CLI

- **Tarjeta instantánea de verificación PIN UKEY2** (`ConsentCard.vue`): el nombre del remitente, el tipo de dispositivo, el PIN, el resumen del contenido y la lista de archivos aparecen en cuanto el núcleo alcanza `WaitingForUserConsent`. Un anillo de **cuenta atrás de 60 segundos** (`QSD_CONSENT_TIMEOUT_MS`, respetando la variable `QSD_CONSENT_TIMEOUT` leída en `dto.rs`) caduca la tarjeta; la caducidad activa el mismo **rechazo automático** (`ConsentDecline` en el canal) que realiza la CLI. Aceptar/Rechazar/Cancelar corresponden 1:1 a `TransferAction`s centrales mediante `ConsentDto::into_channel_message`.
- **Selector de visibilidad de 3 estados** (`StatusHeader.vue` → `qsd_set_visibility`): `Visible`, `Temporarily` (60 s, con el tiempo restante mostrado como `VISIBLE FOR 0:xx`), `Hidden` (`Invisible`). El temporizador Temporarily→Invisible vive en el `mdns.rs` central; la GUI solo refleja el evento `qsd://visibility` y lo conserva mediante el almacén Tauri. Sin reimplementación del temporizador en la GUI.
- **Banner pasivo de alerta de presencia BLE:** los avistamientos del oyente BLE central llegan como eventos `qsd://ble-presence` limitados (120 s). Cuando está oculto (`Invisible`), el puente además dispara una notificación del sistema que sugiere una ventana de visibilidad temporal; el frontend muestra un aviso informativo («Hay un teléfono cercano compartiendo — comprueba la visibilidad»). Respeta la semántica central de `QSD_BLE_SCAN` / `QSD_NO_BLE_SCAN`.
- **Gestión de transferencias de texto/URL en línea sin escrituras forzadas en disco** (`TransferCard.vue`): los contenidos `Text`, `Url` y `WiFi` aparecen en la fila (`TEXT`, `LINK`, `WI-FI · SSID`) con COPY / COPY LINK (plugin clipboard-manager de Tauri) y OPEN para los enlaces; no se escribe ningún archivo en disco para mostrarlos. Los contenidos de archivos muestran el recuento de archivos RX, los nombres y el progreso exacto en bytes (`ack_bytes` / `total_bytes`).
- **Envío saliente de archivos LAN/mDNS con validación de archivos regulares** (`SendPane.vue` + `qsd_send_lan`): prepara archivos mediante diálogo o arrastrar y soltar, elige un endpoint mDNS descubierto (`EndpointDto`, `addr = ip:port`) y envía a través del canal central `sender_file` como `SendInfo { ob: Files }`. La validación coincide con la CLI: selecciones vacías y directorios rechazados con errores `VALIDATION` (`{f}: not a regular file (folders aren't supported)`); el frontend falla rápido en los casos evidentes antes de invocar al backend.

### Ejecutar la GUI desde el código fuente

```bash
cd app/main
pnpm install
pnpm tauri dev     # live GUI against local core_lib (path dep: ../../core_lib)
pnpm ts-check       # vue-tsc --noEmit contract check
pnpm lint           # eslint
pnpm test           # vitest suite (tests/)
pnpm build          # production Tauri bundle
```

Lado Rust: `cd app/main/src-tauri && cargo check` / `cargo test` (incluye las pruebas de formato de intercambio DTO). Los prerrequisitos del sistema son los mismos que los del demonio (BlueZ, NetworkManager, cabeceras libdbus, protoc, cargo ≥ 1.85) más Node/pnpm y dependencias Linux de Tauri 2.x.

---

## Arquitectura y genealogía / Créditos

Vista por capas de este fork:

```
Android phone ──BLE adv / GATT──▶ core_lib (rqs_lib) ◀──Tauri bridge──▶ Vue GUI
                                         │  (unchanged engine)
                                         ▼
                                Wi-Fi Direct GO / client,
                                LAN, hotspot fallback,
                                polkit Wi-Fi helpers
```

- **Autor(es) del `quickshare-direct` original:** el transporte fuera de red central del que depende este fork — supervisión de anuncios BLE y servidor GATT para descubrimiento/primer contacto, mejora a grupo Wi-Fi Direct como propietario / cliente junto a la conexión Wi-Fi normal (con gestión de alternativa monocanal), los ayudantes root bajo demanda `quickshare-ap@USER` / `quickshare-join@USER` con su regla polkit, y el demonio CLI headless `quickshare-direct` con notificaciones de escritorio. Todo el mérito de la ingeniería de protocolo, Bluetooth y Wi-Fi pertenece aquí; este fork no modifica nada de ello.
- **`rQuickShare` de Martichou (y colaboradores como nozwock y martinalderson):** la implementación fundacional en Rust/Tauri del protocolo Nearby Share / Quick Share en Linux sobre la que se construyen tanto el proyecto original como este fork — incluido el núcleo `rqs_lib`, la estructura original de la aplicación Tauri y el trabajo de descubrimiento Bluetooth continuado en `core_lib`.
- **Google Nearby (Apache-2.0):** la especificación del protocolo y las definiciones protobuf que definen el apretón de manos UKEY2, la conexión, la presentación y las tramas de contenido. La licencia de este repositorio sigue siendo GPL-3.0, como rQuickShare.

---

### Instalación (paquetes del demonio + CLI)

Debian 12+ / Ubuntu 22.04+ (y Mint, Pop!_OS, ...), Fedora, Arch (y Manjaro, EndeavourOS, ...),
otros mediante compilación genérica; x86_64 y aarch64:

```
curl -fsSL https://raw.githubusercontent.com/papshmeare/quickshare-direct/main/install.sh | sh
```

Instala la última [versión](https://github.com/papshmeare/quickshare-direct/releases) con
tu gestor de paquetes (`.deb`, `.rpm`, `.pkg.tar.zst`; las dependencias vienen de tu distribución),
abre los puertos de Quick Share si firewalld o ufw están activos e inicia el receptor. Ejecuta la misma
línea de nuevo para actualizar; `curl -fsSL .../install.sh | sh -s -- --uninstall` lo elimina. ¿Prefieres
ver antes lo que ocurre? Descarga `install.sh` y léelo, o instala el paquete desde la
página de versiones a mano (`sudo apt install ./quickshare-direct_amd64.deb`,
`sudo dnf install ./quickshare-direct.x86_64.rpm`,
`sudo pacman -U quickshare-direct-x86_64.pkg.tar.zst`).

Lo que se instala:
- `quickshare-direct`: el receptor (un servicio de usuario en cada sesión gráfica, con
  notificaciones) y los comandos `send` / `devices`;
- dos pequeños ayudantes root, iniciados bajo demanda mediante systemd (`quickshare-ap@USER` crea el
  enlace Wi-Fi directo al recibir, `quickshare-join@USER` se une al del teléfono al enviar), y una
  regla polkit (`packaging/linux/polkit/50-quickshare-direct.rules`) que permite a un usuario local
  activo iniciar/detener solo sus propias instancias (`start`/`stop`/`restart` en
  `quickshare-ap@USER.service` / `quickshare-join@USER.service`);
- un drop-in de NetworkManager que deja tranquilas las interfaces temporales del enlace, una unidad que mantiene
  Bluetooth conectable, definiciones de servicio firewalld/ufw y entradas del gestor de archivos.

Necesita: systemd, BlueZ, NetworkManager con wpa_supplicant (el valor por defecto en estas distribuciones;
no iwd) y una tarjeta Wi-Fi capaz de mantener una segunda interfaz junto a la conexión normal (la mayoría de
tarjetas Intel/MediaTek/Qualcomm). Las notificaciones necesitan un demonio que muestre botones de acción (GNOME,
KDE, swaync, mako, ...). El polkit antiguo de Ubuntu 22.04 no permite a los usuarios iniciar los ayudantes, así que allí
solo funcionan las transferencias por Bluetooth y de la misma red. Configura tu dominio regulatorio Wi-Fi
(`sudo iw reg set XX`): con el dominio mundial por defecto Linux no iniciará un propietario de grupo en 5 GHz.
Ajustes: `~/.config/quickshare-direct/env` (`QSD_NAME=...`, `QSD_DIR=...`) para el receptor,
`/etc/default/quickshare-direct` para los ayudantes. Muchas tarjetas Wi-Fi no pueden alojar el enlace directo en un
segundo canal junto a tu conexión normal, lo que importa cuando tu rúter usa un canal que un
portátil quizá no pueda alojar (canales radar/DFS como el 100): entonces el portátil abandona tu red Wi-Fi
durante los pocos segundos de la transferencia y se reconecta después (`QS_SINGLE_CHANNEL=auto`; `always`
para máxima velocidad también al enviar, `never` para mantener la conexión y recurrir a Bluetooth).

Desde el código fuente: `make && sudo make install` (necesita cargo ≥ 1.85, protoc, cabeceras libdbus), luego
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
Esto configura el receptor (servicio de usuario en la sesión gráfica), el ayudante root que crea
el enlace Wi-Fi directo bajo demanda (`quickshare-ap.service`, que ese usuario puede iniciar vía polkit),
el ayudante que se une al grupo Wi-Fi Direct del teléfono al enviar (`quickshare-join.service`),
puertos del cortafuegos, Bluetooth «conectable» y mantiene a NetworkManager fuera de las interfaces del enlace.
Configura tu dominio regulatorio Wi-Fi (`iw reg set XX`): con el dominio mundial por defecto Linux no
iniciará un punto de acceso / propietario de grupo en 5 GHz.

### Envío (CLI)

En el teléfono, abre Quick Share y toca **Recibir** (o hazlo visible para todos), luego:

```
quickshare-direct send photo.jpg notes.pdf      # to the one phone in receive mode nearby
quickshare-direct send --to Pixel video.mp4     # pick by name when several are around
quickshare-direct devices                       # list phones in receive mode
```
Encuentra el teléfono por Bluetooth, muestra el PIN y, una vez que aceptas en el teléfono, los archivos van
por el grupo Wi-Fi Direct del teléfono (unido en una segunda interfaz, tu Wi-Fi sigue conectado).
En el gestor de archivos: **Enviar a → Teléfono (Quick Share)** de Thunar (cualquier archivo), o Abrir con →
«Enviar con Quick Share» (tipos de archivo comunes); informa mediante notificaciones. El
`SendPane` de la GUI realiza el mismo envío LAN/mDNS mediante `qsd_send_lan` con idénticos PIN y
semántica de archivos regulares.

### Variables de entorno (receptor, remitente, ayudantes)

El comportamiento central se configura mediante el entorno (ver `core_lib/src/bin/quickshare-direct.rs`,
`core_lib/src/hdl/*.rs`):

| Variable | Default | Effect |
|---|---|---|
| `QSD_DIR` | XDG Downloads, else `~/Downloads` | carpeta de descargas |
| `QSD_NAME` | hostname | nombre del dispositivo mostrado en el teléfono |
| `QSD_PORT` | random | puerto TCP para transferencias entrantes |
| `QSD_BWU_PORT` | random | puerto TCP para el oyente de mejora Wi-Fi |
| `QSD_CONSENT_TIMEOUT` | `60` (s) | segundos para responder Aceptar/Rechazar antes del rechazo automático; también controla la cuenta atrás de la GUI |
| `QSD_BWU` | `auto` | preferencia de mejora entrante: `lan` / `hotspot` fuerza una |
| `QSD_BLE_SCAN` | passive BlueZ advertisement monitor | `discovery` = descubrimiento LE permanente (puede impedir que ratones/teclados LE vinculados se reconecten); `off` desactiva el oyente |
| `QSD_NO_BLE_SCAN` | unset | cuando está definida, desactiva por completo la detección BLE del remitente |
| `QSD_BT_CLASSIC` | unset | cuando está definida (`=1`), habilita el primer contacto experimental por Bluetooth Classic |
| `QSD_BLE_L2CAP` / `QSD_L2CAP_CHUNK` | on / default | ajuste BLE L2CAP de la ruta de envío (`=0` fuerza el socket weave) |
| `QSD_WEAVE_ATT` / `QSD_WEAVE_PACKET` | on / MTU default | ajuste del transporte GATT/weave para experimentos |
| `QSD_SEND_MEDIUMS` | wifi_lan-first default | preferencia de medio saliente separada por comas (`wifi_lan`, ...) |
| `QS_SINGLE_CHANNEL` | `auto` | `always` = velocidad máxima al enviar (puede interrumpir la estación); `never` = mantiene la conexión, recurre a Bluetooth |
| `QSD_AP_IFACE` / `QSD_STA_IFACE` | `ap0` / auto | nombres de interfaz de enlace para el ayudante AP |
| `QSD_AP_UNIT` / `QSD_JOIN_UNIT` | `quickshare-ap` / `quickshare-join` | nombres de unidades systemd para los ayudantes root |
| `QSD_SYSTEMCTL` | `systemctl` | anulación del comando de lanzamiento del ayudante |
| `QSD_AP_CREDENTIALS` | `/run/quickshare/credentials` | ruta de entrega de credenciales para el ayudante AP |
| `QSD_JOIN_REQUEST` / `QSD_JOIN_RESULT` | `/run/quickshare-join/...` | rutas de solicitud/resultado para el ayudante de unión |
| `RUST_LOG` | `info` (`warn` for send/devices) | filtro de registros |

Archivo del receptor: `~/.config/quickshare-direct/env` (`QSD_NAME`, `QSD_DIR`, ...). Archivo del ayudante:
`/etc/default/quickshare-direct`.

### Cómo funciona / notas de investigación

[docs/DEV_NOTES.md](docs/DEV_NOTES.md) (hallazgos, registros del teléfono, detalles del protocolo) y
[docs/BLE_RECEIVER_DISCOVERY.md](docs/BLE_RECEIVER_DISCOVERY.md) (la ruta del receptor BLE).

Versiones (este fork): publicar una etiqueta `v*.*.*` ejecuta [`.github/workflows/release.yml`](.github/workflows/release.yml), que compila los paquetes de la GUI Tauri (`.deb`, `.rpm`, `.AppImage`) y el tarball del demonio `directdrop-cli-linux-x86_64.tar.gz` y los publica en la Release de GitHub — instalables vía `get-directdrop.sh` de arriba. Los paquetes del demonio original mantienen su propio flujo: CI (`.github/workflows/packages.yml`) compila y prueba la instalación de los paquetes en cada push; subir `VERSION` en `main` publica la versión `qsd-v<VERSION>`.

Diseño del repositorio: `core_lib` (biblioteca + el binario `quickshare-direct`) y `packaging/`; eso es
todo lo que compilan los paquetes, el módulo NixOS y la CI. `app/main/` es la GUI de escritorio
reactivada de este fork (Tauri + Vue): envuelve `core_lib` en proceso vía `src-tauri/src/dto.rs` + `bridge.rs` y el
contrato `src/types/qsd.ts`, sin tocar el núcleo. Otros restos originales (`snap/`, `BUILD.md`,
`CHANGELOG.md`, release-please, los flujos `build`/`lint`) aún se refieren al `master` original.

Desarrollo (headless): `nix develop`, luego en `core_lib`: `cargo run --bin quickshare-direct`
(env: `QSD_DIR`, `QSD_NAME`, `QSD_PORT`, `QSD_BWU_PORT`, `QSD_BWU=lan|hotspot`, `RUST_LOG`).
Desarrollo (GUI): ver «Ejecutar la GUI desde el código fuente» más arriba.

---

<div align="center">

## ⭐ Muestra tu apoyo

Si DirectDrop lleva tus archivos a tu teléfono sin tocar el rúter, dale una estrella — ayuda mucho.

<a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=social" alt="star this repo" /></a>

<a href="https://star-history.com/#DeveloperMatt02/DirectDrop&Date"><img src="https://api.star-history.com/svg?repos=DeveloperMatt02/DirectDrop&type=Date" width="600" alt="star history chart" /></a>

¿Encontraste un error? [Abre un issue](https://github.com/DeveloperMatt02/DirectDrop/issues/new) — los pull requests son bienvenidos.

</div>
