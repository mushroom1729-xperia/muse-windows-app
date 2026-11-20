# Muse for Windows

A lightweight Windows client for [Muse](https://muse.ai).

There is no official Muse app for Windows (only a Mac app), so this is a
minimal wrapper built with [Tauri](https://tauri.app) on top of the WebView2
runtime that ships with Windows. No bundled Chromium, no Electron — the app
shell itself is just a few MB and stays light on memory.

## Download

Grab the latest build from the
[Releases](https://github.com/mushroom1729-xperia/muse-windows-app/releases)
page:

- `Muse_<version>_x64-setup.exe` — installer
- `Muse-portable-<version>.exe` — portable, no install needed

Requires the Microsoft Edge WebView2 runtime (preinstalled on Windows 11 and
on Windows 10 machines with Edge).

## Build

Builds run on GitHub Actions (`.github/workflows/build.yml`) and publish a
draft release with the installer and the portable exe.

## Credits

- Author / creator: [mushroom1729-xperia](https://github.com/mushroom1729-xperia)
- Co-developed with AI (Muse)
