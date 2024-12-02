#!/bin/bash

# Source environment variables
source ./install_linux_deps.sh

# Set pkg-config path
export PKG_CONFIG_PATH="/usr/lib/pkgconfig:/usr/share/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig"

# Build the project
cargo build --release

# Copy binary to /usr/local/bin
sudo cp target/release/kamiview /usr/local/bin/

# Update desktop database
sudo update-desktop-database 