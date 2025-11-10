#!/bin/bash

cd "$(cd "$(dirname "$0")" && pwd)"

# build
cargo build --release

cd assets
rm -rf ./dist/*

# Move binary
cp ../target/release/virtualpipe ../dist/virtualpipe.bin

# Build flatpak
flatpak-builder .builddir --force-clean build_flatpak.yml --repo=.repo --install --user;
flatpak build-bundle .repo ../dist/VirtualPipe.flatpak net.viniadrii.virtualpipe;