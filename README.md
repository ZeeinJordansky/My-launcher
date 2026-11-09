# My Launcher

**Красивый лаунчер Minecraft на Rust (egui / eframe)**

Современный тёмный интерфейс, кастомная тема, страницы: Главная, Версии, Моды, Скины, Аккаунты, Настройки.

> Это **скелет / прототип**. Полная реализация скачивания, запуска, авторизации Microsoft, модлоадеров и 3D-превью скинов — объёмная задача. Структура и UI уже готовы для расширения.

## Возможности (текущий статус)

| Функция                    | Статус          |
|---------------------------|-----------------|
| Красивый тёмный UI        | ✅ Готово       |
| Навигация по страницам    | ✅              |
| Список версий (Mojang API)| ✅ (загрузка)   |
| Создание инстансов        | 🟡 Заглушка    |
| Запуск Minecraft          | 🟡 Заглушка    |
| Каталог модов (Modrinth)  | 🟡 Заглушка    |
| Редактор / загрузка скинов| 🟡 Заглушка    |
| Настройки (память, Java)  | ✅ UI           |
| Microsoft Auth            | 🟡 Заглушка    |
| Оффлайн-режим             | ✅              |
| Inno Setup установщик     | ✅ Скрипт       |

## Скриншоты / UI

- Боковая панель с навигацией
- Карточки инстансов
- Список версий с фильтром и снапшотами
- Кастомные цвета (teal accent на тёмном фоне)
- Скруглённые карточки и кнопки

## Сборка

```bash
# Требуется Rust (rustup)
cargo build --release
```

Бинарник: `target/release/my-launcher` (Linux/macOS) или `my-launcher.exe` (Windows).

### Windows + Inno Setup

1. Соберите `cargo build --release`
2. Установите [Inno Setup 6](https://jrsoftware.org/isinfo.php)
3. Откройте `installer/my-launcher.iss`
4. Убедитесь, что путь к `my-launcher.exe` правильный
5. Compile → получите `MyLauncher-Setup-0.1.0.exe`

## Roadmap (что нужно доделать)

1. **Полный pipeline запуска**
   - Скачивание version JSON → client.jar → libraries → assets → natives
   - Проверка SHA1
   - Формирование classpath + аргументов
   - Запуск Java-процесса

2. **Авторизация**
   - Microsoft OAuth (device code / browser)
   - Сохранение токенов

3. **Моды**
   - Modrinth API + CurseForge
   - Установка в инстанс (Fabric/Forge/Quilt/NeoForge)

4. **Скины**
   - 3D-превью (через egui или отдельный рендер)
   - Загрузка на Mojang (требует auth)

5. **Инстансы**
   - Полноценное создание / редактирование / удаление
   - Иконки, категории

6. **CI**
   - GitHub Actions для сборки Windows/Linux/macOS + релиз

## Технологии

- **Rust 2021**
- **egui + eframe** — нативный immediate-mode GUI
- **reqwest + tokio** — сеть
- **serde** — JSON
- **rfd** — диалоги выбора файлов

## Лицензия

MIT

---

Репозиторий: https://github.com/ZeeinJordansky/My-launcher
