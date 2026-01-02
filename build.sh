#!/bin/bash
cd $(dirname "$0")

mkdir -p dist
rm -rf dist/*

flatpak-builder .builddir --force-clean flatpak_manifest.yml --repo=.repo;
flatpak build-bundle .repo ./dist/VirtualPipe.flatpak net.viniadrii.VirtualPipe;