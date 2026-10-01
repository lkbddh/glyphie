# Security policy

## Reporting a vulnerability

Report it privately through [Report a vulnerability](https://github.com/lkbddh/glyphie/security/advisories/new) on this repository. Do not open a public issue.

Include the Glyphie version, how you installed it, and the steps to reproduce. Fixes ship in a normal release, and the advisory credits you unless you ask otherwise.

## Supported versions

Only the latest release receives fixes.

## Scope

Glyphie runs locally, makes no network requests, and only writes its own preferences and recent emojis through `cosmic-config`. It copies text to the clipboard with `wl-copy`. Reports about those paths, the packaging (the `.deb`, the Flatpak permissions), or the dependency tree (`cargo audit`) are all in scope.
