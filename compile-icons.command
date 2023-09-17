#! /usr/bin/env bash

cd "$(dirname "$(realpath -- "$0")")" || exit;

/opt/local/bin/convert ./src-tauri/icons/icon_windows/*.png ./src-tauri/icons/icon.ico && echo "Made windows icon";

/usr/bin/iconutil -c icns --output ./src-tauri/icons/icon.icns ./src-tauri/icons/icon_macos.iconset && echo "Made macOS icon";