# My Launcher

**Красивый лаунчер Minecraft на Rust (egui / eframe)**

Современный тёмный интерфейс, реальная загрузка версий, установка client/libraries/assets, запуск Java, каталог модов Modrinth, скины, инстансы, сохранение настроек.

Репозиторий: https://github.com/ZeeinJordansky/My-launcher

## Возможности v0.2.0

| Функция | Статус |
|---------|--------|
| Красивый тёмный UI + кастомная тема | ✅ |
| Список версий (Mojang piston-meta) | ✅ |
| Установка версии (client + libs + natives + assets) | ✅ |
| Создание / удаление инстансов | ✅ |
| Запуск Minecraft (offline) | ✅ |
| Определение Java | ✅ |
| Каталог модов (Modrinth search) | ✅ |
| Загрузка локального скина | ✅ |
| Настройки + сохранение на диск | ✅ |
| Оффлайн-аккаунт | ✅ |
| Microsoft Auth | ⚠️ Нужен Azure App |
| 3D-превью скина | ⏳ |
| Fabric/Forge | ⏳ |
| Inno Setup скрипт | ✅ |

## Сборка

```bash
cargo build --release
```

### Windows-установщик

1. `cargo build --release` на Windows
2. Inno Setup 6 → `installer/my-launcher.iss` → Compile
3. Получите `MyLauncher-Setup-0.2.0.exe`

## Использование

1. **Версии** → скачать версию (первый раз долго — assets)
2. **Инстанс** → создаст на главной
3. **▶ Играть** (нужна Java 17+)
4. **Моды** — поиск Modrinth
5. **Скины** — локальный PNG
6. **Настройки** — память, Java, папка

Данные: `~/.local/share/MyLauncher/` (Linux) / `%APPDATA%\\MyLauncher` (Windows)

## Лицензия

MIT
