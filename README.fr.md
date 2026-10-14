<div align="center">
  <a href="README.md"><img src="https://img.shields.io/badge/English-EN-555?style=flat-square" alt="English" /></a>
  <a href="README.it.md"><img src="https://img.shields.io/badge/Italiano-IT-555?style=flat-square" alt="Italiano" /></a>
  <a href="README.es.md"><img src="https://img.shields.io/badge/Español-ES-555?style=flat-square" alt="Español" /></a>
  <img src="https://img.shields.io/badge/Français-FR-blue?style=flat-square" alt="Français" />
  <a href="README.de.md"><img src="https://img.shields.io/badge/Deutsch-DE-555?style=flat-square" alt="Deutsch" /></a>
  <a href="README.pt.md"><img src="https://img.shields.io/badge/Português-PT-555?style=flat-square" alt="Português" /></a>
  <a href="README.ru.md"><img src="https://img.shields.io/badge/Русский-RU-555?style=flat-square" alt="Русский" /></a>
  <a href="README.zh.md"><img src="https://img.shields.io/badge/中文-ZH-555?style=flat-square" alt="中文" /></a>
  <a href="README.ja.md"><img src="https://img.shields.io/badge/日本語-JA-555?style=flat-square" alt="日本語" /></a>
</div>

<div align="center">
  <img src="app/main/src-tauri/icons/128x128.png" width="128" alt="icône DirectDrop" />
  <h1>DirectDrop</h1>
  <p><strong>Quick Share (Nearby Share) pour Linux qui n'exige pas que les deux appareils soient sur le même Wi-Fi — désormais avec une GUI de bureau.</strong></p>

  <a href="https://github.com/DeveloperMatt02/DirectDrop/releases/latest"><img src="https://img.shields.io/github/v/release/DeveloperMatt02/DirectDrop?style=flat-square" alt="dernière version" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=flat-square" alt="étoiles" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/releases/latest"><img src="https://img.shields.io/github/downloads/DeveloperMatt02/DirectDrop/total?style=flat-square" alt="téléchargements" /></a>
  <img src="https://img.shields.io/badge/License-GPLv3-blue?style=flat-square" alt="licence GPL-3.0" />
  <img src="https://img.shields.io/badge/platform-Linux-blue?style=flat-square" alt="plateforme Linux" />
  <img src="https://img.shields.io/badge/Tauri-2.2-24C8DB?style=flat-square&logo=tauri" alt="Tauri 2.2" />
  <img src="https://img.shields.io/badge/Vue-3.5-4FC08D?style=flat-square&logo=vue.js" alt="Vue 3.5" />
  <a href="https://github.com/DeveloperMatt02/DirectDrop/actions/workflows/release.yml"><img src="https://github.com/DeveloperMatt02/DirectDrop/actions/workflows/release.yml/badge.svg" alt="CI de publication" /></a>

  <br />
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=social" alt="mettre une étoile à ce dépôt" /></a>

  <br /><br />
  <a href="https://github.com/DeveloperMatt02/DirectDrop/releases/latest"><img src="https://img.shields.io/badge/⬇_Download_Latest_Release-38bdf8?style=for-the-badge" alt="télécharger la dernière version" /></a>
  <a href="#installation-rapide-terminal"><img src="https://img.shields.io/badge/🐧_Quick_Install-22c55e?style=for-the-badge" alt="installation rapide" /></a>
  <a href="#captures-décran"><img src="https://img.shields.io/badge/🖥️_Screenshots-a855f7?style=for-the-badge" alt="captures d'écran" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/issues/new"><img src="https://img.shields.io/badge/🐛_Report_Bug-f59e0b?style=for-the-badge" alt="signaler un bug" /></a>
  <a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/badge/⭐_Star-facc15?style=for-the-badge&logo=github" alt="étoile" /></a>
</div>

> Un fork du `quickshare-direct` en amont. La pile émettrice / réceptrice sans interface est conservée telle quelle (annonce BLE / premier contact GATT, bascule vers Wi-Fi Direct GO/client, repli LAN, assistants polkit, démon CLI sans interface). Ce qu'apporte ce fork, c'est une GUI de bureau Tauri repensée dans `app/main/` avec une parité comportementale complète avec la CLI.

## Installation rapide (Terminal)

Commande unique universelle — détecte la distribution et installe le paquet correspondant depuis la
dernière version `DeveloperMatt02/DirectDrop` (`.rpm` sur Fedora/RHEL, `.deb` sur
Debian/Ubuntu, `.AppImage` dans `~/.local/bin` partout ailleurs) :

```bash
curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash
```

Commandes directes par distribution (même source, sans script auxiliaire) :

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

Ou choisissez l'artefact à la main depuis la [dernière version](https://github.com/DeveloperMatt02/DirectDrop/releases/latest).

## 🖥️ Captures d'écran

Fenêtre principale (console en mode exploitation : en-tête de statut, flux en direct portable ⇄ téléphone, panneau d'envoi, tableau des appareils, file de transferts) :

## GUI + démon sans interface : choisissez-en un seul

Les paquets GUI (`.deb` / `.rpm` / `.AppImage`) embarquent leur propre récepteur ; le paquet du démon `quickshare-direct` est le même récepteur sans interface. Exécuter les deux en même temps annonce le portable deux fois sous le même nom d'appareil et les fait se disputer les assistants Wi-Fi par utilisateur (`quickshare-ap@USER`, `quickshare-join@USER`), donc `get-directdrop.sh` désactive le démon (`systemctl --user disable --now quickshare-direct`) chaque fois qu'il installe un paquet GUI. La GUI `.deb`/`.rpm` ajoute aussi DirectDrop au menu des applications (cherchez "DirectDrop") ; l'`.AppImage` ne le fait pas tant qu'elle n'est pas intégrée (Gear Lever, appimaged, ou un fichier `.desktop` manuel). Pour revenir au mode sans interface : quittez la GUI et lancez `systemctl --user enable --now quickshare-direct`.

![Fenêtre principale de DirectDrop](docs/screenshots/gui-main.svg)

Transfert entrant — carte de vérification du PIN UKEY2 avec compte à rebours de 60 s et refus automatique :

![Carte de consentement de DirectDrop](docs/screenshots/gui-consent.svg)

> [!NOTE]
> Il s'agit de schémas provisoires dans le style blueprint de l'application. Remplacez `docs/screenshots/gui-main.svg` et `docs/screenshots/gui-consent.svg` par de vraies captures (`pnpm tauri dev`, capturez la fenêtre, enregistrez en `.png` à côté et mettez à jour les liens ci-dessus).

Le guide de démarrage rapide en amont reste valable :

1. **Découverte + premier contact via Bluetooth LE** (fonctionne avec le téléphone hors Wi-Fi).
2. **Poignée de main chiffrée** (UKEY2, PIN affiché des deux côtés) sur ce lien Bluetooth.
3. **Bascule vers la vitesse Wi-Fi** : le portable crée un **groupe Wi-Fi Direct** à côté de sa connexion Wi-Fi normale (qui continue de fonctionner) et transmet les identifiants au téléphone ; ou, si le téléphone est sur le même Wi-Fi, le transfert passe par le réseau local. Un simple point d'accès est la solution de repli.
4. **Notifications de bureau** : Accepter/Refuser (avec le PIN), puis « Ouvrir le dossier ».

| | Statut |
|---|---|
| Réception depuis Android (testé en amont : Pixel 10, Android 16) | fonctionne |
| Téléphone hors Wi-Fi → Wi-Fi Direct (75 MB, ~5 MB/s) | fonctionne |
| Téléphone sur le même Wi-Fi → LAN | fonctionne |
| Premier contact en Bluetooth Classic | expérimental (`QSD_BT_CLASSIC=1`) |
| Envoi vers Android → Wi-Fi Direct du téléphone (300 MB, ~36 MB/s) | fonctionne (`quickshare-direct send`, gestionnaire de fichiers, panneau d'envoi de la GUI) |
| Paquets : Debian/Ubuntu, Fedora, Arch, générique (x86_64, aarch64) | installation en une ligne, voir ci-dessous |

---

## Ce qu'apporte ce fork : GUI de bureau réactivée et repensée (`app/main/`)

Le dépôt en amont conserve l'ancienne application Tauri de rQuickShare sous `app/` mais ne la compile, ne la distribue ni ne la teste (son chemin d'envoi est limité au LAN/mDNS, sans le chemin Bluetooth / Wi-Fi Direct). Ce fork réactive cette idée sous la forme d'une nouvelle GUI dédiée dans `app/main/` qui pilote le **même** moteur `core_lib` (`rqs_lib`) que la CLI — sans implémentation parallèle du protocole.

### Pile technique et apparence

- **Tauri 2.2, Vue 3.5, Pinia 2.x, Tailwind CSS 3.4.** Le point d'entrée du frontend est `app/main/src/App.vue` ; l'état vit dans les stores Pinia (`src/stores/session.ts`, `devices.ts`, `transfers.ts`).
- **Esthétique blueprint technique :** console sombre en mode exploitation, métriques tabulaires en police monospace, schéma de flux réseau en direct entre le portable et le téléphone (`FlowDiagram.vue` / `FlowHero.vue`), filets de 1 px, sans fioritures marketing. Disposition : `StatusHeader` → `FlowDiagram` → `SendPane` + `DeviceBoard` → `TransferQueue`, avec un fin pied de page détaillant le récepteur (chemin de téléchargement, port).

### Pont Rust en processus non invasif

La GUI exécute `rqs_lib` en processus. La frontière se limite à exactement deux nouveaux fichiers plus un contrat versionné — **zéro modification du moteur de protocole central** :

- `app/main/src-tauri/src/dto.rs` — l'unique format sur le fil. Les types du domaine central dans `core_lib` ne dérivent volontairement pas `Serialize`/`Deserialize` ; chaque charge utile Tauri `invoke`/`emit` passe par ces DTO serde avec des correspondances `From` explicites. `DTO_VERSION = 1`, vérifié par le test Rust `dto_version_matches_ts` face au miroir TypeScript.
- `app/main/src-tauri/src/bridge.rs` — s'abonne aux canaux `broadcast`/`watch` du cœur (`message_sender`, `dch_sender`, `visibility_sender`, `ble_receiver`) et réémet des événements Tauri limités/dédupliqués. Les événements de progression sont limités (émission lors d'un changement d'état, ou d'un delta ≥256 KiB, ou de ≥500 ms ; les états terminaux sont toujours émis). Les débordements `Lagged` du broadcast sont ignorés avec un journal de débogage, jamais un plantage — même politique que la CLI.
- `app/main/src/types/qsd.ts` — miroir TypeScript strict de `dto.rs` (noms d'événements, `VisibilityDto`, `TransferStateDto` avec les 18 états du cœur, `PayloadKindDto`, `TransferDto`, `EndpointDto`, `SendPayloadDto`, `ConsentDto`, `SnapshotDto`, codes d'erreur). Inclut les utilitaires `humanSize` / `describeTransfer` (portage du `describe()` de la CLI), `baseName` et `validateSendFiles`, plus les ensembles `TERMINAL_STATES` / `ACTIVE_PROGRESS_STATES` / `HANDSHAKE_STATES`.

Commandes Tauri exposées (`src-tauri/src/cmds/`, câblées dans `main.rs`) : `qsd_snapshot`, `qsd_set_visibility`, `qsd_set_download_path`, `qsd_set_device_name`, `qsd_start_discovery`, `qsd_stop_discovery`, `qsd_consent`, `qsd_send_lan`, `qsd_get_hostname` (plus les compatibilités héritées `get_hostname` / `change_download_path`). Événements backend → frontend : `qsd://transfer`, `qsd://endpoints`, `qsd://visibility`, `qsd://ble-presence`.

### Parité comportementale complète avec la CLI

- **Carte instantanée de vérification du PIN UKEY2** (`ConsentCard.vue`) : nom de l'expéditeur, type d'appareil, PIN, résumé de la charge utile et liste des fichiers apparaissent dès que le cœur atteint `WaitingForUserConsent`. Un anneau de **compte à rebours de 60 secondes** (`QSD_CONSENT_TIMEOUT_MS`, respectant la surcharge d'env `QSD_CONSENT_TIMEOUT` lue dans `dto.rs`) expire la carte ; l'expiration déclenche le même **refus automatique** (`ConsentDecline` sur le canal) que la CLI. Accepter/Refuser/Annuler correspondent 1:1 aux `TransferAction`s du cœur via `ConsentDto::into_channel_message`.
- **Sélecteur de visibilité à 3 états** (`StatusHeader.vue` → `qsd_set_visibility`) : `Visible`, `Temporarily` (60 s, avec point pulsé orange et affichage du temps restant), `Hidden` (`Invisible`). Le minuteur Temporarily→Invisible vit dans le `mdns.rs` du cœur ; la GUI se contente de refléter l'événement `qsd://visibility` et le persiste via le store Tauri. Aucune réimplémentation du minuteur côté GUI.
- **Bannière d'alerte de présence BLE passive :** les détections de l'écouteur BLE du cœur arrivent sous forme d'événements `qsd://ble-presence` limités (120 s). Lorsqu'elle est masquée (`Invisible`), le pont déclenche en plus une notification système proposant une fenêtre de visibilité temporaire ; le frontend affiche un toast d'info (« Un téléphone à proximité partage — vérifiez la visibilité »). Respecte la sémantique centrale `QSD_BLE_SCAN` / `QSD_NO_BLE_SCAN`.
- **Gestion des transferts de texte/URL en ligne sans écriture forcée sur disque** (`TransferCard.vue`) : les charges utiles `Text`, `Url` et `WiFi` s'affichent en ligne avec aperçu et copie dans le presse-papiers (plugin Tauri clipboard-manager) ; aucun fichier n'est écrit sur le disque pour les afficher. Les charges utiles de fichiers affichent le nombre de fichiers RX, les noms et la progression précise en octets (`ack_bytes` / `total_bytes`).
- **Envoi de fichiers sortant en LAN/mDNS avec validation des fichiers réguliers** (`SendPane.vue` + `qsd_send_lan`) : préparez les fichiers via le dialogue ou le glisser-déposer, choisissez un point de terminaison mDNS découvert (`EndpointDto`, `addr = ip:port`), et envoyez via le canal `sender_file` du cœur sous forme `SendInfo { ob: Files }`. La validation correspond à la CLI : sélections vides et répertoires rejetés avec des erreurs `VALIDATION` (`{f}: not a regular file (folders aren't supported)`) ; le frontend échoue rapidement sur les cas évidents avant d'appeler le backend.

### Lancer la GUI depuis les sources

```bash
cd app/main
pnpm install
pnpm tauri dev     # live GUI against local core_lib (path dep: ../../core_lib)
pnpm ts-check       # vue-tsc --noEmit contract check
pnpm test           # vitest suite (tests/)
pnpm build          # production Tauri bundle
```

Côté Rust : `cd app/main/src-tauri && cargo check` / `cargo test` (inclut les tests du format sur le fil DTO). Les prérequis système sont les mêmes que pour le démon (BlueZ, NetworkManager, en-têtes libdbus, protoc, cargo ≥ 1.85) plus Node/pnpm et les dépendances Linux de Tauri 2.x.

---

## Architecture et généalogie / Crédits

Vue en couches de ce fork :

```
Android phone ──BLE adv / GATT──▶ core_lib (rqs_lib) ◀──Tauri bridge──▶ Vue GUI
                                         │  (unchanged engine)
                                         ▼
                                Wi-Fi Direct GO / client,
                                LAN, hotspot fallback,
                                polkit Wi-Fi helpers
```

- **Auteur(s) du `quickshare-direct` en amont :** le transport hors réseau dont dépend ce fork — surveillance des annonces BLE et serveur GATT pour la découverte/premier contact, bascule vers groupe Wi-Fi Direct propriétaire / client à côté de la connexion Wi-Fi normale (avec gestion du repli monocanal), les assistants root à la demande `quickshare-ap@USER` / `quickshare-join@USER` avec leur règle polkit, et le démon CLI sans interface `quickshare-direct` avec notifications de bureau. Tout le mérite de l'ingénierie protocole, Bluetooth et Wi-Fi leur revient ; ce fork n'y modifie rien.
- **`rQuickShare` par Martichou (et des contributeurs comme nozwock et martinalderson) :** l'implémentation Rust/Tauri fondatrice du protocole Nearby Share / Quick Share sous Linux sur laquelle l'amont et ce fork s'appuient — y compris le cœur `rqs_lib`, la structure originale de l'application Tauri et les travaux de découverte Bluetooth repris dans `core_lib`.
- **Google Nearby (Apache-2.0) :** la spécification du protocole et les définitions protobuf qui définissent les trames UKEY2, de connexion, d'introduction et de charge utile. La licence de ce dépôt reste GPL-3.0, comme rQuickShare.

---

### Installation (démon + paquets CLI)

Debian 12+ / Ubuntu 22.04+ (et Mint, Pop!_OS, ...), Fedora, Arch (et Manjaro, EndeavourOS, ...),
autres via une compilation générique ; x86_64 et aarch64 :

```
curl -fsSL https://raw.githubusercontent.com/papshmeare/quickshare-direct/main/install.sh | sh
```

Il installe la dernière [version](https://github.com/papshmeare/quickshare-direct/releases) avec
votre gestionnaire de paquets (`.deb`, `.rpm`, `.pkg.tar.zst` ; les dépendances viennent de votre distribution),
ouvre les ports Quick Share si firewalld ou ufw est actif, et démarre le récepteur. Relancez la même
ligne pour mettre à jour ; `curl -fsSL .../install.sh | sh -s -- --uninstall` le supprime. Vous préférez
voir d'abord ce qui se passe ? Téléchargez `install.sh` et lisez-le, ou installez le paquet depuis la
page de version à la main (`sudo apt install ./quickshare-direct_amd64.deb`,
`sudo dnf install ./quickshare-direct.x86_64.rpm`,
`sudo pacman -U quickshare-direct-x86_64.pkg.tar.zst`).

Ce qui est installé :
- `quickshare-direct` : le récepteur (un service utilisateur dans chaque session graphique, avec
  notifications) et les commandes `send` / `devices` ;
- deux petits assistants root, démarrés à la demande via systemd (`quickshare-ap@USER` crée le
  lien Wi-Fi direct lors de la réception, `quickshare-join@USER` rejoint celui du téléphone lors de l'envoi), et une
  règle polkit (`packaging/linux/polkit/50-quickshare-direct.rules`) qui permet à un utilisateur local actif
  de démarrer/arrêter uniquement ses propres instances (`start`/`stop`/`restart` sur
  `quickshare-ap@USER.service` / `quickshare-join@USER.service`) ;
- un fichier drop-in NetworkManager qui laisse tranquilles les interfaces de liaison temporaires, une unité qui maintient
  le Bluetooth connectable, des définitions de services firewalld/ufw, et des entrées du gestionnaire de fichiers.

Prérequis : systemd, BlueZ, NetworkManager avec wpa_supplicant (par défaut sur ces distributions ;
pas iwd), et une carte Wi-Fi capable d'exécuter une seconde interface à côté de la connexion normale (la plupart
des cartes Intel/MediaTek/Qualcomm). Les notifications nécessitent un démon qui affiche des boutons d'action (GNOME,
KDE, swaync, mako, ...). L'ancien polkit d'Ubuntu 22.04 ne permet pas aux utilisateurs de démarrer les assistants, donc là
seuls les transferts Bluetooth et sur le même réseau fonctionnent. Réglez votre domaine réglementaire Wi-Fi
(`sudo iw reg set XX`) : avec le domaine mondial par défaut, Linux ne démarrera pas de propriétaire de groupe en 5 GHz.
Paramètres : `~/.config/quickshare-direct/env` (`QSD_NAME=...`, `QSD_DIR=...`) pour le récepteur,
`/etc/default/quickshare-direct` pour les assistants. Beaucoup de puces Wi-Fi ne peuvent pas héberger le lien direct sur un
second canal à côté de votre connexion normale, ce qui compte quand votre routeur utilise un canal
qu'un portable ne peut pas héberger (canaux radar/DFS comme 100) : le portable quitte alors votre réseau Wi-Fi
pendant les quelques secondes du transfert puis se reconnecte après (`QS_SINGLE_CHANNEL=auto` ; `always`
pour la pleine vitesse aussi à l'envoi, `never` pour garder la connexion et revenir au Bluetooth).

Depuis les sources : `make && sudo make install` (nécessite cargo ≥ 1.85, protoc, en-têtes libdbus), puis
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

Cela configure le récepteur (service utilisateur dans la session graphique), l'assistant root qui crée
le lien Wi-Fi direct à la demande (`quickshare-ap.service`, démarrable par cet utilisateur via polkit),
l'assistant qui rejoint le groupe Wi-Fi Direct d'un téléphone lors de l'envoi (`quickshare-join.service`),
les ports du pare-feu, le Bluetooth « connectable », et tient NetworkManager à l'écart des interfaces de liaison.
Réglez votre domaine réglementaire Wi-Fi (`iw reg set XX`) : avec le domaine mondial par défaut, Linux ne
démarrera pas un point d'accès / propriétaire de groupe en 5 GHz.

### Envoi (CLI)

Sur le téléphone, ouvrez Quick Share et touchez **Receive** (ou rendez-le visible par tous), puis :

```
quickshare-direct send photo.jpg notes.pdf      # to the one phone in receive mode nearby
quickshare-direct send --to Pixel video.mp4     # pick by name when several are around
quickshare-direct devices                       # list phones in receive mode
```

Il trouve le téléphone via Bluetooth, affiche le PIN, et une fois que vous acceptez sur le téléphone, les fichiers partent
via le groupe Wi-Fi Direct du téléphone (rejoint sur une seconde interface, votre Wi-Fi reste connecté).
Dans le gestionnaire de fichiers : **Envoyer vers → Téléphone (Quick Share)** de Thunar (tout fichier), ou Ouvrir avec →
« Envoyer avec Quick Share » (types de fichiers courants) ; il rend compte via les notifications. Le
`SendPane` de la GUI effectue le même envoi LAN/mDNS via `qsd_send_lan` avec des sémantiques de PIN et de
fichiers réguliers identiques.

### Variables d'environnement (récepteur, expéditeur, assistants)

Le comportement du cœur est configuré via l'environnement (voir `core_lib/src/bin/quickshare-direct.rs`,
`core_lib/src/hdl/*.rs`) :

| Variable | Default | Effect |
|---|---|---|
| `QSD_DIR` | XDG Downloads, else `~/Downloads` | dossier de téléchargement |
| `QSD_NAME` | hostname | nom d'appareil affiché sur le téléphone |
| `QSD_PORT` | random | port TCP pour les transferts entrants |
| `QSD_BWU_PORT` | random | port TCP pour l'écouteur de bascule Wi-Fi |
| `QSD_CONSENT_TIMEOUT` | `60` (s) | secondes pour répondre Accepter/Refuser avant le refus automatique ; pilote aussi le compte à rebours de la GUI |
| `QSD_BWU` | `auto` | préférence de bascule entrante : `lan` / `hotspot` en force une |
| `QSD_BLE_SCAN` | passive BlueZ advertisement monitor | `discovery` = découverte LE permanente (peut empêcher la reconnexion des souris/claviers LE appariés) ; `off` désactive l'écouteur |
| `QSD_NO_BLE_SCAN` | unset | lorsqu'elle est définie, désactive entièrement la détection BLE de l'expéditeur |
| `QSD_BT_CLASSIC` | unset | lorsqu'elle est définie (`=1`), active le premier contact expérimental en Bluetooth Classic |
| `QSD_BLE_L2CAP` / `QSD_L2CAP_CHUNK` | on / default | réglage BLE L2CAP du chemin d'envoi (`=0` force la socket weave) |
| `QSD_WEAVE_ATT` / `QSD_WEAVE_PACKET` | on / MTU default | réglage du transport GATT/weave pour les expériences |
| `QSD_SEND_MEDIUMS` | wifi_lan-first default | préférence de support sortant séparée par virgules (`wifi_lan`, ...) |
| `QS_SINGLE_CHANNEL` | `auto` | `always` = pleine vitesse à l'envoi (peut couper la station) ; `never` = garde la connexion, repli vers Bluetooth |
| `QSD_AP_IFACE` / `QSD_STA_IFACE` | `ap0` / auto | noms d'interfaces de liaison pour l'assistant AP |
| `QSD_AP_UNIT` / `QSD_JOIN_UNIT` | `quickshare-ap` / `quickshare-join` | noms d'unités systemd pour les assistants root |
| `QSD_SYSTEMCTL` | `systemctl` | surcharge de la commande de lancement des assistants |
| `QSD_AP_CREDENTIALS` | `/run/quickshare/credentials` | chemin de dépôt des identifiants pour l'assistant AP |
| `QSD_JOIN_REQUEST` / `QSD_JOIN_RESULT` | `/run/quickshare-join/...` | chemins de requête/résultat pour l'assistant de jonction |
| `RUST_LOG` | `info` (`warn` for send/devices) | filtre de journalisation |

Fichier du récepteur : `~/.config/quickshare-direct/env` (`QSD_NAME`, `QSD_DIR`, ...). Fichier des assistants :
`/etc/default/quickshare-direct`.

### Fonctionnement / notes de recherche

[docs/DEV_NOTES.md](docs/DEV_NOTES.md) (constats, journaux du téléphone, détails du protocole) et
[docs/BLE_RECEIVER_DISCOVERY.md](docs/BLE_RECEIVER_DISCOVERY.md) (le chemin récepteur BLE).

Versions (ce fork) : pousser un tag `v*.*.*` exécute [`.github/workflows/release.yml`](.github/workflows/release.yml), qui compile les paquets GUI Tauri (`.deb`, `.rpm`, `.AppImage`) et l'archive du démon `directdrop-cli-linux-x86_64.tar.gz` et les publie dans la version GitHub — installable via `get-directdrop.sh` ci-dessus. Les paquets du démon en amont gardent leur propre flux : la CI (`.github/workflows/packages.yml`) compile et teste les paquets à l'installation à chaque push ; incrémenter `VERSION` sur `main` publie la version `qsd-v<VERSION>`.

Disposition du dépôt : `core_lib` (bibliothèque + le binaire `quickshare-direct`) et `packaging/` ; c'est
tout ce que les paquets, le module NixOS et la CI compilent. `app/main/` est la GUI de bureau
réactivée de ce fork (Tauri + Vue) : elle enveloppe `core_lib` en processus via `src-tauri/src/dto.rs` + `bridge.rs` et le
contrat `src/types/qsd.ts`, sans toucher au cœur. Les autres restes en amont (`snap/`, `BUILD.md`,
`CHANGELOG.md`, release-please, les flux `build`/`lint`) se réfèrent toujours au `master` en amont.

Développement (sans interface) : `nix develop`, puis dans `core_lib` : `cargo run --bin quickshare-direct`
(env : `QSD_DIR`, `QSD_NAME`, `QSD_PORT`, `QSD_BWU_PORT`, `QSD_BWU=lan|hotspot`, `RUST_LOG`).
Développement (GUI) : voir « Lancer la GUI depuis les sources » ci-dessus.

---

<div align="center">

## ⭐ Montrez votre soutien

Si DirectDrop achemine vos fichiers vers votre téléphone sans toucher au routeur, mettez-lui une étoile — cela aide beaucoup.

<a href="https://github.com/DeveloperMatt02/DirectDrop/stargazers"><img src="https://img.shields.io/github/stars/DeveloperMatt02/DirectDrop?style=social" alt="mettre une étoile à ce dépôt" /></a>

<a href="https://star-history.com/#DeveloperMatt02/DirectDrop&Date"><img src="https://api.star-history.com/svg?repos=DeveloperMatt02/DirectDrop&type=Date" width="600" alt="graphique d'historique des étoiles" /></a>

Un bug ? [Ouvrez un ticket](https://github.com/DeveloperMatt02/DirectDrop/issues/new) — les pull requests sont les bienvenues.

</div>
