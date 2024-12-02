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
    pkg-config

# Install desktop entry
sudo cp resources/kamiview.desktop /usr/share/applications/
sudo cp resources/icon.png /usr/share/icons/hicolor/256x256/apps/kamiview.png
sudo update-desktop-database 