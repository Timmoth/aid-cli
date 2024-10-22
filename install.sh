#!/bin/bash

# Determine the operating system (Linux or macOS) and architecture (x86_64 or ARM64)
OS="$(uname)"
ARCH="$(uname -m)"

if [[ "$OS" == "Linux" ]]; then
    if [[ "$ARCH" == "x86_64" ]]; then
        URL="https://github.com/Timmoth/aid-cli/releases/download/aid-0.1.7/aid-linux"
    else
        echo "Error: Unsupported architecture for Linux."
        exit 1
    fi
elif [[ "$OS" == "Darwin" ]]; then
    if [[ "$ARCH" == "x86_64" ]]; then
        URL="https://github.com/Timmoth/aid-cli/releases/download/aid-0.1.7/aid-mac"
        # Check if running on an x86 mac with Rosetta
        if [[ $(sysctl -n sysctl.proc_translated) == "1" ]]; then
            echo "Running on Apple Silicon with Rosetta. Consider downloading the ARM64 version."
        fi
    elif [[ "$ARCH" == "arm64" ]]; then
        URL="https://github.com/Timmoth/aid-cli/releases/download/aid-0.1.7/aid-mac-arm"
    else
        echo "Error: Unsupported architecture for macOS."
        exit 1
    fi
else
    echo "Error: Unsupported operating system."
    exit 1
fi

INSTALL_DIR="/usr/local/bin"
FILENAME="aid"

# Check if curl or wget is installed for downloading
if command -v curl >/dev/null 2>&1; then
    DOWNLOADER="curl -L -o"
elif command -v wget >/dev/null 2>&1; then
    DOWNLOADER="wget -O"
else
    echo "Error: Neither curl nor wget is installed."
    exit 1
fi

# Download the file
echo "Downloading aid from $URL..."
$DOWNLOADER "$INSTALL_DIR/$FILENAME" "$URL"

# Verify if the download was successful
if [ ! -f "$INSTALL_DIR/$FILENAME" ]; then
    echo "Error: Download failed."
    exit 1
fi

echo "Download successful! File saved to $INSTALL_DIR/$FILENAME"

# Make the file executable
echo "Making $FILENAME executable..."
chmod +x "$INSTALL_DIR/$FILENAME"

# Check if the install directory is already in PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "Adding $INSTALL_DIR to PATH..."
    
    # Add the directory to the shell profile for persistent PATH updates
    if [ -f "$HOME/.bashrc" ]; then
        echo "export PATH=\$PATH:$INSTALL_DIR" >> "$HOME/.bashrc"
        source "$HOME/.bashrc"
    elif [ -f "$HOME/.bash_profile" ]; then
        echo "export PATH=\$PATH:$INSTALL_DIR" >> "$HOME/.bash_profile"
        source "$HOME/.bash_profile"
    elif [ -f "$HOME/.zshrc" ]; then
        echo "export PATH=\$PATH:$INSTALL_DIR" >> "$HOME/.zshrc"
        source "$HOME/.zshrc"
    else
        echo "Could not find shell profile. Please add $INSTALL_DIR to your PATH manually."
    fi
else
    echo "$INSTALL_DIR is already in PATH."
fi

echo "Installation complete! You can now run 'aid' from any terminal."
