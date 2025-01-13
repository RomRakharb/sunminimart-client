#! /usr/bin/bash

echo
echo "---------------------"
echo "starts install script"
echo "---------------------"
echo
sudo apt-get update && apt-get install -y \   
    pkg-config \    
    libasound2-dev \
    libudev-dev \   
    mesa-utils \ 
    vulkan-tools \
    libwayland-dev \
    libxkbcommon-dev \   
    libvulkan1 \    
    libvulkan-dev \ 
    libegl1-mesa-dev \   
    libgles2-mesa-dev \  
    libx11-dev \    
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libxrandr-dev \
    libxcb1-dev \
    libxcb-icccm4-dev \
    libxcb-image0-dev \
    libxcb-keysyms1-dev \
    libxcb-randr0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxcb-xkb-dev \
    libegl1-mesa \
    libgl1-mesa-glx \
    libgl1-mesa-dri \
    libglu1-mesa-dev \
    libglu1-mesa \
    libgles2-mesa \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
sudo apt-get install -y software-properties-common
sudo apt update
sudo add-apt-repository -y ppa:maveonair/helix-editor
sudo apt update
sudo apt install helix
echo
echo "---------------------"
echo "install script ends"
echo "---------------------"
echo
