<div align="center">
  <a href="README.md"><img src="https://img.shields.io/badge/English-EN-555?style=flat-square" alt="English" /></a>
  <a href="README.it.md"><img src="https://img.shields.io/badge/Italiano-IT-555?style=flat-square" alt="Italiano" /></a>
  <a href="README.es.md"><img src="https://img.shields.io/badge/Español-ES-555?style=flat-square" alt="Español" /></a>
  <a href="README.fr.md"><img src="https://img.shields.io/badge/Français-FR-555?style=flat-square" alt="Français" /></a>
  <a href="README.de.md"><img src="https://img.shields.io/badge/Deutsch-DE-555?style=flat-square" alt="Deutsch" /></a>
  <a href="README.pt.md"><img src="https://img.shields.io/badge/Português-PT-555?style=flat-square" alt="Português" /></a>
  <a href="README.ru.md"><img src="https://img.shields.io/badge/Русский-RU-555?style=flat-square" alt="Русский" /></a>
  <img src="https://img.shields.io/badge/中文-ZH-blue?style=flat-square" alt="中文" />
  <a href="README.ja.md"><img src="https://img.shields.io/badge/日本語-JA-555?style=flat-square" alt="日本語" /></a>
</div>

<div align="center">
  <img src="app/main/src-tauri/icons/128x128.png" width="128" alt="DirectDrop icon" />
  <h1>DirectDrop</h1>
  <p><strong>面向 Linux 的 Quick Share（Nearby Share），无需两台设备连接同一 Wi-Fi——现已配备桌面 GUI。</strong></p>

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
  <a href="#快速安装终端"><img src="https://img.shields.io/badge/🐧_Quick_Install-22c55e?style=for-the-badge" alt="quick install" /></a>
  <a href="#截图"><img src="https://img.shields.io/badge/🖥️_Screenshots-a855f7?style=for-the-badge" alt="screenshots" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/issues/new"><img src="https://img.shields.io/badge/🐛_Report_Bug-f59e0b?style=for-the-badge" alt="report bug" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/badge/⭐_Star-facc15?style=for-the-badge&logo=github" alt="star" /></a>
</div>

> 上游 `quickshare-direct` 的一个分支。无头接收 / 发送协议栈保持不变（BLE 广播 / GATT 首次连接、Wi-Fi Direct GO / 客户端升级、LAN 回退、polkit 辅助工具、无头 CLI 守护进程）。本分支新增的是位于 `app/main/` 中的全新 Tauri 桌面 GUI，与 CLI 行为完全一致。

## 快速安装（终端）

通用一键命令——自动检测发行版，并从最新的 `DeveloperMatt02/DirectDrop` release 安装匹配的软件包（Fedora/RHEL 上为 `.rpm`，Debian/Ubuntu 上为 `.deb`，其他系统将 `.AppImage` 安装到 `~/.local/bin`）：

```bash
curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash
```

各发行版直接命令（同一来源，无需辅助脚本）：

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

或从[最新版本](https://github.com/DeveloperMatt02/DirectDrop/releases/latest)手动选择构建产物。

## 🖥️ 截图

主窗口（操作模式控制台：状态头、实时笔记本 ⇄ 手机流程、发送面板、设备看板、传输队列）：

![DirectDrop main window](docs/screenshots/gui-main.svg)

传入传输——UKEY2 PIN 验证卡片，带 60 秒倒计时和自动拒绝：

![DirectDrop consent card](docs/screenshots/gui-consent.svg)

> [!NOTE]
> 这些是应用蓝图风格的示意占位图。请将 `docs/screenshots/gui-main.svg` 和 `docs/screenshots/gui-consent.svg` 替换为真实截图（运行 `pnpm tauri dev`，截取窗口，保存为同目录下的 `.png` 并更新上方链接）。

上游快速上手流程仍然适用：

1. **通过 Bluetooth LE 发现 + 首次连接**（手机关闭 Wi-Fi 也可用）。
2. **加密握手**（UKEY2，双方均显示 PIN），经由该蓝牙链路完成。
3. **升级到 Wi-Fi 速度**：笔记本在保持正常 Wi-Fi 连接的同时创建一个 **Wi-Fi Direct 群组**（可继续上网），并将凭据交给手机；若手机已在同一 Wi-Fi 下，则经由本地网络传输。普通热点作为回退方案。
4. **桌面通知**：接受 / 拒绝（含 PIN），然后点击“打开文件夹”。

| | 状态 |
|---|---|
| 从 Android 接收（上游已测试：Pixel 10、Android 16） | 可用 |
| 手机关闭 Wi-Fi → Wi-Fi Direct（75 MB，约 5 MB/s） | 可用 |
| 手机在同一 Wi-Fi 下 → LAN | 可用 |
| Bluetooth Classic 首次连接 | 实验性 (`QSD_BT_CLASSIC=1`) |
| 发送到 Android → 手机的 Wi-Fi Direct（300 MB，约 36 MB/s） | 可用 (`quickshare-direct send`、file manager、GUI send pane) |
| 软件包：Debian/Ubuntu、Fedora、Arch、通用版（x86_64、aarch64） | 一键安装，见下文 |

---

## 本分支的新增内容：重新启用并重新设计的桌面 GUI（`app/main/`）

上游仓库在 `app/` 下保留了 rQuickShare 的旧 Tauri 应用，但并不构建、发布或测试它（其发送路径仅支持 LAN/mDNS，没有 Bluetooth / Wi-Fi Direct 路径）。本分支将这一想法重新启用为位于 `app/main/` 的全新专用 GUI，驱动与 CLI 相同的 `core_lib`（`rqs_lib`）引擎——没有并行的协议实现。

### 技术栈与外观

- **Tauri 2.2、Vue 3.5、Pinia 2.x、Tailwind CSS 3.4。**前端入口为 `app/main/src/App.vue`；状态位于 Pinia stores（`src/stores/session.ts`、`devices.ts`、`transfers.ts`）。
- **技术蓝图美学：**深色操作模式控制台、表格等宽字体指标、笔记本与手机之间的实时网络流程图（`FlowDiagram.vue` / `FlowHero.vue`）、1px 细线、无营销冗余。布局：`StatusHeader` → `FlowDiagram` → `SendPane` + `DeviceBoard` → `TransferQueue`，底部附带轻量接收端详情页脚（下载路径、端口）。

### 非侵入式进程内 Rust 桥接

GUI 在进程内运行 `rqs_lib`。边界恰好是两个新文件加一份版本化契约——**对核心协议引擎零改动**：

- `app/main/src-tauri/src/dto.rs` — 唯一的线缆格式。`core_lib` 中的核心领域类型刻意不派生 `Serialize`/`Deserialize`；所有 Tauri `invoke`/`emit` 负载都经过这些 serde DTO，并显式 `From` 映射。`DTO_VERSION = 1`，由 Rust 测试 `dto_version_matches_ts` 对照 TypeScript 镜像强制校验。
- `app/main/src-tauri/src/bridge.rs` — 订阅核心 `broadcast`/`watch` 通道（`message_sender`、`dch_sender`、`visibility_sender`、`ble_receiver`），并重新发射经节流 / 去重的 Tauri 事件。进度事件已节流（状态变化时发射，或增量 ≥256 KiB，或间隔 ≥500 ms；终止状态必定发射）。`Lagged` 广播溢出仅记录 debug 日志并跳过，绝不崩溃——与 CLI 策略相同。
- `app/main/src/types/qsd.ts` — `dto.rs` 的严格 TypeScript 镜像（事件名、`VisibilityDto`、包含全部 18 种核心状态的 `TransferStateDto`、`PayloadKindDto`、`TransferDto`、`EndpointDto`、`SendPayloadDto`、`ConsentDto`、`SnapshotDto`、错误码）。包含 `humanSize` / `describeTransfer`（CLI `describe()` 的移植）、`baseName` 和 `validateSendFiles` 辅助函数，以及 `TERMINAL_STATES` / `ACTIVE_PROGRESS_STATES` / `HANDSHAKE_STATES` 集合。

Tauri 命令（位于 `src-tauri/src/cmds/`，在 `main.rs` 中接线）：`qsd_snapshot`、`qsd_set_visibility`、`qsd_set_download_path`、`qsd_set_device_name`、`qsd_start_discovery`、`qsd_stop_discovery`、`qsd_consent`、`qsd_send_lan`、`qsd_get_hostname`（另有旧版 `get_hostname` / `change_download_path` 兼容层）。后端 → 前端事件：`qsd://transfer`、`qsd://endpoints`、`qsd://visibility`、`qsd://ble-presence`。

### 与 CLI 完全一致的行为

- **即时 UKEY2 PIN 验证卡片**（`ConsentCard.vue`）：当核心进入 `WaitingForUserConsent` 即显示发送方名称、设备类型、PIN、负载摘要和文件列表。**60 秒倒计时**环（`QSD_CONSENT_TIMEOUT_MS`，遵循在 `dto.rs` 中读取的 `QSD_CONSENT_TIMEOUT` 环境变量覆盖）到期后卡片失效；到期触发与 CLI 相同的**自动拒绝**（向通道发送 `ConsentDecline`）。接受 / 拒绝 / 取消经由 `ConsentDto::into_channel_message` 与核心 `TransferAction` 一一映射。
- **三态可见性选择器**（`StatusHeader.vue` → `qsd_set_visibility`）：`Visible`、`Temporarily`（60 秒，带琥珀色脉冲圆点和剩余时间显示）、`Hidden`（`Invisible`）。Temporarily→Invisible 计时器位于核心 `mdns.rs`；GUI 仅反映 `qsd://visibility` 事件并经由 Tauri store 持久化。不在 GUI 侧重新实现计时器。
- **被动 BLE 在场提醒横幅：**核心 BLE 监听器发现以节流（120 秒）的 `qsd://ble-presence` 事件到达。当处于隐藏（`Invisible`）时，桥接层还会触发系统通知，提示开启临时可见窗口；前端显示 info 提示（“附近有手机正在分享——请检查可见性”）。遵循核心 `QSD_BLE_SCAN` / `QSD_NO_BLE_SCAN` 语义。
- **内联文本 / 链接传输处理，无需强制落盘**（`TransferCard.vue`）：`Text`、`Url` 和 `WiFi` 负载以内联预览加剪贴板复制呈现（Tauri clipboard-manager 插件）；显示时不向磁盘写入文件。文件负载显示接收文件数、文件名和字节精确进度（`ack_bytes` / `total_bytes`）。
- **出站 LAN/mDNS 文件发送，带常规文件校验**（`SendPane.vue` + `qsd_send_lan`）：经由对话框或拖放暂存文件，选择发现到的 mDNS 端点（`EndpointDto`，`addr = ip:port`），并经由核心 `sender_file` 通道以 `SendInfo { ob: Files }` 发送。校验与 CLI 一致：空选择和目录会被以 `VALIDATION` 错误拒绝（`{f}: not a regular file (folders aren't supported)`）；前端在调用后端前对明显情况快速失败。

### 从源码运行 GUI

```bash
cd app/main
pnpm install
pnpm tauri dev     # live GUI against local core_lib (path dep: ../../core_lib)
pnpm ts-check       # vue-tsc --noEmit contract check
pnpm test           # vitest suite (tests/)
pnpm build          # production Tauri bundle
```

Rust 侧：`cd app/main/src-tauri && cargo check` / `cargo test`（含 DTO 线缆格式测试）。系统前置依赖与守护进程相同（BlueZ、NetworkManager、libdbus 头文件、protoc、cargo ≥ 1.85），另需 Node/pnpm 和 Tauri 2.x Linux 依赖。

---

## 架构与渊源 / 致谢

本分支的分层视图：

```
Android phone ──BLE adv / GATT──▶ core_lib (rqs_lib) ◀──Tauri bridge──▶ Vue GUI
                                        │  (unchanged engine)
                                        ▼
                               Wi-Fi Direct GO / client,
                               LAN, hotspot fallback,
                               polkit Wi-Fi helpers
```

- **上游 `quickshare-direct` 作者：**本分支依赖的核心离网传输——用于发现 / 首次连接的 BLE 广播监听与 GATT 服务、在保持正常 Wi-Fi 连接的同时升级 Wi-Fi Direct 群主 / 客户端（带单通道回退处理）、按需 `quickshare-ap@USER` / `quickshare-join@USER` root 辅助工具及其 polkit 规则，以及带桌面通知的无头 `quickshare-direct` CLI 守护进程。所有协议、蓝牙和 Wi-Fi 工程功劳均归于此；本分支未做任何修改。
- **`rQuickShare`（Martichou，贡献者包括 nozwock 和 martinalderson）：**Linux 上 Nearby Share / Quick Share 协议的奠基性 Rust/Tauri 实现，上游与本分支均基于此构建——包括 `rqs_lib` 核心、原始 Tauri 应用结构，以及延续至 `core_lib` 的蓝牙发现工作。
- **Google Nearby（Apache-2.0）：**定义 UKEY2 握手、连接、介绍和负载帧的协议规范与 protobuf 定义。本仓库许可证与 rQuickShare 一样，仍为 GPL-3.0。

---

### 安装（守护进程 + CLI 软件包）

Debian 12+ / Ubuntu 22.04+（及 Mint、Pop!_OS 等）、Fedora、Arch（及 Manjaro、EndeavourOS 等），其他系统使用通用构建；支持 x86_64 和 aarch64：

```
curl -fsSL https://raw.githubusercontent.com/papshmeare/quickshare-direct/main/install.sh | sh
```

它会用你的包管理器安装最新的 [release](https://github.com/papshmeare/quickshare-direct/releases)（`.deb`、`.rpm`、`.pkg.tar.zst`；依赖来自你的发行版），若 firewalld 或 ufw 处于活动状态则开放 Quick Share 端口，并启动接收端。再次运行同一命令即可更新；`curl -fsSL .../install.sh | sh -s -- --uninstall` 可卸载。想先看清楚再装？下载 `install.sh` 通读一遍，或从发布页手动安装软件包（`sudo apt install ./quickshare-direct_amd64.deb`、`sudo dnf install ./quickshare-direct.x86_64.rpm`、`sudo pacman -U quickshare-direct-x86_64.pkg.tar.zst`）。

安装内容包括：

- `quickshare-direct`：接收端（每个图形会话中的用户服务，带通知）以及 `send` / `devices` 命令；
- 两个小型 root 辅助工具，经由 systemd 按需启动（接收时创建直连 Wi-Fi 链路的 `quickshare-ap@USER`，发送时加入手机群组的 `quickshare-join@USER`），以及一条 polkit 规则（`packaging/linux/polkit/50-quickshare-direct.rules`），允许活动本地用户仅启停自己的实例（`quickshare-ap@USER.service` / `quickshare-join@USER.service` 的 `start`/`stop`/`restart`）；
- 一个 NetworkManager drop-in（不对临时链路接口做处理）、一个保持蓝牙可连接的单元、firewalld/ufw 服务定义和文件管理器入口。

需要：systemd、BlueZ、带 wpa_supplicant 的 NetworkManager（这些发行版默认即是；不是 iwd），以及可在正常连接旁运行第二个接口的 Wi-Fi 网卡（大多数 Intel/MediaTek/Qualcomm 网卡均可）。通知需要支持操作按钮的守护进程（GNOME、KDE、swaync、mako 等）。Ubuntu 22.04 的旧 polkit 无法让用户启动辅助工具，因此仅支持蓝牙和同网段传输。请设置 Wi-Fi 监管域（`sudo iw reg set XX`）：使用默认 world 域时 Linux 无法在 5 GHz 上启动群主。设置：接收端用 `~/.config/quickshare-direct/env`（`QSD_NAME=...`、`QSD_DIR=...`），辅助工具用 `/etc/default/quickshare-direct`。许多 Wi-Fi 芯片无法在正常连接旁的第二个信道上承载直连链路，当路由器使用的信道是笔记本可能无法承载的信道时尤为重要（雷达 / DFS 信道，如 100）：此时笔记本会在传输的几秒钟内离开你的 Wi-Fi 网络，之后重新连接（`QS_SINGLE_CHANNEL=auto`；发送时全速用 `always`，保持连接并回退到蓝牙用 `never`）。

从源码构建：`make && sudo make install`（需要 cargo ≥ 1.85、protoc、libdbus 头文件），然后 `systemctl --user enable --now quickshare-direct`。

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
这将配置接收端（图形会话中的用户服务）、按需创建直连 Wi-Fi 链路的 root 辅助工具（`quickshare-ap.service`，该用户可经由 polkit 启动）、发送时加入手机 Wi-Fi Direct 群组的辅助工具（`quickshare-join.service`）、防火墙端口、蓝牙“可连接”，并让 NetworkManager 远离链路接口。请设置 Wi-Fi 监管域（`iw reg set XX`）：使用默认 world 域时 Linux 无法在 5 GHz 上启动接入点 / 群主。

### 发送（CLI）

在手机上打开 Quick Share 并点击 **Receive**（或将其设为对所有人可见），然后：

```
quickshare-direct send photo.jpg notes.pdf      # to the one phone in receive mode nearby
quickshare-direct send --to Pixel video.mp4     # pick by name when several are around
quickshare-direct devices                       # list phones in receive mode
```
它经由蓝牙找到手机，显示 PIN，在手机上接受后，文件即经由手机的 Wi-Fi Direct 群组传输（在第二个接口上加入，你的 Wi-Fi 保持连接）。在文件管理器中：Thunar 的 **Send To → Phone (Quick Share)**（任意文件），或打开方式 → “Send with Quick Share”（常见文件类型）；经由通知报告进度。GUI 的 `SendPane` 经由 `qsd_send_lan` 执行相同的 LAN/mDNS 发送，具有相同的 PIN 和常规文件语义。

### 环境变量（接收端、发送端、辅助工具）

核心行为经由环境变量配置（见 `core_lib/src/bin/quickshare-direct.rs`、`core_lib/src/hdl/*.rs`）：

| Variable | Default | Effect |
|---|---|---|
| `QSD_DIR` | XDG Downloads, else `~/Downloads` | 下载文件夹 |
| `QSD_NAME` | hostname | 在手机上显示的设备名称 |
| `QSD_PORT` | random | 传入传输的 TCP 端口 |
| `QSD_BWU_PORT` | random | Wi-Fi 升级监听器的 TCP 端口 |
| `QSD_CONSENT_TIMEOUT` | `60` (s) | 等待接受 / 拒绝直至自动拒绝的秒数；同时驱动 GUI 倒计时 |
| `QSD_BWU` | `auto` | 入站升级偏好：`lan` / `hotspot` 强制使用其一 |
| `QSD_BLE_SCAN` | passive BlueZ advertisement monitor | `discovery` = 常开 LE 发现（可能阻止已配对的 LE 鼠标 / 键盘重连）；`off` 禁用监听器 |
| `QSD_NO_BLE_SCAN` | unset | 设置后完全禁用发送端 BLE 检测 |
| `QSD_BT_CLASSIC` | unset | 设置后（`=1`）启用实验性 Bluetooth Classic 首次连接 |
| `QSD_BLE_L2CAP` / `QSD_L2CAP_CHUNK` | on / default | 发送路径 BLE L2CAP 调优（`=0` 强制使用 weave socket） |
| `QSD_WEAVE_ATT` / `QSD_WEAVE_PACKET` | on / MTU default | GATT/weave 传输调优，用于实验 |
| `QSD_SEND_MEDIUMS` | wifi_lan-first default | 逗号分隔的出站介质偏好（`wifi_lan` 等） |
| `QS_SINGLE_CHANNEL` | `auto` | `always` = 发送时全速（可能断开 station）；`never` = 保持连接，回退到蓝牙 |
| `QSD_AP_IFACE` / `QSD_STA_IFACE` | `ap0` / auto | AP 辅助工具的链路接口名 |
| `QSD_AP_UNIT` / `QSD_JOIN_UNIT` | `quickshare-ap` / `quickshare-join` | root 辅助工具的 systemd 单元名 |
| `QSD_SYSTEMCTL` | `systemctl` | 辅助工具启动命令覆盖 |
| `QSD_AP_CREDENTIALS` | `/run/quickshare/credentials` | AP 辅助工具的凭据落盘路径 |
| `QSD_JOIN_REQUEST` / `QSD_JOIN_RESULT` | `/run/quickshare-join/...` | join 辅助工具的请求 / 结果路径 |
| `RUST_LOG` | `info` (`warn` for send/devices) | 日志过滤器 |

接收端文件：`~/.config/quickshare-direct/env`（`QSD_NAME`、`QSD_DIR` 等）。辅助工具文件：`/etc/default/quickshare-direct`。

### 工作原理 / 研究笔记

[docs/DEV_NOTES.md](docs/DEV_NOTES.md)（发现、手机日志、协议细节）和 [docs/BLE_RECEIVER_DISCOVERY.md](docs/BLE_RECEIVER_DISCOVERY.md)（BLE 接收端路径）。

版本发布（本分支）：推送 `v*.*.*` 标签即运行 [`.github/workflows/release.yml`](.github/workflows/release.yml)，构建 Tauri GUI 软件包（`.deb`、`.rpm`、`.AppImage`）和 `directdrop-cli-linux-x86_64.tar.gz` 守护进程压缩包，并发布到 GitHub Release——可经由上文 `get-directdrop.sh` 安装。上游守护进程软件包保持各自流程：CI（`.github/workflows/packages.yml`）在每次推送时构建并安装测试软件包；在 `main` 上递增 `VERSION` 即发布 `qsd-v<VERSION>` 版本。

仓库布局：`core_lib`（库 + `quickshare-direct` 二进制）与 `packaging/`；软件包、NixOS 模块和 CI 构建所需仅此而已。`app/main/` 是本分支重新启用的桌面 GUI（Tauri + Vue）：经由 `src-tauri/src/dto.rs` + `bridge.rs` 和 `src/types/qsd.ts` 契约在进程内封装 `core_lib`，不触碰核心。上游其他遗留物（`snap/`、`BUILD.md`、`CHANGELOG.md`、release-please、`build`/`lint` 工作流）仍指向其上游 `master`。

开发（无头模式）：`nix develop`，然后在 `core_lib` 中：`cargo run --bin quickshare-direct`（环境变量：`QSD_DIR`、`QSD_NAME`、`QSD_PORT`、`QSD_BWU_PORT`、`QSD_BWU=lan|hotspot`、`RUST_LOG`）。开发（GUI）：见上文“从源码运行 GUI”。

---

<div align="center">

## ⭐ 支持我们

如果 DirectDrop 让你无需经过路由器就能把文件传到手机，请给它一颗星——这帮助很大。

<a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=social" alt="star this repo" /></a>

<a href="https://star-history.com/#DeveloperMatt02/DirectDrop&Date"><img src="https://api.star-history.com/svg?repos=DeveloperMatt02/DirectDrop&type=Date" width="600" alt="star history chart" /></a>

发现 bug？[Open an issue](https://github.com/DeveloperMatt02/DirectDrop/issues/new)——欢迎提交 pull request。

</div>
