name := 'glyphie'
export APPID := 'com.lkbddh.Glyphie'

rootdir := ''
prefix := '/usr'

base-dir := absolute_path(clean(rootdir / prefix))

cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
bin-src := cargo-target-dir / 'release' / name
bin-dst := base-dir / 'bin' / name

desktop := APPID + '.desktop'
desktop-dst := base-dir / 'share' / 'applications' / desktop

metainfo := APPID + '.metainfo.xml'
metainfo-dst := base-dir / 'share' / 'metainfo' / metainfo

icons-src := 'icons' / 'hicolor'
icons-dst := base-dir / 'share' / 'icons' / 'hicolor'
cosmic-icons-dst := base-dir / 'share' / 'icons' / 'Cosmic'
doc-dst := base-dir / 'share' / 'doc' / name

# Default recipe
default: build-release

# Runs `cargo clean`
clean:
    cargo clean

# Formats all Rust sources
fmt:
    cargo fmt --all

# Runs unit tests
test:
    cargo test

# Compiles with debug profile
build-debug *args:
    cargo build {{args}}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Runs a clippy check
check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

# Runs local validation checks
validate:
    cargo fmt --all -- --check
    cargo clippy --all-features -- -W clippy::pedantic
    cargo test
    desktop-file-validate {{desktop}}
    appstreamcli validate --no-net {{metainfo}}

# Run with debug logs
run *args:
    cargo build --release
    env RUST_LOG=glyphie=debug RUST_BACKTRACE=full {{bin-src}} {{args}}

# Installs files (run `just build-release` first, then `sudo just install`)
install:
    install -Dm0755 {{bin-src}} {{bin-dst}}
    install -Dm0644 {{desktop}} {{desktop-dst}}
    install -Dm0644 {{metainfo}} {{metainfo-dst}}
    for size in `ls {{icons-src}}`; do \
        for file in `ls {{icons-src}}/$size/apps/`; do \
            install -Dm0644 "{{icons-src}}/$size/apps/$file" "{{icons-dst}}/$size/apps/{{APPID}}.${file##*.}"; \
        done \
    done
    install -Dm0644 "{{icons-src}}/scalable/apps/glyphie.svg" "{{cosmic-icons-dst}}/scalable/apps/{{APPID}}.svg"
    install -Dm0644 README.md "{{doc-dst}}/README.md"
    install -Dm0644 LICENSE "{{doc-dst}}/LICENSE"
    install -Dm0644 THIRD_PARTY_NOTICES.md "{{doc-dst}}/THIRD_PARTY_NOTICES.md"

# Builds a .deb in target/deb from the release binary (run `just build-release` first)
deb:
    #!/usr/bin/env bash
    set -euo pipefail
    version=$(cargo pkgid | sed 's/.*[#@]//')
    arch=$(dpkg --print-architecture)
    pkg={{cargo-target-dir}}/deb/{{name}}_${version}_${arch}
    rm -rf "$pkg"
    just rootdir="$pkg" install
    # dpkg-shlibdeps only runs next to a debian/control, so give it a throwaway one.
    tmp=$(mktemp -d)
    trap 'rm -rf "$tmp"' EXIT
    mkdir "$tmp/debian"
    printf 'Source: {{name}}\n\nPackage: {{name}}\nArchitecture: any\n' > "$tmp/debian/control"
    shlibs=$(cd "$tmp" && dpkg-shlibdeps -O "$OLDPWD/$pkg/usr/bin/{{name}}" | sed 's/^shlibs:Depends=//')
    mkdir -p "$pkg/DEBIAN"
    cat > "$pkg/DEBIAN/control" <<EOF
    Package: {{name}}
    Version: ${version}
    Architecture: ${arch}
    Maintainer: lkbddh <204492431+lkbddh@users.noreply.github.com>
    Installed-Size: $(du -sk "$pkg/usr" | cut -f1)
    Depends: ${shlibs}, libwayland-client0, wl-clipboard, fonts-noto-color-emoji
    Recommends: libvulkan1 | libegl1
    Section: utils
    Priority: optional
    Homepage: https://github.com/lkbddh/glyphie
    Description: Emoji picker for COSMIC
     Fast libcosmic-native emoji picker with skin tone and gender support.
    EOF
    dpkg-deb --root-owner-group --build "$pkg" {{cargo-target-dir}}/deb/

# Uninstalls installed files
uninstall:
    rm -f {{bin-dst}}
    rm -f {{desktop-dst}}
    rm -f {{metainfo-dst}}
    for size in `ls {{icons-src}}`; do \
        rm -f "{{icons-dst}}/$size/apps/{{APPID}}."*; \
    done
    rm -f "{{cosmic-icons-dst}}/scalable/apps/{{APPID}}.svg"
    rm -f "{{doc-dst}}/README.md"
    rm -f "{{doc-dst}}/LICENSE"
    rm -f "{{doc-dst}}/THIRD_PARTY_NOTICES.md"
