#!/bin/bash

# CodeRcar - Robot Source Code Builder
# Matthew Piercey <matthew@botinabox.ca>
# Version: 0.2.0

# This script simplifies the CodeRcar build process quite a bit;
# the stripped executable will be in the dist/ folder

# ---------------- 
# Troubleshooting:
# ---------------- 

# cargo: command not found - try installing Rustup at https://www.rust-lang.org/tools/install

# Could not find specification for target "arm-unknown-linux-gnueabihf"
# ^ Install the Raspberry Pi toolchain (https://github.com/raspberrypi/tools)
# and make sure it's available at (replace username with your username):
# /home/username/build-rpi/tools/arm-bcm2708/arm-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc

# ^ Not completely true. There are other ways of making this work...
# Like rustup install --toolchain arm-unknown-linux-gnueabihf
# ^ Pretty sure that works too, and it's more reliable/cross-platform

# If you have Rust 1.44.0 or above, you'll be able to use the strip feature in cargo +nightly

# Could not compile `codercar-rust`:
# ^ You probably broke something while changing src/main.rs. - Or I did...

# Setting up some colour codes:
RED='\033[1;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NOCOLOUR='\033[0m'

# Clearing the screen for visibility's sake
clear

# Printing a welcome message
echo -e "${NOCOLOUR}\nCodeRcar - Robot Source Code Builder\n${PURPLE}-----${NOCOLOUR}"
sleep 0.2s
echo -e "Version: ${GREEN}0.2.0${NOCOLOUR}"
echo -e "Copyright 2020 - ${GREEN}Bot-In-a-Box${NOCOLOUR} <${CYAN}https://botinabox.ca${NOCOLOUR}>"
echo -e "Author: ${GREEN}Matthew Piercey${NOCOLOUR} <${CYAN}matthew@botinabox.ca${NOCOLOUR}>"
echo -e "${PURPLE}-----${NOCOLOUR}\n"
sleep 0.2s

# Triggers the Rust compiler; tells it to build for production and target
# Raspberry Pi's running Arm6 (in this case the Pi Zero W)
echo -e "${BLUE}Compiling Code for Release:${NOCOLOUR}"
sleep 0.2s

cargo build --release --target arm-unknown-linux-gnueabihf &&

# Strips the executable compiled above
# This step optimizes the compiled executable further, but it is optional
# $HOME/build-rpi/tools/arm-bcm2708/arm-linux-gnueabihf/bin/arm-linux-gnueabihf-strip \
# "./target/arm-unknown-linux-gnueabihf/release/codercar-rust" &&

# Makes the dist directory, if it doesn't already exist
mkdir -p dist

# Copies the stripped executable to the dist folder and renames it codercar
mv ./target/arm-unknown-linux-gnueabihf/release/codercar-rust ./dist/codercar &> /dev/null

# Prints the filesize of the compiled and stripped executable
echo -e "\n${BLUE}Executable Size: ${GREEN}$(ls dist/codercar -l --block-size=KB | awk '{print $5}')"

# Prints a good-bye message
echo -e "${BLUE}Check the ${GREEN}dist/${BLUE} folder to see the compiled executable :)${NOCOLOUR}\n"