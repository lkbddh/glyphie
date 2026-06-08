# Release Checklist

Use this before posting or tagging a release.

## Required

- Commit `Cargo.lock` for reproducible native builds.
- Commit `cargo-sources.json` whenever the Flatpak manifest is advertised.
- Keep `LICENSE` and `THIRD_PARTY_NOTICES.md` in the release.
- Refresh `THIRD_PARTY_NOTICES.md` after dependency, bundled tool, embedded data, or icon changes.
- Run `just validate`.
- Run `cargo build --release`.
- Test `just install` on a clean prefix or VM.
- Confirm `wl-copy` clipboard persistence in a Wayland session.
- Confirm single-instance launch focuses the existing picker.
- Confirm preferences and recent emojis persist across restart.

## Nice to have

- Add a real screenshot at `screenshots/main.png`.
- Add the screenshot back to `com.aldeastudio.Glyphie.metainfo.xml` once the URL is live.
- Publish a short install note that says native users need Rust, `just`, `wl-clipboard`, and an emoji font.
