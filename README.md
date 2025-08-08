[🇷🇺 Русская версия](README.ru.md)

<div align="center">

# 🎵 Yandex Music (Unofficial)

**Unofficial desktop client for Yandex.Music built with Tauri**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/FerrisMind/Yandex_Music)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)](https://github.com/FerrisMind/Yandex_Music)
[![License](https://img.shields.io/badge/license-Unofficial-red.svg)](https://github.com/FerrisMind/Yandex_Music)
[![Tauri](https://img.shields.io/badge/framework-Tauri-purple.svg)](https://tauri.app/)

*A native Windows application that provides seamless access to Yandex.Music in a dedicated desktop window*

</div>

---

## ✨ Features

- 🖥️ **Native Windows Application** - Built with Tauri for optimal performance
- 🚀 **Fast Loading** - Quick startup and smooth operation
- 🎵 **Full Music Access** - Complete access to Yandex.Music web interface

## 🚀 Tauri Advantages

### Performance Benefits
- **Lightweight** - Much smaller file size compared to Electron
- **Fast Startup** - Native Rust backend ensures quick application launch
- **Low Memory Usage** - Efficient resource management
- **Native Performance** - Direct system API access without browser overhead

### Security Benefits
- **Sandboxed Environment** - Secure execution with limited permissions
- **No Node.js Runtime** - Reduced attack surface
- **Rust Backend** - Memory safety and thread safety guarantees
- **System Integration** - Safe access to native system features

### Development Benefits
- **Cross-Platform** - Single codebase for multiple platforms
- **Modern Web Technologies** - Use HTML, CSS, and JavaScript for UI
- **Hot Reload** - Fast development cycle with instant updates
- **Rich Ecosystem** - Access to Rust's extensive library ecosystem

## 🚀 Quick Start

### Prerequisites

- **Node.js** (version 18 or higher)
- **Rust** (latest stable version)
- **Cargo** (installed with Rust)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/FerrisMind/Yandex_Music.git
   cd Yandex_Music
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

## 🛠️ Building

### Development Build
```bash
npm run tauri dev
```

### Production Build

#### Windows (PowerShell)
```powershell
.\build-release.ps1
```

#### Windows (Command Prompt)
```cmd
build-release.bat
```

#### Manual Build
```bash
npm run build
cd src-tauri
cargo tauri build
cd ..
```

## 📄 License

This is an unofficial client. All rights to Yandex Music belong to Yandex.

## 👨‍💻 Author

**FerrisMind**

---

<div align="center">

**Made with ❤️ using [Tauri](https://tauri.app/)**

[![Tauri](https://img.shields.io/badge/powered%20by-Tauri-purple.svg)](https://tauri.app/)

</div>
