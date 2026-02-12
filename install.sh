#!/usr/bin/env bash
set -euo pipefail

BINARY_NAME="git-standup"
PREFIX="${PREFIX:-$HOME/.local}"
BINDIR="${BINDIR:-$PREFIX/bin}"

usage() {
    echo "Usage: $0 [install|uninstall]"
    echo ""
    echo "Options:"
    echo "  install     Build and install $BINARY_NAME to $BINDIR"
    echo "  uninstall   Remove $BINARY_NAME from $BINDIR"
    echo ""
    echo "Environment variables:"
    echo "  PREFIX      Installation prefix (default: ~/.local)"
    echo "  BINDIR      Binary directory (default: \$PREFIX/bin)"
}

check_deps() {
    if ! command -v cargo &>/dev/null; then
        echo "Error: cargo is not installed."
        echo "Install Rust via https://rustup.rs"
        exit 1
    fi
}

do_install() {
    check_deps

    echo "Building $BINARY_NAME (release)..."
    cargo build --release

    echo "Installing $BINARY_NAME to $BINDIR..."
    mkdir -p "$BINDIR"
    install -m 0755 "target/release/$BINARY_NAME" "$BINDIR/$BINARY_NAME"

    echo "Done. Installed $BINARY_NAME to $BINDIR/$BINARY_NAME"

    if ! echo "$PATH" | tr ':' '\n' | grep -qx "$BINDIR"; then
        echo ""
        echo "NOTE: $BINDIR is not in your PATH."
        echo "Add it by running:"
        echo ""
        echo "  echo 'export PATH=\"$BINDIR:\$PATH\"' >> ~/.bashrc && source ~/.bashrc"
    fi
}

do_uninstall() {
    if [ -f "$BINDIR/$BINARY_NAME" ]; then
        echo "Removing $BINDIR/$BINARY_NAME..."
        rm -f "$BINDIR/$BINARY_NAME"
        echo "Done."
    else
        echo "$BINARY_NAME is not installed at $BINDIR/$BINARY_NAME"
        exit 1
    fi
}

case "${1:-install}" in
    install)
        do_install
        ;;
    uninstall)
        do_uninstall
        ;;
    -h|--help|help)
        usage
        ;;
    *)
        echo "Unknown command: $1"
        usage
        exit 1
        ;;
esac
