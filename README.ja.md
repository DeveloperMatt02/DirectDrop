<div align="center">
  <img src="https://img.shields.io/badge/日本語-JA-blue?style=flat-square" alt="日本語" />
  <a href="README.md"><img src="https://img.shields.io/badge/English-EN-555?style=flat-square" alt="English" /></a>
  <a href="README.it.md"><img src="https://img.shields.io/badge/Italiano-IT-555?style=flat-square" alt="Italiano" /></a>
  <a href="README.es.md"><img src="https://img.shields.io/badge/Español-ES-555?style=flat-square" alt="Español" /></a>
  <a href="README.fr.md"><img src="https://img.shields.io/badge/Français-FR-555?style=flat-square" alt="Français" /></a>
  <a href="README.de.md"><img src="https://img.shields.io/badge/Deutsch-DE-555?style=flat-square" alt="Deutsch" /></a>
  <a href="README.pt.md"><img src="https://img.shields.io/badge/Português-PT-555?style=flat-square" alt="Português" /></a>
  <a href="README.ru.md"><img src="https://img.shields.io/badge/Русский-RU-555?style=flat-square" alt="Русский" /></a>
  <a href="README.zh.md"><img src="https://img.shields.io/badge/中文-ZH-555?style=flat-square" alt="中文" /></a>
</div>

<div align="center">
  <img src="app/main/src-tauri/icons/128x128.png" width="128" alt="DirectDrop icon" />
  <h1>DirectDrop</h1>
  <p><strong>Linux 向け Quick Share（Nearby Share）です。両方のデバイスが同じ Wi-Fi に接続している必要はありません — デスクトップ GUI 付きです。</strong></p>

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
  <a href="#クイックインストールターミナル"><img src="https://img.shields.io/badge/🐧_Quick_Install-22c55e?style=for-the-badge" alt="quick install" /></a>
  <a href="#スクリーンショット"><img src="https://img.shields.io/badge/🖥️_Screenshots-a855f7?style=for-the-badge" alt="screenshots" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/issues/new"><img src="https://img.shields.io/badge/🐛_Report_Bug-f59e0b?style=for-the-badge" alt="report bug" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/badge/⭐_Star-facc15?style=for-the-badge&logo=github" alt="star" /></a>
</div>

> 上流の `quickshare-direct` のフォークです。ヘッドレスの受信 / 送信スタックは変更なく維持しています（BLE アドバタイズ / GATT によるファーストコンタクト、Wi-Fi Direct の GO / クライアントへのアップグレード、LAN フォールバック、polkit ヘルパー、ヘッドレス CLI デーモン）。このフォークが追加するのは、`app/main/` に再設計された Tauri 製デスクトップ GUI で、CLI と完全に同等の動作を備えています。

## クイックインストール（ターミナル）

汎用ワンライナーです — ディストリビューションを検出して、最新の `DeveloperMatt02/DirectDrop` リリースから対応するバンドルをインストールします（Fedora / RHEL では `.rpm`、Debian / Ubuntu では `.deb`、それ以外では `.AppImage` を `~/.local/bin` に導入します）：

```bash
curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash
```

ディストリビューションごとの直接コマンドです（同じソースで、ヘルパースクリプトは使いません）：

```bash
# Fedora / RHEL (.rpm)
URL=$(curl -fsSL https://api.github.com/repos/DeveloperMatt02/DirectDrop/releases/latest \
  | grep -o 'https://[^"]*\.rpm' | head -n 1)
sudo dnf install -y "$URL"

# Debian / Ubuntu (.deb)
URL=$(curl -fsSL https://api.github.com/repos/DeveloperMatt02/DirectDrop/releases/latest \
  | grep -o 'https://[^"]*\.deb' | head -n 1)
curl -fL -o /tmp/directdrop.deb "$URL"
sudo apt-get install -y /tmp/directdrop.deb
```

または[最新リリース](https://github.com/DeveloperMatt02/DirectDrop/releases/latest)からアセットを手動で選択してください。

## 🖥️ スクリーンショット

メインウィンドウです（オペレートモードのコンソール：ステータスヘッダー、ラップトップ ⇄ スマートフォンのライブフロー、送信ペイン、デバイスボード、転送キュー）：

![DirectDrop main window](docs/screenshots/gui-main.svg)

受信転送です — 60 秒カウントダウンと自動辞退付きの UKEY2 PIN 確認カード：

![DirectDrop consent card](docs/screenshots/gui-consent.svg)

> [!NOTE]
> これらはアプリの設計図スタイルによる概略プレースホルダーです。`docs/screenshots/gui-main.svg` と `docs/screenshots/gui-consent.svg` を実際のキャプチャに置き換えてください（`pnpm tauri dev` でウィンドウを撮影し、それらの隣に `.png` として保存して上記のリンクを更新します）。

上流のクイックスタートは引き続き有効です：

1. **Bluetooth LE 経由の検出 + ファーストコンタクト**です（スマートフォンが Wi-Fi オフでも動作します）。
2. その Bluetooth リンク上で**暗号化ハンドシェイク**を行います（UKEY2、PIN は両側に表示されます）。
3. **Wi-Fi の速度にアップグレード**します：ラップトップは通常の Wi-Fi 接続の横に **Wi-Fi Direct グループ**を作成し（通常接続は維持されます）、認証情報をスマートフォンに渡します。スマートフォンが同じ Wi-Fi に接続している場合は、転送がローカルネットワークに移行します。通常のホットスポットがフォールバックです。
4. **デスクトップ通知**です：Accept / Decline（PIN 付き）を選択し、その後「フォルダーを開く」を実行します。

| | ステータス |
|---|---|
| Android から受信します（上流でのテスト：Pixel 10、Android 16） | 動作します |
| スマートフォンが Wi-Fi オフ → Wi-Fi Direct（75 MB、~5 MB/s） | 動作します |
| スマートフォンが同じ Wi-Fi 上 → LAN | 動作します |
| Bluetooth Classic によるファーストコンタクト | 実験的です（`QSD_BT_CLASSIC=1`） |
| Android へ送信 → スマートフォンの Wi-Fi Direct（300 MB、~36 MB/s） | 動作します（`quickshare-direct send`、ファイルマネージャー、GUI 送信ペイン） |
| パッケージ：Debian / Ubuntu、Fedora、Arch、汎用（x86_64、aarch64） | ワンラインインストールで、以下を参照してください |

---

## このフォークの新機能：復活・再設計したデスクトップ GUI（`app/main/`）

上流リポジトリは rQuickShare の旧 Tauri アプリを `app/` 配下に保持していますが、ビルドも配布もテストもしていません（その送信パスは LAN / mDNS 専用で、Bluetooth / Wi-Fi Direct パスがありません）。このフォークでは、その発想を `app/main/` の新規・専用 GUI として復活させ、CLI と**同じ** `core_lib`（`rqs_lib`）エンジンを駆動します — 並行したプロトコル実装はありません。

### スタックと外観

- **Tauri 2.2、Vue 3.5、Pinia 2.x、Tailwind CSS 3.4 です。**フロントエンドのエントリは `app/main/src/App.vue` です。状態は Pinia ストア（`src/stores/session.ts`、`devices.ts`、`transfers.ts`）に保持します。
- **テクニカルな設計図の美学です：**ダークなオペレートモードコンソール、表形式の等幅メトリクス、ラップトップとスマートフォン間のライブネットワークフロー図（`FlowDiagram.vue` / `FlowHero.vue`）、1px のヘアライン、宣伝的な装飾はありません。レイアウトは `StatusHeader` → `FlowDiagram` → `SendPane` + `DeviceBoard` → `TransferQueue` で、薄い受信者詳細フッター（ダウンロードパス、ポート）付きです。

### 非侵襲的なインプロセス Rust ブリッジ

GUI は `rqs_lib` をインプロセスで実行します。境界は新規ファイルちょうど 2 つとバージョン管理されたコントラクトであり — **コアプロトコルエンジンへの変更はゼロ**です：

- `app/main/src-tauri/src/dto.rs` — 唯一のワイヤーフォーマットです。`core_lib` のコアドメイン型は意図的に `Serialize` / `Deserialize` を導出しません。すべての Tauri `invoke` / `emit` ペイロードは、明示的な `From` マッピングを持つこれらの serde DTO を経由します。`DTO_VERSION = 1` で、TypeScript 側ミラーに対する Rust テスト `dto_version_matches_ts` によって強制されます。
- `app/main/src-tauri/src/bridge.rs` — コアの `broadcast` / `watch` チャネル（`message_sender`、`dch_sender`、`visibility_sender`、`ble_receiver`）を購読し、スロットル / 重複排除した Tauri イベントを再送出します。進捗イベントはスロットルされます（状態変化時、または ≥256 KiB の差分、または ≥500 ms で送出し、終端状態は常に送出します）。`Lagged` によるブロードキャストオーバーフローはデバッグログでスキップし、クラッシュしません — CLI と同じポリシーです。
- `app/main/src/types/qsd.ts` — `dto.rs` の厳密な TypeScript ミラーです（イベント名、`VisibilityDto`、18 のコア状態すべてを持つ `TransferStateDto`、`PayloadKindDto`、`TransferDto`、`EndpointDto`、`SendPayloadDto`、`ConsentDto`、`SnapshotDto`、エラーコード）。CLI の `describe()` を移植した `humanSize` / `describeTransfer`、`baseName`、`validateSendFiles` ヘルパーに加え、`TERMINAL_STATES` / `ACTIVE_PROGRESS_STATES` / `HANDSHAKE_STATES` セットを含みます。

公開されている Tauri コマンドです（`src-tauri/src/cmds/`、`main.rs` で配線）：`qsd_snapshot`、`qsd_set_visibility`、`qsd_set_download_path`、`qsd_set_device_name`、`qsd_start_discovery`、`qsd_stop_discovery`、`qsd_consent`、`qsd_send_lan`、`qsd_get_hostname`（および旧来の `get_hostname` / `change_download_path` シム）。バックエンド → フロントエンドのイベント：`qsd://transfer`、`qsd://endpoints`、`qsd://visibility`、`qsd://ble-presence` です。

### CLI との完全な動作互換

- **即時の UKEY2 PIN 確認カード**です（`ConsentCard.vue`）：送信者名、デバイスタイプ、PIN、ペイロード概要、ファイルリストが、コアが `WaitingForUserConsent` に到達次第表示されます。**60 秒カウントダウン**リング（`dto.rs` で読み取る `QSD_CONSENT_TIMEOUT` 環境オーバーライドを尊重する `QSD_CONSENT_TIMEOUT_MS`）でカードが期限切れになります。有効期限切れでは CLI と同じ**自動辞退**（チャネル上の `ConsentDecline`）が実行されます。Accept / Decline / Cancel は `ConsentDto::into_channel_message` 経由でコアの `TransferAction` に 1 対 1 で対応します。
- **3 状態の公開範囲セレクター**です（`StatusHeader.vue` → `qsd_set_visibility`）：`Visible`、`Temporarily`（60 秒、琥珀色パルスドットと残り時間表示付き）、`Hidden`（`Invisible`）です。Temporarily → Invisible タイマーはコアの `mdns.rs` に存在します。GUI は `qsd://visibility` イベントを反映し、Tauri ストア経由で保持するだけです。GUI 側でのタイマー再実装はありません。
- **パッシブ BLE プレゼンス通知バナーです：**コア BLE リスナーの検出は、スロットルされた（120 秒）`qsd://ble-presence` イベントとして到着します。非公開（`Invisible`）時には、ブリッジが一時的公開ウィンドウを促すシステム通知を追加で発火します。フロントエンドは情報トーストを表示します（「近くのスマートフォンが共有中です — 公開設定を確認してください」）。コアの `QSD_BLE_SCAN` / `QSD_NO_BLE_SCAN` セマンティクスを尊重します。
- **強制ディスク書き込みなしのインラインテキスト / URL 転送処理**です（`TransferCard.vue`）：`Text`、`Url`、`WiFi` ペイロードはプレビュー付きでインライン表示し、クリップボードコピー（Tauri clipboard-manager プラグイン）に対応します。表示のためにファイルがディスクに書き込まれることはありません。ファイルペイロードは受信ファイル数、名前、バイト正確な進捗（`ack_bytes` / `total_bytes`）を表示します。
- **通常ファイル検証付きのアウトバウンド LAN / mDNS ファイル送信**です（`SendPane.vue` + `qsd_send_lan`）：ダイアログまたはドラッグ＆ドロップでファイルをステージし、検出された mDNS エンドポイント（`EndpointDto`、`addr = ip:port`）を選択して、コアの `sender_file` チャネル経由で `SendInfo { ob: Files }` として送信します。検証は CLI と一致します：空の選択とディレクトリは `VALIDATION` エラー（`{f}: not a regular file (folders aren't supported)`）で拒否されます。フロントエンドはバックエンド呼び出し前に明らかなケースを先行して失敗させます。

### ソースから GUI を実行する

```bash
cd app/main
pnpm install
pnpm tauri dev     # live GUI against local core_lib (path dep: ../../core_lib)
pnpm ts-check       # vue-tsc --noEmit contract check
pnpm test           # vitest suite (tests/)
pnpm build          # production Tauri bundle
```

Rust 側：`cd app/main/src-tauri && cargo check` / `cargo test` です（DTO ワイヤーフォーマットテストを含みます）。システム前提条件はデーモンと同じです（BlueZ、NetworkManager、libdbus ヘッダー、protoc、cargo ≥ 1.85）に加え、Node / pnpm と Tauri 2.x Linux 依存関係が必要です。

---

## アーキテクチャと系譜 / クレジット

このフォークの層別ビューです：

```
Android phone ──BLE adv / GATT──▶ core_lib (rqs_lib) ◀──Tauri bridge──▶ Vue GUI
                                        │  (unchanged engine)
                                        ▼
                               Wi-Fi Direct GO / client,
                               LAN, hotspot fallback,
                               polkit Wi-Fi helpers
```

- **上流 `quickshare-direct` の作者：**このフォークが依存するオフネットワーク転送のコアです — 検出 / ファーストコンタクト用の BLE アドバタイズ監視と GATT サーバー、通常の Wi-Fi 接続と並行した Wi-Fi Direct グループオーナー / クライアントへのアップグレード（シングルチャネルフォールバック処理付き）、オンデマンド root ヘルパー `quickshare-ap@USER` / `quickshare-join@USER` とその polkit ルール、デスクトップ通知付きヘッドレス `quickshare-direct` CLI デーモンです。プロトコル、Bluetooth、Wi-Fi のすべてのエンジニアリングクレジットはここに帰属します。このフォークはそれらを一切変更していません。
- **Martichou による `rQuickShare`（nozwock や martinalderson などのコントリビューター）：**上流とこのフォークの両方が基づく、Linux 上の Nearby Share / Quick Share プロトコルの基礎的な Rust / Tauri 実装です — `rqs_lib` コア、元の Tauri アプリ構造、`core_lib` に引き継がれた Bluetooth 検出作業を含みます。
- **Google Nearby（Apache-2.0）：** UKEY2 ハンドシェイク、接続、紹介、ペイロードフレームを定義するプロトコル仕様と protobuf 定義です。このリポジトリのライセンスは rQuickShare と同様に GPL-3.0 のままです。

---

### インストール（デーモン + CLI パッケージ）

Debian 12+ / Ubuntu 22.04+（および Mint、Pop!_OS など）、Fedora、Arch（および Manjaro、EndeavourOS など）、その他は汎用ビルドで対応します。x86_64 と aarch64 です：

```
curl -fsSL https://raw.githubusercontent.com/papshmeare/quickshare-direct/main/install.sh | sh
```

お使いのパッケージマネージャーで最新の[リリース](https://github.com/papshmeare/quickshare-direct/releases)をインストールします（`.deb`、`.rpm`、`.pkg.tar.zst`。依存関係はディストリビューションから取得します）。firewalld または ufw が有効な場合は Quick Share ポートを開き、受信側を起動します。同じ行を再度実行すると更新されます。`curl -fsSL .../install.sh | sh -s -- --uninstall` で削除します。事前に内容を確認したいですか？ `install.sh` をダウンロードして読むか、リリースページからパッケージを手動でインストールしてください（`sudo apt install ./quickshare-direct_amd64.deb`、`sudo dnf install ./quickshare-direct.x86_64.rpm`、`sudo pacman -U quickshare-direct-x86_64.pkg.tar.zst`）。

インストールされる内容：
- `quickshare-direct`：受信側です（すべてのグラフィカルセッションにおけるユーザーサービスで、通知付き）と `send` / `devices` コマンドです。
- 2 つの小さな root ヘルパーで、systemd 経由でオンデマンド起動します（受信時に直接 Wi-Fi リンクを作成する `quickshare-ap@USER`、送信時にスマートフォン側に参加する `quickshare-join@USER`）。polkit ルール（`packaging/linux/polkit/50-quickshare-direct.rules`）により、アクティブなローカルユーザーは自分自身のインスタンスのみ開始 / 停止できます（`quickshare-ap@USER.service` / `quickshare-join@USER.service` 上の `start` / `stop` / `restart`）。
- 一時リンクインターフェースを除外する NetworkManager ドロップイン、Bluetooth を接続可能に保つユニット、firewalld / ufw サービス定義、ファイルマネージャーエントリです。

必要なもの：systemd、BlueZ、wpa_supplicant 付き NetworkManager（これらディストリビューションのデフォルトです。iwd ではありません）、通常接続の横で第二インターフェースを実行できる Wi-Fi カード（ほとんどの Intel / MediaTek / Qualcomm カード）です。通知にはアクションボタンを表示するデーモンが必要です（GNOME、KDE、swaync、mako など）。Ubuntu 22.04 の古い polkit ではユーザーがヘルパーを起動できないため、Bluetooth と同一ネットワーク転送のみ動作します。Wi-Fi レギュラトリドメインを設定してください（`sudo iw reg set XX`）：デフォルトの world ドメインでは Linux は 5 GHz でグループオーナーを開始しません。設定：受信側用に `~/.config/quickshare-direct/env`（`QSD_NAME=...`、`QSD_DIR=...`）、ヘルパー用に `/etc/default/quickshare-direct` です。多くの Wi-Fi チップは通常接続の横で別チャネル上に直接リンクをホストできず、これはルーターがラップトップがホストできないチャネル（100 のようなレーダー / DFS チャネル）を使用している場合に重要です。その場合ラップトップは転送の数秒間 Wi-Fi ネットワークから離脱し、その後再接続します（`QS_SINGLE_CHANNEL=auto`。送信時もフルスピードにするには `always`、接続を維持して Bluetooth にフォールバックするには `never`）。
 
ソースから：`make && sudo make install` です（cargo ≥ 1.85、protoc、libdbus ヘッダーが必要です）。その後 `systemctl --user enable --now quickshare-direct` を実行します。

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
これは受信側（グラフィカルセッション内のユーザーサービス）、オンデマンドで直接 Wi-Fi リンクを作成する root ヘルパー（polkit 経由でそのユーザーが起動できる `quickshare-ap.service`）、送信時にスマートフォンの Wi-Fi Direct グループに参加するヘルパー（`quickshare-join.service`）、ファイアウォールポート、Bluetooth「接続可能」、NetworkManager をリンクインターフェースから除外する設定を行います。Wi-Fi レギュラトリドメインを設定してください（`iw reg set XX`）：デフォルトの world ドメインでは Linux は 5 GHz でアクセスポイント / グループオーナーを開始しません。

### 送信（CLI）

スマートフォンで Quick Share を開き、**Receive**（または全員に公開）をタップしてから：

```
quickshare-direct send photo.jpg notes.pdf      # to the one phone in receive mode nearby
quickshare-direct send --to Pixel video.mp4     # pick by name when several are around
quickshare-direct devices                       # list phones in receive mode
```
スマートフォンを Bluetooth 経由で検出し、PIN を表示します。スマートフォン側で承認すると、ファイルはスマートフォンの Wi-Fi Direct グループ経由で送信されます（第二インターフェースで参加し、Wi-Fi は接続維持されます）。ファイルマネージャーでは：Thunar の **Send To → Phone (Quick Share)**（任意のファイル）、または Open With →「Send with Quick Share」（一般的なファイルタイプ）です。通知経由で報告します。GUI の `SendPane` は `qsd_send_lan` 経由で同じ LAN / mDNS 送信を、同一の PIN と通常ファイルセマンティクスで実行します。

### 環境変数（受信・送信・ヘルパー）

コア動作は環境経由で設定します（`core_lib/src/bin/quickshare-direct.rs`、`core_lib/src/hdl/*.rs` を参照してください）：

| Variable | Default | Effect |
|---|---|---|
| `QSD_DIR` | XDG Downloads, else `~/Downloads` | ダウンロードフォルダーです |
| `QSD_NAME` | hostname | スマートフォンに表示されるデバイス名です |
| `QSD_PORT` | random | 受信転送用 TCP ポートです |
| `QSD_BWU_PORT` | random | Wi-Fi アップグレードリスナー用 TCP ポートです |
| `QSD_CONSENT_TIMEOUT` | `60` (s) | Accept / Decline 応答までの秒数で、自動辞退します。GUI カウントダウンも駆動します |
| `QSD_BWU` | `auto` | 受信アップグレード設定です：`lan` / `hotspot` で 1 つに強制します |
| `QSD_BLE_SCAN` | passive BlueZ advertisement monitor | `discovery` = 常時 LE 検出です（ボンデッド LE マウス / キーボードの再接続を妨げる場合があります）。`off` でリスナーを無効化します |
| `QSD_NO_BLE_SCAN` | unset | 設定時は送信側 BLE 検出を完全に無効化します |
| `QSD_BT_CLASSIC` | unset | 設定時（`=1`）は実験的な Bluetooth Classic ファーストコンタクトを有効化します |
| `QSD_BLE_L2CAP` / `QSD_L2CAP_CHUNK` | on / default | 送信パス BLE L2CAP 調整用です（`=0` で weave ソケットを強制します） |
| `QSD_WEAVE_ATT` / `QSD_WEAVE_PACKET` | on / MTU default | 実験用の GATT / weave 転送調整です |
| `QSD_SEND_MEDIUMS` | wifi_lan-first default | カンマ区切りの送信メディア設定です（`wifi_lan` など） |
| `QS_SINGLE_CHANNEL` | `auto` | `always` = 送信時もフルスピードです（station が切断される場合があります）。`never` = 接続を維持し Bluetooth にフォールバックします |
| `QSD_AP_IFACE` / `QSD_STA_IFACE` | `ap0` / auto | AP ヘルパー用リンクインターフェース名です |
| `QSD_AP_UNIT` / `QSD_JOIN_UNIT` | `quickshare-ap` / `quickshare-join` | root ヘルパー用 systemd ユニット名です |
| `QSD_SYSTEMCTL` | `systemctl` | ヘルパー起動コマンドのオーバーライドです |
| `QSD_AP_CREDENTIALS` | `/run/quickshare/credentials` | AP ヘルパー用認証情報ドロップパスです |
| `QSD_JOIN_REQUEST` / `QSD_JOIN_RESULT` | `/run/quickshare-join/...` | join ヘルパー用リクエスト / 結果パスです |
| `RUST_LOG` | `info` (`warn` for send/devices) | ログフィルターです |

受信側ファイル：`~/.config/quickshare-direct/env`（`QSD_NAME`、`QSD_DIR` など）です。ヘルパーファイル：`/etc/default/quickshare-direct` です。

### 仕組み / 調査メモ

[docs/DEV_NOTES.md](docs/DEV_NOTES.md)（調査結果、スマートフォンログ、プロトコル詳細）と [docs/BLE_RECEIVER_DISCOVERY.md](docs/BLE_RECEIVER_DISCOVERY.md)（BLE 受信側パス）です。

リリース（このフォーク）：`v*.*.*` タグのプッシュで [`.github/workflows/release.yml`](.github/workflows/release.yml) が実行され、Tauri GUI バンドル（`.deb`、`.rpm`、`.AppImage`）と `directdrop-cli-linux-x86_64.tar.gz` デーモン tarball をビルドして GitHub Release に公開します — 上記の `get-directdrop.sh` 経由でインストールできます。上流デーモンパッケージは独自フローを維持します：CI（`.github/workflows/packages.yml`）は毎プッシュでパッケージをビルド・インストールテストします。`main` 上の `VERSION` を上げるとリリース `qsd-v<VERSION>` が公開されます。

リポジトリ構成：`core_lib`（ライブラリ + `quickshare-direct` バイナリ）と `packaging/` です。これらがすべてのパッケージ、NixOS モジュール、CI ビルドです。`app/main/` はこのフォークで復活したデスクトップ GUI（Tauri + Vue）です。`src-tauri/src/dto.rs` + `bridge.rs` と `src/types/qsd.ts` コントラクト経由で `core_lib` をインプロセスラップし、コアには触れません。その他の上流の残余（`snap/`、`BUILD.md`、`CHANGELOG.md`、release-please、`build` / `lint` ワークフロー）は引き続き上流の `master` を指しています。

開発（ヘッドレス）：`nix develop` の後、`core_lib` で `cargo run --bin quickshare-direct` です（環境：`QSD_DIR`、`QSD_NAME`、`QSD_PORT`、`QSD_BWU_PORT`、`QSD_BWU=lan|hotspot`、`RUST_LOG`）。開発（GUI）：上記「ソースから GUI を実行する」を参照してください。

---

<div align="center">

## ⭐ 応援のお願い

DirectDrop でルーターを触らずにスマートフォンへファイルを届けられたら、スターを付けてください — 大きな助けになります。

<a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=social" alt="star this repo" /></a>

<a href="https://star-history.com/#DeveloperMatt02/DirectDrop&Date"><img src="https://api.star-history.com/svg?repos=DeveloperMatt02/DirectDrop&type=Date" width="600" alt="star history chart" /></a>

バグを見つけましたか？[課題を開く](https://github.com/DeveloperMatt02/DirectDrop/issues/new) — プルリクエストも歓迎します。

</div>
