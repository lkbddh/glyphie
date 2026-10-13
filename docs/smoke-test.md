# Glyphie Smoke Test

Run these checks after changes that touch launch, clipboard, search, or keyboard handling.

## Native

1. Run `just validate`.
2. Build and run with `just run`.
3. Confirm the picker opens on **Recent** and the search field is focused.
4. Type `grinning face`, press `Enter`, then paste into another app and confirm `😀` was copied.
5. Reopen, turn Caps Lock on, type `happy`, press `→` twice, then `Enter`, and confirm the third result was copied. Turn Caps Lock off.
6. Reopen, type `happy`, press `←`, type `x`, and confirm the field reads `happyx`.
7. Reopen, search `orca`, then `pickle`, and confirm each shows its emoji. They come from the bundled font, so this has to work on Ubuntu 24.04 too.
8. Start `glyphie` again while it is already open and confirm the existing picker is focused instead of opening a second window.
9. Enable multiselect, select two emojis, press `Enter`, then paste and confirm both emojis were copied in selection order.
10. Change skin tone and gender filter, close and reopen, and confirm the preferences persisted.

## Flatpak

1. Regenerate `cargo-sources.json` if dependencies changed.
2. Build and install with `flatpak-builder --user --install --force-clean build-dir com.lkbddh.Glyphie.yml`.
3. Run with `flatpak run com.lkbddh.Glyphie`.
4. Repeat the native clipboard, single-instance, multiselect, and persistence checks.
