#!/bin/bash

# Determine the operating system (Linux or macOS)
OS="$(uname)"
if [[ "$OS" == "Linux" ]]; then
    URL="https://github.com/Timmoth/aid-cli/releases/download/aid-0.1.0/aid-linux"
elif [[ "$OS" == "Darwin" ]]; then
    URL="https://github.com/Timmoth/aid-cli/releases/download/aid-0.1.0/aid-mac"
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
echo "Downloading aid..."
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
