#!/bin/sh
set -eu

OUTPUT_DIR="${OUTPUT_DIR:-dist}"
SKIP_BUILD=0

usage() {
    cat <<'EOF'
Package the current host platform Codeseed binary.

Usage:
  package-current-target.sh [--output-dir DIR] [--skip-build]

Options:
  --output-dir DIR  Directory for codeseed-<target>.tar.gz. Defaults to dist.
  --skip-build      Package the existing target/release/codeseed binary.
  -h, --help        Show this help.
EOF
}

fail() {
    printf '%s\n' "codeseed-package: error: $*" >&2
    exit 1
}

has_cmd() {
    command -v "$1" >/dev/null 2>&1
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --output-dir)
            [ "$#" -ge 2 ] || fail "--output-dir requires a value"
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --skip-build)
            SKIP_BUILD=1
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

checksum() {
    path="$1"
    if has_cmd shasum; then
        shasum -a 256 "$path" > "$path.sha256"
    elif has_cmd sha256sum; then
        sha256sum "$path" > "$path.sha256"
    else
        printf '%s\n' "codeseed-package: warning: shasum or sha256sum not found; checksum skipped" >&2
    fi
}

is_codeseed_checkout || fail "run from a Codeseed source checkout"
has_cmd cargo || fail "cargo is required"
has_cmd tar || fail "tar is required"

target="$(detect_target)"
binary="target/release/codeseed"
archive="$OUTPUT_DIR/codeseed-$target.tar.gz"
tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/codeseed-package.XXXXXX")"

cleanup() {
    rm -rf "$tmp_dir"
}
trap cleanup EXIT INT TERM

if [ "$SKIP_BUILD" -eq 0 ]; then
    cargo build --release
fi

[ -f "$binary" ] || fail "binary not found: $binary"

mkdir -p "$OUTPUT_DIR"
cp "$binary" "$tmp_dir/codeseed"
chmod 755 "$tmp_dir/codeseed"
tar -czf "$archive" -C "$tmp_dir" codeseed
checksum "$archive"

printf '%s\n' "target: $target"
printf '%s\n' "archive: $archive"
[ -f "$archive.sha256" ] && printf '%s\n' "checksum: $archive.sha256"
