#!/bin/bash

OS=$(uname)

if [[ "$OS" == "Linux" ]]; then
    if [ -f "/etc/lsb-release" ]; then
        echo "Detected Ubuntu/Debian-based Linux"
        sudo apt update && sudo apt install -y gcc libssl-dev pkg-config
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . "$HOME/.cargo/env"
        git clone https://github.com/KimWang906/creamhack.git
        cargo install --path ./creamhack
    elif [ -f "/etc/redhat-release" ]; then
        echo "Detected Red Hat/CentOS-based Linux"
        sudo dnf install -y gcc openssl-devel pkgconfig
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . "$HOME/.cargo/env"
        git clone https://github.com/KimWang906/creamhack.git
        cargo install --path ./creamhack
    else
        echo "Detected other Linux distribution"
        echo "Other Linux distributions are not supported."
        exit 1
    fi

elif [[ "$OS" == "Darwin" ]]; then
    echo "Detected macOS"
    if ! command -v git &> /dev/null; then
        echo "Git is not installed. Please install Git first."
        exit 1
    else
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        git clone https://github.com/KimWang906/creamhack.git
        cargo install --path ./creamhack
    fi
elif [[ "$OS" == "MINGW"* || "$OS" == "CYGWIN"* ]]; then
    echo "Detected Windows (MINGW/CYGWIN)"
    echo "Not implemented yet. Please install manually."
elif grep -qi microsoft /proc/version; then
    echo "Detected Windows Subsystem for Linux (WSL)"
    echo "Not implemented yet. Please install manually."
else
    echo "Unknown operating system: $OS"
    exit 1
fi
