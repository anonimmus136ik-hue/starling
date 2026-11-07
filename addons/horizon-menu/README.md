# Horizon Menu — экспериментальный .hmod

Отдельное меню для Horizon Android. Это основа интерфейса, **не менеджер чужих модов**:
проверенный SDK не предоставляет списка установленных модов и доступа к их настройкам.

## Что работает по исходникам
- Экран меню по команде `/hmodmenu`.
- Собственная страница Settings в Options > Mods.
- Русский и английский язык; выбор сохраняется только в хранилище этого мода.
- Справка о встроенной странице Mods и кнопка закрытия.

Нет изменений движка, миров, инвентаря, управления или настроек других модов.
Разрешения: `ui` и `storage` (изолированное хранилище этого мода).
Нет tick/frame-хуков и фоновой обработки.

## Скачать и попробовать
1. Открой Actions → Build Horizon Menu → успешный запуск.
2. Скачай артефакт horizon-menu-hmod, распакуй ZIP.
3. Импортируй horizon_menu.hmod в Horizon.
4. В Options > Mods включи Horizon Menu.
5. Нажми Settings у Horizon Menu или в мире введи /hmodmenu.
6. Для удаления выключи мод и удали только его .hmod.

Сначала попробуй в тестовом мире. APK заменять не нужно.
Совместимость с установленным APK 0.7.8 и сенсорный интерфейс требуют проверки на телефоне;
успешная сборка сама по себе этого не доказывает.

## Сборка
Из корня этого репозитория:
```sh
rustup target add wasm32-unknown-unknown
cargo install --path crates/cargo-horizon-mod --locked
cd addons/horizon-menu
cargo test --lib
cargo horizon-mod build
cargo horizon-mod test
cargo horizon-mod check target/horizon-mod/horizon_menu.hmod
```
SDK и упаковщик берутся из той же версии репозитория, что и этот мод.
Сборка не требует Minecraft-файлов и не создаёт APK.
