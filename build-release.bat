@echo off
echo Сборка релизной версии Яндекс Музыки...
echo.

echo Очистка предыдущих сборок...
if exist "src-tauri\target\x86_64-pc-windows-msvc\release" rmdir /s /q "src-tauri\target\x86_64-pc-windows-msvc\release"
if exist "src-tauri\target\release" rmdir /s /q "src-tauri\target\release"

echo Установка зависимостей...
npm install

echo Сборка фронтенда...
npm run build

echo Сборка Tauri приложения...
cd src-tauri
cargo tauri build

echo Копирование исполняемого файла в правильное место...
if exist "target\x86_64-pc-windows-msvc\release\yandex-music-unofficial.exe" (
    copy "target\x86_64-pc-windows-msvc\release\yandex-music-unofficial.exe" "target\release\yandex-music-unofficial.exe"
    echo Файл скопирован успешно
) else (
    echo Ошибка: исполняемый файл не найден
)

cd ..

echo.
echo Сборка завершена!
echo Релизная версия находится в: src-tauri\target\x86_64-pc-windows-msvc\release\bundle\nsis\
echo Исполняемый файл: src-tauri\target\x86_64-pc-windows-msvc\release\yandex-music-unofficial.exe
echo Установщик: Яндекс Музыка (Unofficial)_0.1.0_x64-setup.exe
pause
