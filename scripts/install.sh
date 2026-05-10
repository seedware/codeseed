#!/bin/sh
set -eu

CODESEED_REPO="${CODESEED_REPO:-seedware/codeseed}"
CODESEED_VERSION="${CODESEED_VERSION:-latest}"
CODESEED_HOME="${CODESEED_HOME:-"$HOME/.codeseed"}"
CODESEED_BIN_DIR="${CODESEED_BIN_DIR:-"$CODESEED_HOME/bin"}"
CODESEED_DOWNLOAD_BASE="${CODESEED_DOWNLOAD_BASE:-https://github.com/$CODESEED_REPO/releases}"
CODESEED_SOURCE_URL="${CODESEED_SOURCE_URL:-https://github.com/$CODESEED_REPO.git}"
CODESEED_INSTALL_MODE="${CODESEED_INSTALL_MODE:-auto}"

usage() {
    cat <<'EOF'
Install Codeseed.

Usage:
  install.sh [OPTIONS]

Options:
  --version <VERSION>      Install a specific release version. Defaults to latest.
  --home <DIR>             Codeseed home directory. Defaults to ~/.codeseed.
  --bin-dir <DIR>          Directory for the codeseed executable. Defaults to ~/.codeseed/bin.
  --repo <OWNER/REPO>      GitHub repository for release downloads. Defaults to seedware/codeseed.
  --local                  Force local cargo build from the current source checkout.
  --prebuilt               Force prebuilt binary download.
  -h, --help               Show this help.

Environment:
  CODESEED_HOME            Global Codeseed home. Defaults to ~/.codeseed.
  CODESEED_BIN_DIR         Executable directory. Defaults to $CODESEED_HOME/bin.
  CODESEED_VERSION         Release version. Defaults to latest.
  CODESEED_DOWNLOAD_BASE   Release base URL.
  CODESEED_SOURCE_URL      Git URL used for source-build fallback.
  CODESEED_INSTALL_MODE    auto, local, or prebuilt.
EOF
}

info() {
    printf '%s\n' "codeseed-install: $*"
}

fail() {
    printf '%s\n' "codeseed-install: error: $*" >&2
    exit 1
}

has_cmd() {
    command -v "$1" >/dev/null 2>&1
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --version)
            [ "$#" -ge 2 ] || fail "--version requires a value"
            CODESEED_VERSION="$2"
            shift 2
            ;;
        --home)
            [ "$#" -ge 2 ] || fail "--home requires a value"
            CODESEED_HOME="$2"
            CODESEED_BIN_DIR="$CODESEED_HOME/bin"
            shift 2
            ;;
        --bin-dir)
            [ "$#" -ge 2 ] || fail "--bin-dir requires a value"
            CODESEED_BIN_DIR="$2"
            shift 2
            ;;
        --repo)
            [ "$#" -ge 2 ] || fail "--repo requires a value"
            CODESEED_REPO="$2"
            CODESEED_DOWNLOAD_BASE="https://github.com/$CODESEED_REPO/releases"
            CODESEED_SOURCE_URL="https://github.com/$CODESEED_REPO.git"
            shift 2
            ;;
        --local)
            CODESEED_INSTALL_MODE="local"
            shift
            ;;
        --prebuilt)
            CODESEED_INSTALL_MODE="prebuilt"
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            fail "unknown option: $1"
            ;;
    esac
done

detect_target() {
    os="$(uname -s 2>/dev/null || true)"
    arch="$(uname -m 2>/dev/null || true)"

    case "$arch" in
        x86_64|amd64) arch="x86_64" ;;
        arm64|aarch64) arch="aarch64" ;;
        *) fail "unsupported CPU architecture: $arch" ;;
    esac

    case "$os" in
        Darwin) printf '%s-apple-darwin\n' "$arch" ;;
        Linux) printf '%s-unknown-linux-gnu\n' "$arch" ;;
        *) fail "unsupported operating system: $os" ;;
    esac
}

is_codeseed_checkout() {
    [ -f Cargo.toml ] && grep 'name = "codeseed"' Cargo.toml >/dev/null 2>&1
}

prepare_dirs() {
    mkdir -p "$CODESEED_HOME" "$CODESEED_BIN_DIR" "$CODESEED_HOME/config" "$CODESEED_HOME/cache"
}

install_binary() {
    src="$1"
    [ -f "$src" ] || fail "binary not found: $src"
    cp "$src" "$CODESEED_BIN_DIR/codeseed"
    chmod 755 "$CODESEED_BIN_DIR/codeseed"
}

build_local() {
    is_codeseed_checkout || fail "current directory is not a Codeseed source checkout"
    has_cmd cargo || fail "cargo is required for local build"

    info "building from local source with cargo"
    cargo build --release
    install_binary "target/release/codeseed"
}

download() {
    url="$1"
    output="$2"

    if has_cmd curl; then
        curl -fsSL "$url" -o "$output"
    elif has_cmd wget; then
        wget -qO "$output" "$url"
    else
        fail "curl or wget is required to download prebuilt binaries"
    fi
}

release_url() {
    target="$1"
    archive="codeseed-$target.tar.gz"
    if [ "$CODESEED_VERSION" = "latest" ]; then
        printf '%s/latest/download/%s\n' "$CODESEED_DOWNLOAD_BASE" "$archive"
    else
        printf '%s/download/%s/%s\n' "$CODESEED_DOWNLOAD_BASE" "$CODESEED_VERSION" "$archive"
    fi
}

download_prebuilt() {
    target="$(detect_target)"
    url="$(release_url "$target")"
    tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/codeseed-install.XXXXXX")"
    archive="$tmp_dir/codeseed.tar.gz"

    info "downloading prebuilt binary for $target"
    info "$url"
    if ! download "$url" "$archive"; then
        rm -rf "$tmp_dir"
        return 1
    fi

    if ! tar -xzf "$archive" -C "$tmp_dir"; then
        rm -rf "$tmp_dir"
        return 1
    fi

    binary="$(find "$tmp_dir" -type f -name codeseed | head -n 1)"
    if [ -z "$binary" ]; then
        rm -rf "$tmp_dir"
        return 1
    fi

    install_binary "$binary"
    rm -rf "$tmp_dir"
}

build_from_git() {
    has_cmd cargo || return 1
    has_cmd git || return 1

    tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/codeseed-source.XXXXXX")"
    info "building from source fallback: $CODESEED_SOURCE_URL"
    if ! git clone --depth 1 "$CODESEED_SOURCE_URL" "$tmp_dir/repo"; then
        rm -rf "$tmp_dir"
        return 1
    fi

    old_pwd="$(pwd)"
    cd "$tmp_dir/repo"
    if ! cargo build --release; then
        cd "$old_pwd"
        rm -rf "$tmp_dir"
        return 1
    fi
    install_binary "target/release/codeseed"
    cd "$old_pwd"
    rm -rf "$tmp_dir"
}

path_hint() {
    case ":$PATH:" in
        *":$CODESEED_BIN_DIR:"*) return 0 ;;
    esac

    cat <<EOF

Codeseed was installed, but $CODESEED_BIN_DIR is not in PATH.
Add this to your shell profile:

  export PATH="$CODESEED_BIN_DIR:\$PATH"

EOF
}

prepare_dirs

case "$CODESEED_INSTALL_MODE" in
    local)
        build_local
        ;;
    prebuilt)
        download_prebuilt || fail "failed to install prebuilt Codeseed binary"
        ;;
    auto)
        if is_codeseed_checkout && has_cmd cargo; then
            build_local
        elif download_prebuilt; then
            :
        elif build_from_git; then
            :
        else
            fail "failed to install Codeseed; install cargo or provide a prebuilt release"
        fi
        ;;
    *)
        fail "invalid CODESEED_INSTALL_MODE: $CODESEED_INSTALL_MODE"
        ;;
esac

info "installed $("$CODESEED_BIN_DIR/codeseed" --version 2>/dev/null || printf 'codeseed')"
info "home: $CODESEED_HOME"
info "binary: $CODESEED_BIN_DIR/codeseed"
path_hint

