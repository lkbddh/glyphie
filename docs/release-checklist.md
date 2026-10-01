# Release Checklist

Use this before posting or tagging a release.

## Required

- Commit `Cargo.lock` for reproducible native builds.
- Commit `cargo-sources.json` whenever the Flatpak manifest is advertised.
- Keep `LICENSE` and `THIRD_PARTY_NOTICES.md` in the release.
- Refresh `THIRD_PARTY_NOTICES.md` after dependency, bundled tool, embedded data, or icon changes.
- Run `just validate`.
- Run `cargo build --release`.
- Push the `v<version>` tag. The Release workflow builds `glyphie_<version>_amd64.deb` and `glyphie-<version>.flatpak`, then attaches them and `SHA256SUMS` to the release, creating a draft if none exists. To build locally instead, run `just deb` and the `flatpak-builder` / `flatpak build-bundle` steps in `.github/workflows/release.yml`.
- Test `just install` on a clean prefix or VM.
- Confirm `wl-copy` clipboard persistence in a Wayland session.
- Confirm single-instance launch focuses the existing picker.
- Confirm preferences and recent emojis persist across restart.

## Nice to have

- Add a real screenshot at `screenshots/main.png`.
- Add the screenshot back to `com.lkbddh.Glyphie.metainfo.xml` once the URL is live.
- Publish a short install note that says native users need Rust, `just`, `wl-clipboard`, and an emoji font.
