[🇺🇸 English version](README.md)

<div align="center">

<img src=".github/assets/icon.svg" width="100" height="100" alt="Иконка Яндекс Музыки">

# Яндекс Музыка (Unofficial)

**Неофициальный десктопный клиент для Яндекс Музыки, созданный с помощью Tauri**

[![Статус сборки](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/FerrisMind/Yandex_Music)
[![Платформа](https://img.shields.io/badge/platform-Windows-blue.svg)](https://github.com/FerrisMind/Yandex_Music)
[![Лицензия](https://img.shields.io/badge/license-Unofficial-red.svg)](https://github.com/FerrisMind/Yandex_Music)
[![Tauri](https://img.shields.io/badge/framework-Tauri-purple.svg)](https://tauri.app/)

*Нативное приложение для Windows, обеспечивающее удобный доступ к Яндекс Музыке в отдельном окне*

</div>

---

## ✨ Возможности

- 🖥️ **Нативное приложение Windows** - Создано с помощью Tauri для оптимальной производительности
- 🚀 **Быстрая загрузка** - Быстрый запуск и плавная работа
- 🎵 **Полный доступ к музыке** - Полный доступ к веб-интерфейсу Яндекс Музыки

## 🚀 Преимущества Tauri

### Преимущества производительности
- **Легковесность** - Значительно меньший размер файлов по сравнению с Electron
- **Быстрый запуск** - Нативный Rust backend обеспечивает быстрое запускание приложения
- **Низкое потребление памяти** - Эффективное управление ресурсами
- **Нативная производительность** - Прямой доступ к системным API без накладных расходов браузера

### Преимущества безопасности
- **Изолированная среда** - Безопасное выполнение с ограниченными правами
- **Без Node.js Runtime** - Уменьшенная поверхность атаки
- **Rust Backend** - Гарантии безопасности памяти и потоков
- **Системная интеграция** - Безопасный доступ к нативным системным функциям

### Преимущества разработки
- **Кроссплатформенность** - Единая кодовая база для нескольких платформ
- **Современные веб-технологии** - Использование HTML, CSS и JavaScript для UI
- **Hot Reload** - Быстрый цикл разработки с мгновенными обновлениями
- **Богатая экосистема** - Доступ к обширной экосистеме библиотек Rust

## 🚀 Быстрый старт

### Требования

- **Node.js** (версия 18 или выше)
- **Rust** (последняя стабильная версия)
- **Cargo** (устанавливается вместе с Rust)

### Установка

1. **Клонируйте репозиторий**
   ```bash
   git clone https://github.com/FerrisMind/Yandex-Music-Unofficial.git
   cd Yandex-Music-Unofficial
   ```

2. **Установите зависимости**
   ```bash
   npm install
   ```

3. **Запустите в режиме разработки**
   ```bash
   npm run tauri dev
   ```

## 🛠️ Сборка

### Сборка для разработки
```bash
npm run tauri dev
```

### Сборка релизной версии

#### Windows (PowerShell)
```powershell
.\build-release.ps1
```

#### Windows (Command Prompt)
```cmd
build-release.bat
```

#### Ручная сборка
```bash
npm run build
cd src-tauri
cargo tauri build
cd ..
```

## 📄 Лицензия

Это неофициальный клиент. Все права на Яндекс Музыку принадлежат Яндексу.

## 👨‍💻 Автор

**FerrisMind**

---

<div align="center">

**Создано с ❤️ используя [Tauri](https://tauri.app/)**

[![Tauri](https://img.shields.io/badge/powered%20by-Tauri-purple.svg)](https://tauri.app/)

</div>
