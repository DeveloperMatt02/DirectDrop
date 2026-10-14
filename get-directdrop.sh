#!/bin/sh
# get-directdrop.sh: install DirectDrop (desktop GUI) from GitHub Releases.
#
#   curl -fsSL https://raw.githubusercontent.com/DeveloperMatt02/DirectDrop/main/get-directdrop.sh | bash
#
# Detects the distribution and installs the matching bundle from the latest
# DeveloperMatt02/DirectDrop release: .rpm via dnf/yum on Fedora/RHEL,
# .deb via apt-get on Debian/Ubuntu, .AppImage into ~/.local/bin everywhere else.
set -eu

REPO=DeveloperMatt02/DirectDrop
APP=directdrop
say() { printf '\033[1m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33mwarning:\033[0m %s\n' "$*" >&2; }
die() { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

[ "$(uname -s)" = Linux ] || die "Linux only"
case $(uname -m) in
  x86_64 | amd64) ;;
  *) die "unsupported CPU architecture $(uname -m): this installer ships x86_64 builds" ;;
esac

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

asset_url() { # extension
  grep '"browser_download_url"' "$TMP/release.json" | grep "\.$1\"" | head -n 1 | cut -d '"' -f 4 || true
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
    URL=$(asset_url rpm)
    [ -n "$URL" ] || die "no .rpm in the latest $REPO release"
    FILE=$TMP/$(basename "$URL")
    say "downloading $(basename "$URL")"
    fetch "$URL" "$FILE"
    say "installing with dnf"
    if command -v dnf >/dev/null; then $SUDO dnf install -y "$FILE"
    else $SUDO yum install -y "$FILE"; fi
    ;;
  deb)
    URL=$(asset_url deb)
    [ -n "$URL" ] || die "no .deb in the latest $REPO release"
    FILE=$TMP/$(basename "$URL")
    say "downloading $(basename "$URL")"
    fetch "$URL" "$FILE"
    chmod 644 "$FILE" && chmod 755 "$TMP" # apt's sandbox user must read it
    say "installing with apt"
    $SUDO apt-get update -q || warn "apt-get update failed; trying the install anyway"
    $SUDO apt-get install -y "$FILE"
    ;;
  appimage)
    URL=$(asset_url AppImage)
    [ -n "$URL" ] || die "no .AppImage in the latest $REPO release"
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

say "done. Launch '$APP' (or DirectDrop from the app menu)."
