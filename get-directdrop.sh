#!/bin/sh
# get-directdrop.sh: install DirectDrop from GitHub Releases.
#
#   curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash
#
# Detects the distribution and CPU and installs the matching bundle from the
# latest DeveloperMatt02/DirectDrop release: .rpm via dnf/yum on Fedora/RHEL,
# .deb via apt-get on Debian/Ubuntu, .AppImage into ~/.local/bin everywhere
# else. Prefers the DirectDrop GUI build; falls back to the quickshare-direct
# daemon package (same project; shipped by the qsd-v* daemon releases until
# a v*.*.* tag publishes the GUI bundles). When a GUI bundle is installed,
# the headless daemon is disabled: the GUI embeds its own receiver and two
# receivers would advertise the same device twice and fight over the
# per-user Wi-Fi helpers.
#
# Edition: --gui / --cli / --auto (default), or DIRECTDROP_EDITION env.
# Without either and with a terminal attached, the edition is asked
# interactively (prompt reads /dev/tty, so `curl | bash` is safe).
set -eu

REPO=DeveloperMatt02/DirectDrop
APP=directdrop
say() { printf '\033[1m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33mwarning:\033[0m %s\n' "$*" >&2; }
die() { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

usage() {
  cat <<USAGE
Usage: $0 [--gui|--cli|--auto] [-h|--help]

Installs DirectDrop from the latest GitHub release.
  --gui    desktop GUI only (fails if the release has no GUI bundle yet)
  --cli    headless quickshare-direct daemon only
  --auto   GUI if available, else the daemon (default)
Env DIRECTDROP_EDITION=gui|cli|auto overrides the default; flags win over
env. With none given and a terminal attached, the edition is asked
(piped stdin is safe: the prompt uses /dev/tty).
USAGE
}

EDITION_FLAG=
for arg in "$@"; do
  case $arg in
    --gui) EDITION_FLAG=gui ;;
    --cli) EDITION_FLAG=cli ;;
    --auto) EDITION_FLAG=auto ;;
    -h|--help) usage; exit 0 ;;
    *) die "unknown option '$arg' (see --help)" ;;
  esac
done

[ "$(uname -s)" = Linux ] || die "Linux only"
case $(uname -m) in
  x86_64 | amd64) ARCH=x86_64 DEB_ARCH=amd64 ;;
  aarch64 | arm64) ARCH=aarch64 DEB_ARCH=arm64 ;;
  *) die "unsupported CPU architecture $(uname -m) (this installer ships x86_64 and aarch64 builds)" ;;
esac

EDITION=${EDITION_FLAG:-${DIRECTDROP_EDITION:-auto}}
case $EDITION in
  auto|gui|cli) ;;
  *) die "DIRECTDROP_EDITION must be gui, cli or auto (got '$EDITION')" ;;
esac
if [ "$EDITION" = auto ] && [ -c /dev/tty ]; then
  printf '\nInstall which edition?\n  1) Automatic: GUI if available, else daemon (default)\n  2) GUI only\n  3) CLI daemon only\nChoice [1]: ' > /dev/tty
  choice=
  IFS= read -r choice < /dev/tty 2>/dev/null || choice=
  case $choice in
    2|gui|GUI) EDITION=gui ;;
    3|cli|CLI) EDITION=cli ;;
  esac
fi

if [ "$(id -u)" -eq 0 ]; then
  SUDO=
else
  command -v sudo >/dev/null || die "needs root: install sudo or run as root"
  SUDO=sudo
fi

# shellcheck disable=SC1091
[ -r /etc/os-release ] && . /etc/os-release
ID=${ID:-unknown}
LIKE=" $ID ${ID_LIKE:-} "

fetch() { # url dest
  if command -v curl >/dev/null; then curl -fL --retry 3 -o "$2" "$1"
  elif command -v wget >/dev/null; then wget -O "$2" "$1"
  else die "needs curl or wget"; fi
}

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

say "looking up the latest $REPO release"
fetch "https://api.github.com/repos/$REPO/releases/latest" "$TMP/release.json" \
  || die "couldn't reach the GitHub API (offline or rate-limited?)"

# Pick a bundle URL from the release JSON ($TMP/release.json), honoring EDITION:
# gui = DirectDrop GUI builds only, cli = anything but GUI (daemon packages),
# auto = GUI first, daemon fallback. Always prints (possibly empty), never fails.
match_url() { # product-filter ext arch-ere ('DirectDrop' | '!' | '')
  # NOTE: match on the asset file name (after the last '/'), never on the
  # full URL: every URL contains '/DeveloperMatt02/DirectDrop/releases/...'.
  grep '"browser_download_url"' "$TMP/release.json" \
    | { case $1 in '') cat ;; '!') grep -v "/DirectDrop[^/]" ;; *) grep "/DirectDrop[^/]" ;; esac; } \
    | grep "\.$2\"" | grep -E "$3" | head -n 1 | cut -d '"' -f 4 || true
}

asset_url() { # ext arch-ere
  case $EDITION in
    gui) match_url DirectDrop "$1" "$2"; return 0 ;;
    cli) match_url '!' "$1" "$2"; return 0 ;;
  esac
  url=$(match_url DirectDrop "$1" "$2")
  if [ -n "$url" ]; then printf '%s' "$url"; return 0; fi
  match_url '' "$1" "$2"
  return 0
}

FORMAT=
case $LIKE in
  *" fedora "* | *" rhel "* | *" centos "*)
    if command -v dnf >/dev/null || command -v yum >/dev/null; then FORMAT=rpm; fi ;;
  *" debian "* | *" ubuntu "*)
    if command -v apt-get >/dev/null; then FORMAT=deb; fi ;;
esac
[ -n "$FORMAT" ] || FORMAT=appimage

case $FORMAT in
  rpm)
    URL=$(asset_url rpm "$ARCH")
    [ -n "$URL" ] || die "no $EDITION .rpm for $ARCH in the latest $REPO release"
    FILE=$TMP/$(basename "$URL")
    say "downloading $(basename "$URL")"
    fetch "$URL" "$FILE"
    say "installing with dnf"
    if command -v dnf >/dev/null; then $SUDO dnf install -y "$FILE"
    else $SUDO yum install -y "$FILE"; fi
    ;;
  deb)
    URL=$(asset_url deb "$DEB_ARCH")
    [ -n "$URL" ] || die "no $EDITION .deb for $DEB_ARCH in the latest $REPO release"
    FILE=$TMP/$(basename "$URL")
    say "downloading $(basename "$URL")"
    fetch "$URL" "$FILE"
    chmod 644 "$FILE" && chmod 755 "$TMP" # apt's sandbox user must read it
    say "installing with apt"
    $SUDO apt-get update -q || warn "apt-get update failed; trying the install anyway"
    $SUDO apt-get install -y "$FILE"
    ;;
  appimage)
    if [ "$EDITION" = cli ]; then
      die "no CLI AppImage exists; download quickshare-direct-$ARCH.tar.gz from https://github.com/$REPO/releases/latest"
    fi
    URL=$(asset_url AppImage "$DEB_ARCH|$ARCH")
    [ -n "$URL" ] || die "no $EDITION .AppImage for $ARCH in the latest $REPO release"
    DEST="$HOME/.local/bin/$APP"
    mkdir -p "$HOME/.local/bin"
    say "installing $(basename "$URL") to $DEST"
    fetch "$URL" "$DEST"
    chmod +x "$DEST"
    case ":$PATH:" in
      *":$HOME/.local/bin:"*) ;;
      *) warn "$HOME/.local/bin is not in PATH; add it or run $DEST directly" ;;
    esac
    ;;
esac

case $(basename "$URL") in
  DirectDrop*) WHAT=gui ;;
  *) WHAT=daemon ;;
esac

# The GUI embeds its own receiver: keep a single advertiser on this machine.
# Best effort, never fatal (no systemd / daemon not installed / root without SUDO_USER).
disable_daemon() {
  command -v systemctl >/dev/null 2>&1 || return 0
  if [ "$(id -u)" -eq 0 ]; then
    [ -n "${SUDO_USER:-}" ] || return 0
    sudo -u "$SUDO_USER" XDG_RUNTIME_DIR="/run/user/$(id -u "$SUDO_USER")" \
      systemctl --user disable --now quickshare-direct.service >/dev/null 2>&1 || return 0
  else
    systemctl --user disable --now quickshare-direct.service >/dev/null 2>&1 || return 0
  fi
  say "headless daemon disabled (the GUI is the receiver now)"
}

if [ "$WHAT" = gui ]; then
  disable_daemon
  say "done. Launch '$APP' (or DirectDrop from the app menu)."
else
  say "done (this release ships the daemon; GUI bundles come with a v*.*.* tag)."
  say "Receive: pick '$(uname -n)' in Quick Share on the phone. Send: 'quickshare-direct send FILE'."
fi
