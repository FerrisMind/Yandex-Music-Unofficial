#!/usr/bin/env bash

set -euo pipefail

# Colors
if [ -t 1 ]; then
  BOLD="\033[1m"; GREEN="\033[32m"; YELLOW="\033[33m"; CYAN="\033[36m"; RED="\033[31m"; RESET="\033[0m"
else
  BOLD=""; GREEN=""; YELLOW=""; CYAN=""; RED=""; RESET=""
fi

info()  { echo -e "${GREEN}${BOLD}$*${RESET}"; }
warn()  { echo -e "${YELLOW}$*${RESET}"; }
error() { echo -e "${RED}${BOLD}$*${RESET}"; }

# Ensure macOS
if [[ "$(uname -s)" != "Darwin" ]]; then
  error "Этот скрипт предназначен только для macOS."
  exit 1
fi

# Resolve project root (directory of this script)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

info "Сборка релизной версии Яндекс Музыки для macOS (universal x64+arm64)..."
echo

# Read product name and version from tauri.conf.json via Node.js (fallbacks provided)
if command -v node >/dev/null 2>&1; then
  APP_NAME="$(node -e "try{console.log(require('./src-tauri/tauri.conf.json').productName||'Яндекс Музыка (Unofficial)')}catch(e){console.log('Яндекс Музыка (Unofficial)')}")"
  APP_VERSION="$(node -e "try{console.log(require('./src-tauri/tauri.conf.json').version||'0.0.0')}catch(e){console.log('0.0.0')}")"
else
  warn "Node не найден в PATH, использую значения по умолчанию для имени и версии"
  APP_NAME="Яндекс Музыка (Unofficial)"
  APP_VERSION="0.0.0"
fi

TARGET_DIR="$SCRIPT_DIR/src-tauri/target"
UNIVERSAL_DIR="$TARGET_DIR/universal-apple-darwin/release"
APP_BUNDLE_DIR="$UNIVERSAL_DIR/bundle/macos"
DMG_BUNDLE_DIR="$UNIVERSAL_DIR/bundle/dmg"

info "Очистка предыдущих сборок..."
rm -rf "$UNIVERSAL_DIR" || true
rm -rf "$TARGET_DIR/aarch64-apple-darwin/release" || true
rm -rf "$TARGET_DIR/x86_64-apple-darwin/release" || true
rm -rf "$SCRIPT_DIR/dist" || true

info "Установка целей Rust (aarch64 и x86_64)..."
rustup target add aarch64-apple-darwin x86_64-apple-darwin >/dev/null

info "Установка зависимостей..."
npm install

info "Сборка фронтенда..."
npm run build

info "Сборка Tauri-приложения (universal, DMG)..."
# Используем Tauri CLI через npm-скрипт: добавляем флаги после "--"
npm run tauri build -- --target universal-apple-darwin --bundles dmg

echo
info "Проверка результата сборки..."
if [[ ! -d "$APP_BUNDLE_DIR" ]]; then
  error "Не найдена папка macOS-бандла: $APP_BUNDLE_DIR"
  exit 1
fi

if [[ -d "$DMG_BUNDLE_DIR" ]]; then
  LATEST_DMG="$(ls -t "$DMG_BUNDLE_DIR"/*.dmg 2>/dev/null | head -n 1 || true)"
else
  LATEST_DMG=""
fi

if [[ -n "$LATEST_DMG" && -f "$LATEST_DMG" ]]; then
  info "DMG-файл успешно создан:"
  echo "$LATEST_DMG"
else
  warn "DMG-файл не найден в $DMG_BUNDLE_DIR"
  warn "Попытка создать DMG локально через create-dmg (если установлена)"
  if command -v create-dmg >/dev/null 2>&1; then
    # Создаём DMG из содержимого папки bundle/macos (в ней лежит *.app)
    SAFE_APP_NAME="$APP_NAME"
    OUT_DMG_PATH="$SCRIPT_DIR/${SAFE_APP_NAME}.dmg"
    # Удалим существующий DMG с таким именем
    rm -f "$OUT_DMG_PATH" || true
    create-dmg \
      --volname "${SAFE_APP_NAME} Installer" \
      --window-pos 200 120 \
      --window-size 800 400 \
      --icon-size 120 \
      --app-drop-link 600 200 \
      "$OUT_DMG_PATH" \
      "$APP_BUNDLE_DIR"

    if [[ -f "$OUT_DMG_PATH" ]]; then
      info "DMG-файл создан (create-dmg):"
      echo "$OUT_DMG_PATH"
    else
      error "Не удалось создать DMG даже через create-dmg."
      exit 1
    fi
  else
    error "Не найден DMG и отсутствует утилита create-dmg. Установите: brew install create-dmg"
    exit 1
  fi
fi

echo
info "Сборка завершена!"
echo "Версия: $APP_VERSION"
echo "Папка с бандлами: $UNIVERSAL_DIR/bundle"
if [[ -d "$DMG_BUNDLE_DIR" ]]; then
  echo "DMG (tauri bundler): $DMG_BUNDLE_DIR"
fi
echo
if command -v open >/dev/null 2>&1; then
  warn "Открываю папку с результатами..."
  open "$UNIVERSAL_DIR/bundle" || true
fi


