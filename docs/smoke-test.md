# Glyphie Smoke Test

Run these checks after changes that touch launch, clipboard, search, or keyboard handling.

## Native

1. Run `just validate`.
2. Build and run with `just run`.
3. Confirm the picker opens on **Recent** and the search field is focused.
4. Type `grinning face`, press `Enter`, then paste into another app and confirm `😀` was copied.
5. Start `glyphie` again while it is already open and confirm the existing picker is focused instead of opening a second window.
6. Enable multiselect, select two emojis, press `Enter`, then paste and confirm both emojis were copied in selection order.
7. Change skin tone and gender filter, close and reopen, and confirm the preferences persisted.

## Flatpak

1. Regenerate `cargo-sources.json` if dependencies changed.
2. Build and install with `flatpak-builder --user --install --force-clean build-dir com.aldeastudio.Glyphie.yml`.
3. Run with `flatpak run com.aldeastudio.Glyphie`.
4. Repeat the native clipboard, single-instance, multiselect, and persistence checks.
