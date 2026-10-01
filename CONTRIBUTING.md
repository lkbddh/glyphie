# Contributing to Glyphie

Thanks for helping. Glyphie is a small, fast emoji picker for COSMIC, so it takes changes that keep it small and fast. Read [What Glyphie is (and isn't)](README.md#what-glyphie-is-and-isnt) before proposing a feature.

If you are an AI coding agent, or you are directing one, read [AGENTS.md](AGENTS.md) as well. It has the same rules plus what an agent needs to work in this codebase.

## Reporting a bug

Open an issue with:

- the Glyphie version (Settings page in the app) and how you installed it: `.deb`, Flatpak, or from source
- your COSMIC and distribution versions
- what you did, what you expected, and what happened instead
- for crashes or wrong behaviour, the output of `RUST_LOG=glyphie=debug glyphie` (for the Flatpak: `RUST_LOG=glyphie=debug flatpak run com.lkbddh.Glyphie`)

## Suggesting a feature

Open an issue first and describe the problem rather than the solution. Glyphie has no online content, accounts, or telemetry, and it closes after copying; features that change that will not be merged.

## Setting up

You need Rust 1.93 or newer, [`just`](https://github.com/casey/just), and the build packages listed under [Requirements](README.md#requirements). Running the app needs a COSMIC or other Wayland session.

```bash
git clone https://github.com/lkbddh/glyphie.git
cd glyphie
just run        # release build, runs with debug logs
just validate   # what every pull request must pass
```

## Pull requests

`main` is protected: every change lands through a pull request, and nobody pushes to it directly.

1. Fork, branch from `main`, and keep one change per pull request.
2. Run `just validate`. It checks formatting, pedantic Clippy, the unit tests, and the desktop and AppStream metadata.
3. If you touched the UI, launch, clipboard, search, or keyboard handling, run the checks in [docs/smoke-test.md](docs/smoke-test.md) and attach a screenshot for visible changes.
4. In the description, say what was wrong, what you changed, and how you checked it.

Some files need extra steps:

- **Dependencies.** When `Cargo.lock` changes, regenerate `cargo-sources.json` with [`flatpak-cargo-generator.py`](https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo) and update the Rust table in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). Avoid new dependencies unless the change cannot be done without one.
- **Emoji data.** `data/emojis.json` is generated from Unicode data. Do not edit it by hand; open an issue if an emoji is wrong or missing.
- **Text.** User-facing strings live in `i18n/en/glyphie.ftl`. To add a translation, copy that file to `i18n/<language>/glyphie.ftl` and translate the values.
- **Icons.** UI icons in `assets/` come from [Phosphor Icons](https://phosphoricons.com/). Credit any new icon source in `THIRD_PARTY_NOTICES.md`.

## AI-assisted contributions

You may use AI tools. The person who opens the pull request is responsible for it: you have read every line, run `just validate` and the relevant smoke tests yourself, and can answer review questions without asking the tool. Pull requests opened by an agent with no person behind them will be closed.

## License

Glyphie is licensed under [GPL-3.0-or-later](LICENSE). By contributing, you agree that your contribution is licensed the same way.
