#!/bin/bash

# Install required system packages
sudo apt-get update
sudo apt-get install -y \
    libmpv-dev \
    mpv \
    libgtk-3-dev \
    libx11-dev \
    libxdo-dev \
    libssl-dev \
    pkg-config \
    build-essential \
    libgdk-pixbuf2.0-dev \
    libglib2.0-dev \
    libatk1.0-dev \
    libpango1.0-dev \
    libcairo2-dev \
    libgdk3.0-cil-dev \
    libgtk-3-0 \
    libgtk-3-common \
    libgtk-3-dev \
    gir1.2-gtk-3.0 \
    gir1.2-gdk-3.0

# Set up pkg-config path
export PKG_CONFIG_PATH="/usr/lib/pkgconfig:/usr/share/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig"

# Install desktop entry
sudo cp resources/kamiview.desktop /usr/share/applications/
sudo cp resources/icon.png /usr/share/icons/hicolor/256x256/apps/kamiview.png
sudo update-desktop-database

# Make script executable
chmod +x "$0"