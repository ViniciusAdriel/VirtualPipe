#!/bin/bash
cd $(dirname "$0")

cargo build --release

mkdir -p dist
rm -rf dist/*

flatpak-builder .builddir --force-clean flatpak_manifest-dev.yml --repo=.repo --install --user;
flatpak build-bundle .repo ./dist/VirtualPipe.flatpak net.viniadrii.VirtualPipe;