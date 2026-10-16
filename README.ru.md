<div align="center">
  <a href="README.md"><img src="https://img.shields.io/badge/English-EN-555?style=flat-square" alt="English" /></a>
  <a href="README.it.md"><img src="https://img.shields.io/badge/Italiano-IT-555?style=flat-square" alt="Italiano" /></a>
  <a href="README.es.md"><img src="https://img.shields.io/badge/Español-ES-555?style=flat-square" alt="Español" /></a>
  <a href="README.fr.md"><img src="https://img.shields.io/badge/Français-FR-555?style=flat-square" alt="Français" /></a>
  <a href="README.de.md"><img src="https://img.shields.io/badge/Deutsch-DE-555?style=flat-square" alt="Deutsch" /></a>
  <a href="README.pt.md"><img src="https://img.shields.io/badge/Português-PT-555?style=flat-square" alt="Português" /></a>
  <img src="https://img.shields.io/badge/Русский-RU-blue?style=flat-square" alt="Русский" />
  <a href="README.zh.md"><img src="https://img.shields.io/badge/中文-ZH-555?style=flat-square" alt="中文" /></a>
  <a href="README.ja.md"><img src="https://img.shields.io/badge/日本語-JA-555?style=flat-square" alt="日本語" /></a>
</div>

<div align="center">
  <img src="app/main/src-tauri/icons/128x128.png" width="128" alt="DirectDrop icon" />
  <h1>DirectDrop</h1>
  <p><strong>Quick Share (Nearby Share) для Linux, которому не нужно, чтобы оба устройства были в одной сети Wi-Fi, — теперь с графическим интерфейсом для рабочего стола.</strong></p>

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
  <a href="#быстрая-установка-терминал"><img src="https://img.shields.io/badge/🐧_Quick_Install-22c55e?style=for-the-badge" alt="quick install" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/issues/new"><img src="https://img.shields.io/badge/🐛_Report_Bug-f59e0b?style=for-the-badge" alt="report bug" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/badge/⭐_Star-facc15?style=for-the-badge&logo=github" alt="star" /></a>
</div>

> Форк апстрима `quickshare-direct`. Безголовый стек приёма/отправки сохранён без изменений (первый контакт через BLE-рекламу/GATT, ускорение через Wi-Fi Direct в режимах GO/клиент, LAN-фолбэк, polkit-помощники, безголовый CLI-демон). Что добавляет этот форк — это переработанный настольный GUI на Tauri в `app/main/` с полным поведенческим паритетом с CLI.

## Быстрая установка (терминал)

Универсальная однострочная команда — определяет дистрибутив и устанавливает подходящий пакет из
последнего релиза `DeveloperMatt02/DirectDrop` (`.rpm` на Fedora/RHEL, `.deb` на
Debian/Ubuntu, `.AppImage` в `~/.local/bin` во всех остальных случаях):

```bash
curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash
```

Установщик спрашивает, какую редакцию установить (GUI, если доступен, иначе демон).
Для установки без вопросов: `DIRECTDROP_EDITION=cli curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash`,
или `curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash -s -- --cli`
(`--gui` только GUI, `--auto` явное значение по умолчанию).

Прямые команды для каждого дистрибутива (тот же источник, без вспомогательного скрипта):

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

Или выберите файл вручную на странице [последнего релиза](https://github.com/DeveloperMatt02/DirectDrop/releases/latest).

## GUI и безголовый демон: выберите что-то одно

Пакеты GUI (`.deb` / `.rpm` / `.AppImage`) содержат встроенный приёмник; пакет демона `quickshare-direct` — тот же приёмник, но безголовый. Одновременный запуск обоих анонсирует ноутбук дважды под одним именем устройства и заставляет их бороться за пользовательские Wi-Fi-помощники (`quickshare-ap@USER`, `quickshare-join@USER`), поэтому `get-directdrop.sh` отключает демон (`systemctl --user disable --now quickshare-direct`) при каждой установке пакета GUI. GUI из `.deb`/`.rpm` также добавляет DirectDrop в меню приложений (ищите "DirectDrop"); `.AppImage` — нет, пока он не интегрирован (Gear Lever, appimaged или вручную созданный файл `.desktop`). Чтобы вернуться к безголовому режиму: закройте GUI и выполните `systemctl --user enable --now quickshare-direct`.

Краткое руководство апстрима по-прежнему применимо:

1. **Обнаружение + первый контакт по Bluetooth LE** (работает, когда телефон отключён от Wi-Fi).
2. **Зашифрованное рукопожатие** (UKEY2, PIN показан на обеих сторонах) через это Bluetooth-соединение.
3. **Ускорение до скорости Wi-Fi**: ноутбук создаёт **группу Wi-Fi Direct** рядом со своим обычным Wi-Fi-подключением (которое продолжает работать) и передаёт учётные данные телефону; либо, если телефон находится в том же Wi-Fi, передача переходит в локальную сеть. Запасной вариант — обычная точка доступа.
4. **Настольные уведомления**: Принять/Отклонить (с PIN), затем «Открыть папку».

| | Статус |
|---|---|
| Приём с Android (протестировано в апстриме: Pixel 10, Android 16) | работает |
| Телефон без Wi-Fi → Wi-Fi Direct (75 МБ, ~5 МБ/с) | работает |
| Телефон в том же Wi-Fi → LAN | работает |
| Первый контакт по Bluetooth Classic | экспериментально (`QSD_BT_CLASSIC=1`) |
| Отправка на Android → Wi-Fi Direct телефона (300 МБ, ~36 МБ/с) | работает (`quickshare-direct send`, файловый менеджер, панель отправки GUI) |
| Отправка только по Bluetooth (без общей Wi-Fi) на OnePlus Nord4 (Android 16) | работает (панель отправки GUI, BLE L2CAP, без запроса сопряжения) |
| Пакеты: Debian/Ubuntu, Fedora, Arch, универсальный (x86_64, aarch64) | установка одной строкой, см. ниже |

---

## Что добавляет этот форк: возрождённый и переработанный настольный GUI (`app/main/`)

Апстрим-репозиторий сохраняет старый Tauri-приложение rQuickShare в `app/`, но не собирает, не поставляет и не тестирует его (его путь отправки — только LAN/mDNS, без пути Bluetooth / Wi-Fi Direct). Этот форк возрождает эту идею как новый, специально созданный GUI в `app/main/`, который управляет **тем же** движком `core_lib` (`rqs_lib`), что и CLI, — без параллельной реализации протокола.

### Стек и внешний вид

- **Tauri 2.2, Vue 3.5, Pinia 2.x, Tailwind CSS 3.4.** Точка входа фронтенда — `app/main/src/App.vue`; состояние живёт в Pinia-сторах (`src/stores/session.ts`, `devices.ts`, `transfers.ts`, `transferGroups.ts`).
- **Раскладка с боковой панелью и лентой активности:** левая боковая панель (`StatusHeader.vue`) содержит радар (`FlowHero.vue`, волны только пока ПК виден), имя ПК, переключатель **Visible / Temporarily / Hidden**, папку загрузок и порт. Основная область — **Activity**: сверху карточка запроса на согласие, затем компактная панель отправки (`SendPane.vue` + `DeviceBoard.vue`) и список передач (`TransferQueue.vue`), сгруппированный в **NOW / TODAY / EARLIER**, от новых к старым. Строки интерфейса — английские, заглавными буквами.
- **Светлая и тёмная тема автоматически:** следует настройке рабочего стола. В Linux читает `org.freedesktop.appearance color-scheme` из портала настроек (`ashpd`, команда `qsd_system_theme`, событие `qsd://system-theme`), с запасным вариантом через `theme()` Tauri, а затем через медиазапрос `prefers-color-scheme`. Цвета заданы токенами `--qsd-*`.
- **Строки передач:** направление, бейдж используемого канала (Bluetooth LE, BLE L2CAP, Wi-Fi LAN, Wi-Fi Direct, hotspot), один тост на передачу плюс предупреждение в строке при переходе на более медленный канал, скорость в реальном времени и средняя скорость (AVG) после завершения. **OPEN FILE** / **OPEN FOLDER** показываются только для приёма (1 файл: обе кнопки; несколько файлов: только папка).
- **Более надёжный приёмник (core):** задачи анонса mDNS и BLE сами перезапускаются с backoff, поэтому ПК больше не исчезает из списка ближайших устройств после ошибки; после ошибки или отмены при приёме неполные файлы удаляются, а завершённые остаются.

### Неинвазивный внутрипроцессный Rust-мост

GUI запускает `rqs_lib` внутри процесса. Граница — это ровно два новых файла плюс версионированный контракт — **ноль изменений в основном движке протокола**:

- `app/main/src-tauri/src/dto.rs` — единственный проводной формат. Основные доменные типы в `core_lib` намеренно не выводят `Serialize`/`Deserialize`; каждый полезный груз Tauri `invoke`/`emit` проходит через эти serde DTO с явными отображениями `From`. `DTO_VERSION = 1`, проверяется Rust-тестом `dto_version_matches_ts` по TypeScript-зеркалу.
- `app/main/src-tauri/src/bridge.rs` — подписывается на основные каналы `broadcast`/`watch` (`message_sender`, `dch_sender`, `visibility_sender`, `ble_receiver`) и повторно излучает троттлированные/дедуплицированные события Tauri. События прогресса троттлируются (излучение при смене состояния, либо при дельте ≥256 КиБ, либо ≥500 мс; терминальные состояния излучаются всегда). Переполнения `Lagged` в broadcast пропускаются с отладочным логом, никогда не приводят к падению — та же политика, что и в CLI.
- `app/main/src/types/qsd.ts` — строгое TypeScript-зеркало `dto.rs` (имена событий, `VisibilityDto`, `TransferStateDto` со всеми 18 основными состояниями, `PayloadKindDto`, `TransferDto`, `EndpointDto`, `SendPayloadDto`, `ConsentDto`, `SnapshotDto`, коды ошибок). Включает помощники `humanSize` / `describeTransfer` (порт CLI-функции `describe()`), `baseName` и `validateSendFiles`, плюс наборы `TERMINAL_STATES` / `ACTIVE_PROGRESS_STATES` / `HANDSHAKE_STATES`.

Команды Tauri (`src-tauri/src/cmds/`, связаны в `main.rs`): `qsd_snapshot`, `qsd_set_visibility`, `qsd_set_download_path`, `qsd_set_device_name`, `qsd_start_discovery`, `qsd_stop_discovery`, `qsd_consent`, `qsd_send_lan`, `qsd_scan_ble`, `qsd_send_ble`, `qsd_cancel_send`, `qsd_open_path`, `qsd_system_theme`, `qsd_get_hostname` (плюс старые прокладки `get_hostname` / `change_download_path`). События бэкенд → фронтенд: `qsd://transfer`, `qsd://endpoints`, `qsd://visibility`, `qsd://ble-presence`, `qsd://system-theme`.

### Полное соответствие CLI по поведению

- **Мгновенная карточка проверки UKEY2 PIN** (`ConsentCard.vue`): имя отправителя, тип устройства, PIN, сводка полезной нагрузки и список файлов появляются, как только ядро достигает `WaitingForUserConsent`. **60-секундное кольцо обратного отсчёта** (`QSD_CONSENT_TIMEOUT_MS`, с учётом переопределения через env `QSD_CONSENT_TIMEOUT`, читаемого в `dto.rs`) завершает срок действия карточки; истечение срока запускает то же **автоотклонение** (`ConsentDecline` в канале), что выполняет CLI. Принять/Отклонить/Отмена отображаются 1:1 на основные `TransferAction` через `ConsentDto::into_channel_message`.
- **3-позиционный селектор видимости** (`StatusHeader.vue` → `qsd_set_visibility`): `Visible`, `Temporarily` (60 с, с оставшимся временем в виде `VISIBLE FOR 0:xx`), `Hidden` (`Invisible`). Таймер перехода Temporarily→Invisible живёт в основном `mdns.rs`; GUI только отражает событие `qsd://visibility` и сохраняет его через Tauri store. Без повторной реализации таймера на стороне GUI.
- **Пассивный баннер присутствия по BLE:** обнаружения слушателя BLE ядра приходят как троттлированные (120 с) события `qsd://ble-presence`. Когда скрыто (`Invisible`), мост дополнительно посылает системное уведомление с предложением временного окна видимости; фронтенд показывает информационный тост («Рядом телефон делится — проверьте видимость»). Учитывает основную семантику `QSD_BLE_SCAN` / `QSD_NO_BLE_SCAN`.
- **Встроенная обработка передач текста/URL без принудительной записи на диск** (`TransferCard.vue`): полезные нагрузки `Text`, `Url` и `WiFi` показываются в строке (`TEXT`, `LINK`, `WI-FI · SSID`) с кнопками COPY / COPY LINK (плагин Tauri clipboard-manager) и OPEN для ссылок; никакой файл на диск для их отображения не записывается. Файловые полезные нагрузки показывают число RX-файлов, имена и побайтово точный прогресс (`ack_bytes` / `total_bytes`).
- **Исходящая отправка файлов по LAN/mDNS с проверкой обычных файлов** (`SendPane.vue` + `qsd_send_lan`): подготовьте файлы через диалог или drag-and-drop, выберите обнаруженную mDNS-конечную точку (`EndpointDto`, `addr = ip:port`) и отправьте через основной канал `sender_file` как `SendInfo { ob: Files }`. Проверка соответствует CLI: пустые выборки и каталоги отклоняются с ошибками `VALIDATION` (`{f}: not a regular file (folders aren't supported)`); фронтенд быстро отказывает в очевидных случаях ещё до вызова бэкенда.

### Запуск GUI из исходников

```bash
cd app/main
pnpm install
pnpm tauri dev     # live GUI against local core_lib (path dep: ../../core_lib)
pnpm ts-check       # vue-tsc --noEmit contract check
pnpm lint           # eslint
pnpm test           # vitest suite (tests/)
pnpm build          # production Tauri bundle
```

Сторона Rust: `cd app/main/src-tauri && cargo check` / `cargo test` (включая тесты проводного формата DTO). Системные предпосылки те же, что и у демона (BlueZ, NetworkManager, заголовки libdbus, protoc, cargo ≥ 1.85), плюс Node/pnpm и зависимости Tauri 2.x для Linux.

---

## Архитектура и происхождение / Благодарности

Послойный вид этого форка:

```
Android phone ──BLE adv / GATT──▶ core_lib (rqs_lib) ◀──Tauri bridge──▶ Vue GUI
                                        │  (unchanged engine)
                                        ▼
                               Wi-Fi Direct GO / client,
                               LAN, hotspot fallback,
                               polkit Wi-Fi helpers
```

- **Автор(ы) апстрима `quickshare-direct`:** основной вне-сетевой транспорт, от которого зависит этот форк, — мониторинг BLE-рекламы и GATT-сервер для обнаружения/первого контакта, повышение до Wi-Fi Direct в режимах владельца группы / клиента рядом с обычным Wi-Fi-подключением (с обработкой одноканального фолбэка), вспомогательные on-demand root-помощники `quickshare-ap@USER` / `quickshare-join@USER` с их polkit-правилом, и безголовый CLI-демон `quickshare-direct` с настольными уведомлениями. Вся заслуга за протокол, Bluetooth и Wi-Fi-инженерию принадлежит им; этот форк ничего из этого не меняет.
- **`rQuickShare` от Martichou (и участников, таких как nozwock и martinalderson):** основополагающая Rust/Tauri-реализация протокола Nearby Share / Quick Share на Linux, на которой построены и апстрим, и этот форк, — включая ядро `rqs_lib`, исходную структуру Tauri-приложения и работу по обнаружению Bluetooth, продолженную в `core_lib`.
- **Google Nearby (Apache-2.0):** спецификация протокола и protobuf-определения, задающие кадры рукопожатия UKEY2, соединения, представления и полезной нагрузки. Лицензия этого репозитория остаётся GPL-3.0, как у rQuickShare.

---

### Установка (демон + пакеты CLI)

Debian 12+ / Ubuntu 22.04+ (и Mint, Pop!_OS, ...), Fedora, Arch (и Manjaro, EndeavourOS, ...),
остальные — через универсальную сборку; x86_64 и aarch64:

```
curl -fsSL https://raw.githubusercontent.com/papshmeare/quickshare-direct/main/install.sh | sh
```

Он устанавливает последний [релиз](https://github.com/papshmeare/quickshare-direct/releases) вашим
пакетным менеджером (`.deb`, `.rpm`, `.pkg.tar.zst`; зависимости берутся из вашего дистрибутива),
открывает порты Quick Share, если активны firewalld или ufw, и запускает приёмник. Запустите ту же
строку снова, чтобы обновиться; `curl -fsSL .../install.sh | sh -s -- --uninstall` удаляет его. Хотите
сначала увидеть, что произойдёт? Скачайте `install.sh` и прочитайте его, либо установите пакет со
страницы релизов вручную (`sudo apt install ./quickshare-direct_amd64.deb`,
`sudo dnf install ./quickshare-direct.x86_64.rpm`,
`sudo pacman -U quickshare-direct-x86_64.pkg.tar.zst`).

Что устанавливается:
- `quickshare-direct`: приёмник (пользовательская служба в каждой графической сессии, с
  уведомлениями) и команды `send` / `devices`;
- два небольших root-помощника, запускаемых по требованию через systemd (`quickshare-ap@USER` создаёт
  прямое Wi-Fi-соединение при приёме, `quickshare-join@USER` присоединяется к соединению телефона при отправке), и
  polkit-правило (`packaging/linux/polkit/50-quickshare-direct.rules`), которое позволяет активному локальному
  пользователю запускать/останавливать только свои собственные экземпляры (`start`/`stop`/`restart` для
  `quickshare-ap@USER.service` / `quickshare-join@USER.service`);
- drop-in NetworkManager, который оставляет временные интерфейсы соединения в покое, юнит, поддерживающий
  Bluetooth доступным для подключения, определения служб firewalld/ufw и записи файлового менеджера.

Требуется: systemd, BlueZ, NetworkManager с wpa_supplicant (по умолчанию в этих дистрибутивах;
не iwd), и карта Wi-Fi, способная запустить второй интерфейс рядом с обычным подключением (большинство
карт Intel/MediaTek/Qualcomm). Уведомлениям нужен демон, показывающий кнопки действий (GNOME,
KDE, swaync, mako, ...). Более старый polkit в Ubuntu 22.04 не может разрешить пользователям запускать помощники, поэтому там
работают только Bluetooth и передачи в одной сети. Установите свой регуляторный домен Wi-Fi
(`sudo iw reg set XX`): с регуляторным доменом world по умолчанию Linux не запустит владельца группы на 5 ГГц.
Настройки: `~/.config/quickshare-direct/env` (`QSD_NAME=...`, `QSD_DIR=...`) для приёмника,
`/etc/default/quickshare-direct` для помощников. Многие чипы Wi-Fi не могут разместить прямое соединение на
втором канале рядом с вашим обычным подключением, что важно, когда ваш роутер использует канал, который
ноутбук не может разместить (радарные/DFS-каналы, такие как 100): тогда ноутбук покидает вашу сеть Wi-Fi
на несколько секунд передачи и переподключается после неё (`QS_SINGLE_CHANNEL=auto`; `always`
для полной скорости и при отправке, `never` чтобы сохранить подключение и откатиться на Bluetooth).

Из исходников: `make && sudo make install` (нужны cargo ≥ 1.85, protoc, заголовки libdbus), затем
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
Это настраивает приёмник (пользовательскую службу в графической сессии), root-помощник, создающий
прямое Wi-Fi-соединение по требованию (`quickshare-ap.service`, запускаемый этим пользователем через polkit),
помощник, присоединяющийся к группе Wi-Fi Direct телефона при отправке (`quickshare-join.service`),
порты брандмауэра, «доступность» Bluetooth и удерживает NetworkManager вдали от интерфейсов соединения.
Установите свой регуляторный домен Wi-Fi (`iw reg set XX`): с регуляторным доменом world по умолчанию Linux не
запустит точку доступа / владельца группы на 5 ГГц.

### Отправка (CLI)

На телефоне откройте Quick Share и нажмите **Receive** (или сделайте его видимым для всех), затем:

```
quickshare-direct send photo.jpg notes.pdf      # to the one phone in receive mode nearby
quickshare-direct send --to Pixel video.mp4     # pick by name when several are around
quickshare-direct devices                       # list phones in receive mode
```
Он находит телефон по Bluetooth, показывает PIN, и как только вы подтвердите на телефоне, файлы пойдут
через группу Wi-Fi Direct телефона (подключение на втором интерфейсе, ваш Wi-Fi остаётся подключённым).
В файловом менеджере: **Отправить → Телефон (Quick Share)** в Thunar (любой файл), или Открыть с помощью →
«Send with Quick Share» (распространённые типы файлов); отчёт идёт через уведомления. GUI
`SendPane` выполняет ту же отправку по LAN/mDNS через `qsd_send_lan` с идентичными PIN и
семантикой обычных файлов.

### Переменные окружения (приёмник, отправитель, помощники)

Основное поведение настраивается через окружение (см. `core_lib/src/bin/quickshare-direct.rs`,
`core_lib/src/hdl/*.rs`):

| Variable | Default | Эффект |
|---|---|---|
| `QSD_DIR` | XDG Downloads, else `~/Downloads` | папка загрузок |
| `QSD_NAME` | hostname | имя устройства, отображаемое на телефоне |
| `QSD_PORT` | random | TCP-порт для входящих передач |
| `QSD_BWU_PORT` | random | TCP-порт для слушателя ускорения Wi-Fi |
| `QSD_CONSENT_TIMEOUT` | `60` (s) | секунд на ответ «Принять/Отклонить» до автоотклонения; также управляет обратным отсчётом GUI |
| `QSD_BWU` | `auto` | предпочтение входящего ускорения: `lan` / `hotspot` принудительно выбирает одно |
| `QSD_BLE_SCAN` | passive BlueZ advertisement monitor | `discovery` = постоянное LE-обнаружение (может мешать переподключению сопряжённых LE-мышей/клавиатур); `off` отключает слушатель |
| `QSD_NO_BLE_SCAN` | unset | когда установлен, полностью отключает обнаружение отправителя по BLE |
| `QSD_BT_CLASSIC` | unset | когда установлен (`=1`), включает экспериментальный первый контакт по Bluetooth Classic |
| `QSD_BLE_L2CAP` / `QSD_L2CAP_CHUNK` | on / default | настройка L2CAP по BLE для пути отправки (`=0` принудительно включает weave-сокет) |
| `QSD_WEAVE_ATT` / `QSD_WEAVE_PACKET` | on / MTU default | настройка GATT/weave-транспорта для экспериментов |
| `QSD_SEND_MEDIUMS` | wifi_lan-first default | предпочтение исходящей среды через запятую (`wifi_lan`, ...) |
| `QS_SINGLE_CHANNEL` | `auto` | `always` = полная скорость при отправке (может разорвать станцию); `never` = сохранить подключение, откатиться на Bluetooth |
| `QSD_AP_IFACE` / `QSD_STA_IFACE` | `ap0` / auto | имена интерфейсов соединения для AP-помощника |
| `QSD_AP_UNIT` / `QSD_JOIN_UNIT` | `quickshare-ap` / `quickshare-join` | имена systemd-юнитов для root-помощников |
| `QSD_SYSTEMCTL` | `systemctl` | переопределение команды запуска помощников |
| `QSD_AP_CREDENTIALS` | `/run/quickshare/credentials` | путь сброса учётных данных для AP-помощника |
| `QSD_JOIN_REQUEST` / `QSD_JOIN_RESULT` | `/run/quickshare-join/...` | пути запроса/результата для join-помощника |
| `RUST_LOG` | `info` (`warn` for send/devices) | фильтр логов |

Файл приёмника: `~/.config/quickshare-direct/env` (`QSD_NAME`, `QSD_DIR`, ...). Файл помощников:
`/etc/default/quickshare-direct`.

### Как это работает / заметки об исследовании

[docs/DEV_NOTES.md](docs/DEV_NOTES.md) (находки, логи телефона, детали протокола) и
[docs/BLE_RECEIVER_DISCOVERY.md](docs/BLE_RECEIVER_DISCOVERY.md) (путь приёмника по BLE).

Релизы (этот форк): отправка тега `v*.*.*` запускает [`.github/workflows/release.yml`](.github/workflows/release.yml), который собирает пакеты GUI на Tauri (`.deb`, `.rpm`, `.AppImage`) и tarball демона `directdrop-cli-linux-x86_64.tar.gz` и публикует их в GitHub Release — устанавливаемые через `get-directdrop.sh` выше. Пакеты демона апстрима сохраняют свой собственный поток: CI (`.github/workflows/packages.yml`) собирает и тестово устанавливает пакеты при каждом пуше; увеличение `VERSION` на `main` публикует релиз `qsd-v<VERSION>`.

Структура репозитория: `core_lib` (библиотека + бинарник `quickshare-direct`) и `packaging/`; это всё,
что собирают пакеты, модуль NixOS и CI. `app/main/` — это возрождённый настольный GUI этого форка
(Tauri + Vue): он оборачивает `core_lib` внутри процесса через `src-tauri/src/dto.rs` + `bridge.rs` и
контракт `src/types/qsd.ts`, не трогая ядро. Прочие остатки апстрима (`snap/`, `BUILD.md`,
`CHANGELOG.md`, release-please, рабочие процессы `build`/`lint`) по-прежнему относятся к апстримному `master`.

Разработка (безголовая): `nix develop`, затем в `core_lib`: `cargo run --bin quickshare-direct`
(env: `QSD_DIR`, `QSD_NAME`, `QSD_PORT`, `QSD_BWU_PORT`, `QSD_BWU=lan|hotspot`, `RUST_LOG`).
Разработка (GUI): см. «Запуск GUI из исходников» выше.

---

<div align="center">

## ⭐ Поддержите проект

Если DirectDrop доставляет ваши файлы на телефон, не трогая роутер, поставьте звезду — это очень помогает.

<a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=social" alt="star this repo" /></a>

<a href="https://star-history.com/#DeveloperMatt02/DirectDrop&Date"><img src="https://api.star-history.com/svg?repos=DeveloperMatt02/DirectDrop&type=Date" width="600" alt="star history chart" /></a>

Нашли ошибку? [Откройте issue](https://github.com/DeveloperMatt02/DirectDrop/issues/new) — pull request'ы приветствуются.

</div>
