# Release Checklist

Use this before posting or tagging a release.

## Required

- Commit `Cargo.lock` for reproducible native builds.
- Commit `cargo-sources.json` whenever the Flatpak manifest is advertised.
- Keep `LICENSE` and `THIRD_PARTY_NOTICES.md` in the release.
- Refresh `THIRD_PARTY_NOTICES.md` after dependency, bundled tool, embedded data, or icon changes.
- Run `just validate`.
- Run `cargo build --release`.
- Push the `v<version>` tag. The Release workflow builds `glyphie_<version>_amd64.deb` and `glyphie-<version>.flatpak` and stages them with `SHA256SUMS` on a draft release, creating the draft if needed. Write the notes, run the checks below against the draft's packages, then publish it. The workflow never replaces or deletes a release file; to re-run it, first remove the files from the draft.
- To build the packages locally instead:

  ```bash
  just build-release && just deb
  flatpak-builder --user --install-deps-from=flathub --force-clean --repo=repo build-dir com.lkbddh.Glyphie.yml
  flatpak build-bundle repo glyphie-<version>.flatpak com.lkbddh.Glyphie --runtime-repo=https://dl.flathub.org/repo/flathub.flatpakrepo
  ```
- Test `just install` on a clean prefix or VM.
- Confirm `wl-copy` clipboard persistence in a Wayland session.
- Confirm single-instance launch focuses the existing picker.
- Confirm preferences and recent emojis persist across restart.

## Nice to have

- Add a real screenshot at `screenshots/main.png`.
- Add the screenshot back to `com.lkbddh.Glyphie.metainfo.xml` once the URL is live.
- Publish a short install note that says native users need Rust, `just`, `wl-clipboard`, and an emoji font.
