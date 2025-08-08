Write-Host "Сборка релизной версии Яндекс.Музыки..." -ForegroundColor Green
Write-Host ""

Write-Host "Очистка предыдущих сборок..." -ForegroundColor Yellow
if (Test-Path "src-tauri\target\x86_64-pc-windows-msvc\release") {
    Remove-Item "src-tauri\target\x86_64-pc-windows-msvc\release" -Recurse -Force
}
if (Test-Path "src-tauri\target\release") {
    Remove-Item "src-tauri\target\release" -Recurse -Force
}

Write-Host "Установка зависимостей..." -ForegroundColor Yellow
npm install

Write-Host "Сборка фронтенда..." -ForegroundColor Yellow
npm run build

Write-Host "Сборка Tauri приложения..." -ForegroundColor Yellow
Set-Location src-tauri
cargo tauri build

Write-Host "Копирование исполняемого файла в правильное место..." -ForegroundColor Yellow
if (Test-Path "target\x86_64-pc-windows-msvc\release\yandex-music-unofficial.exe") {
    Copy-Item "target\x86_64-pc-windows-msvc\release\yandex-music-unofficial.exe" "target\release\yandex-music-unofficial.exe"
    Write-Host "Файл скопирован успешно" -ForegroundColor Green
} else {
    Write-Host "Ошибка: исполняемый файл не найден" -ForegroundColor Red
}

Set-Location ..

Write-Host ""
Write-Host "Сборка завершена!" -ForegroundColor Green
Write-Host "Релизная версия находится в: src-tauri\target\x86_64-pc-windows-msvc\release\bundle\nsis\" -ForegroundColor Cyan
Write-Host "Исполняемый файл: src-tauri\target\x86_64-pc-windows-msvc\release\yandex-music-unofficial.exe" -ForegroundColor Cyan
Write-Host "Установщик: Яндекс Музыка (Unofficial)_0.1.0_x64-setup.exe" -ForegroundColor Cyan
Read-Host "Нажмите Enter для выхода"
