# My Launcher v0.2.0

## Что сделано

- Реальная загрузка манифеста версий с Mojang
- Установка версии: client.jar, libraries, natives, asset index + objects
- Запуск Minecraft в оффлайн-режиме через найденную Java
- Инстансы: создание, удаление, сохранение
- Поиск модов через Modrinth API
- Локальная загрузка скина (PNG)
- Настройки и профиль сохраняются на диск
- Красивый тёмный UI (egui)
- Inno Setup скрипт для Windows-установщика

## Сборка Windows EXE + Installer

```bat
cargo build --release
:: затем откройте installer\my-launcher.iss в Inno Setup 6 и Compile
```

Готовый установщик: `installer\Output\MyLauncher-Setup-0.2.0.exe`

## Ограничения этой среды

Из Linux-песочницы нельзя:
1. Собрать нативный Windows `.exe` (нет mingw/MSVC)
2. Запустить Inno Setup
3. Залить бинарник в GitHub Releases (нет API upload asset)

Соберите локально на Windows и создайте Release вручную:
https://github.com/ZeeinJordansky/My-launcher/releases/new

Tag: `v0.2.0`
