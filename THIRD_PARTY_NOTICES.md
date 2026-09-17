# Third-Party Notices

This file records third-party open-source software, embedded data, and assets used by Glyphie.
Glyphie keeps these credits in the repository and packaged docs instead of showing donation or personal support links in the app UI.

For source installs, the Rust dependency set is reproducible from `Cargo.lock`.
For binary or Flatpak distribution, ship this file together with `LICENSE`.

## Embedded Data And Assets

| Project | Used for | License | Source |
|---|---|---|---|
| Unicode CLDR / Unicode Emoji data | Emoji names, categories, and search keywords in `data/emojis.json` | Unicode-3.0 | https://unicode.org/ |
| Phosphor Icons | SVG category and action icons under `assets/` | MIT | https://phosphoricons.com/ |
| wl-clipboard | `wl-copy` / `wl-paste` bundled by the Flatpak manifest | GPL-3.0-or-later | https://github.com/bugaevc/wl-clipboard |

## Rust Dependencies

Generated from `cargo metadata --format-version 1` for the current lockfile. Some upstream packages do not publish a Cargo `license` field; where a repository-level license file was present, this inventory uses that repository license.

| Package | Version | License | Source |
|---|---:|---|---|
| `ab_glyph` | `0.2.32` | Apache-2.0 | https://github.com/alexheretic/ab-glyph |
| `ab_glyph_rasterizer` | `0.1.10` | Apache-2.0 | https://github.com/alexheretic/ab-glyph |
| `accesskit` | `0.22.0` | MIT OR Apache-2.0 | https://github.com/AccessKit/accesskit |
| `accesskit_atspi_common` | `0.15.0` | MIT OR Apache-2.0 | https://github.com/AccessKit/accesskit |
| `accesskit_consumer` | `0.32.0` | MIT OR Apache-2.0 | https://github.com/AccessKit/accesskit |
| `accesskit_macos` | `0.23.0` | MIT OR Apache-2.0 | https://github.com/AccessKit/accesskit |
| `accesskit_unix` | `0.18.0` | MIT OR Apache-2.0 | https://github.com/AccessKit/accesskit |
| `accesskit_windows` | `0.30.0` | MIT OR Apache-2.0 | https://github.com/AccessKit/accesskit |
| `accesskit_winit` | `0.30.0` | Apache-2.0 | https://github.com/AccessKit/accesskit |
| `adler2` | `2.0.1` | 0BSD OR MIT OR Apache-2.0 | https://github.com/oyvindln/adler2 |
| `ahash` | `0.8.12` | MIT OR Apache-2.0 | https://github.com/tkaitchuck/ahash |
| `aho-corasick` | `1.1.5` | Unlicense OR MIT | https://github.com/BurntSushi/aho-corasick |
| `aliasable` | `0.1.3` | MIT | https://github.com/avitex/rust-aliasable |
| `allocator-api2` | `0.2.21` | MIT OR Apache-2.0 | https://github.com/zakarumych/allocator-api2 |
| `almost` | `0.2.0` | CC0-1.0 | https://github.com/thomcc/almost |
| `android-activity` | `0.6.1` | MIT OR Apache-2.0 | https://github.com/rust-mobile/android-activity |
| `android-properties` | `0.2.2` | MIT | https://github.com/miklelappo/android-properties |
| `android_system_properties` | `0.1.6` | MIT OR Apache-2.0 | https://github.com/nical/android_system_properties |
| `anstream` | `1.0.0` | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle.git |
| `anstyle` | `1.0.14` | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle.git |
| `anstyle-parse` | `1.0.0` | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle.git |
| `anstyle-query` | `1.1.5` | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle.git |
| `anstyle-wincon` | `3.0.11` | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle.git |
| `apply` | `0.3.0` | Unlicense | https://github.com/burtonageo/apply |
| `approx` | `0.5.1` | Apache-2.0 | https://github.com/brendanzab/approx |
| `arc-swap` | `1.9.2` | MIT OR Apache-2.0 | https://github.com/vorner/arc-swap |
| `arrayref` | `0.3.9` | BSD-2-Clause | https://github.com/droundy/arrayref |
| `arrayvec` | `0.7.8` | MIT OR Apache-2.0 | https://github.com/bluss/arrayvec |
| `as-raw-xcb-connection` | `1.0.1` | MIT OR Apache-2.0 | https://github.com/psychon/as-raw-xcb-connection |
| `ash` | `0.38.0+1.3.281` | MIT OR Apache-2.0 | https://github.com/ash-rs/ash |
| `ashpd` | `0.11.1` | MIT | https://github.com/bilelmoussaoui/ashpd |
| `ashpd` | `0.12.3` | MIT | https://github.com/bilelmoussaoui/ashpd |
| `async-broadcast` | `0.7.2` | MIT OR Apache-2.0 | https://github.com/smol-rs/async-broadcast |
| `async-channel` | `2.5.0` | Apache-2.0 OR MIT | https://github.com/smol-rs/async-channel |
| `async-executor` | `1.14.0` | Apache-2.0 OR MIT | https://github.com/smol-rs/async-executor |
| `async-io` | `2.6.0` | Apache-2.0 OR MIT | https://github.com/smol-rs/async-io |
| `async-lock` | `3.4.2` | Apache-2.0 OR MIT | https://github.com/smol-rs/async-lock |
| `async-process` | `2.5.0` | Apache-2.0 OR MIT | https://github.com/smol-rs/async-process |
| `async-recursion` | `1.1.1` | MIT OR Apache-2.0 | https://github.com/dcchut/async-recursion |
| `async-signal` | `0.2.14` | Apache-2.0 OR MIT | https://github.com/smol-rs/async-signal |
| `async-task` | `4.7.1` | Apache-2.0 OR MIT | https://github.com/smol-rs/async-task |
| `async-trait` | `0.1.92` | MIT OR Apache-2.0 | https://github.com/dtolnay/async-trait |
| `atomic-waker` | `1.1.2` | Apache-2.0 OR MIT | https://github.com/smol-rs/atomic-waker |
| `atomicwrites` | `0.4.2` | MIT | https://github.com/untitaker/rust-atomicwrites |
| `atspi` | `0.29.0` | Apache-2.0 OR MIT | https://github.com/odilia-app/atspi |
| `atspi-common` | `0.13.0` | Apache-2.0 OR MIT | https://github.com/odilia-app/atspi |
| `atspi-proxies` | `0.13.0` | Apache-2.0 OR MIT | https://github.com/odilia-app/atspi |
| `auto_enums` | `0.8.10` | Apache-2.0 OR MIT | https://github.com/taiki-e/auto_enums |
| `autocfg` | `1.5.1` | Apache-2.0 OR MIT | https://github.com/cuviper/autocfg |
| `base64` | `0.22.1` | MIT OR Apache-2.0 | https://github.com/marshallpierce/rust-base64 |
| `basic-toml` | `0.1.10` | MIT OR Apache-2.0 | https://github.com/dtolnay/basic-toml |
| `bit-set` | `0.8.0` | Apache-2.0 OR MIT | https://github.com/contain-rs/bit-set |
| `bit-vec` | `0.8.0` | Apache-2.0 OR MIT | https://github.com/contain-rs/bit-vec |
| `bitflags` | `1.3.2` | MIT/Apache-2.0 | https://github.com/bitflags/bitflags |
| `bitflags` | `2.13.2` | MIT OR Apache-2.0 | https://github.com/bitflags/bitflags |
| `block` | `0.1.6` | MIT | http://github.com/SSheldon/rust-block |
| `block-buffer` | `0.12.1` | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| `block2` | `0.5.1` | MIT | https://github.com/madsmtm/objc2 |
| `block2` | `0.6.2` | MIT | https://github.com/madsmtm/objc2 |
| `blocking` | `1.7.0` | Apache-2.0 OR MIT | https://github.com/smol-rs/blocking |
| `borsh` | `1.8.1` | MIT OR Apache-2.0 | https://github.com/near/borsh-rs |
| `bstr` | `1.13.1` | MIT OR Apache-2.0 | https://github.com/BurntSushi/bstr |
| `btoi` | `0.5.0` | MIT OR Apache-2.0 | https://github.com/niklasf/rust-btoi |
| `build_helpers` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `bumpalo` | `3.20.3` | MIT OR Apache-2.0 | https://github.com/fitzgen/bumpalo |
| `by_address` | `1.2.1` | MIT OR Apache-2.0 | https://github.com/mbrubeck/by_address |
| `bytemuck` | `1.25.2` | Zlib OR Apache-2.0 OR MIT | https://github.com/Lokathor/bytemuck |
| `bytemuck_derive` | `1.12.1` | Zlib OR Apache-2.0 OR MIT | https://github.com/Lokathor/bytemuck |
| `byteorder-lite` | `0.1.0` | Unlicense OR MIT | https://github.com/image-rs/byteorder-lite |
| `bytes` | `1.12.1` | MIT | https://github.com/tokio-rs/bytes |
| `calloop` | `0.14.4` | MIT | https://github.com/Smithay/calloop |
| `calloop-wayland-source` | `0.4.1` | MIT | https://github.com/smithay/calloop-wayland-source |
| `cc` | `1.4.6` | MIT OR Apache-2.0 | https://github.com/rust-lang/cc-rs |
| `cfg-if` | `1.0.5` | MIT OR Apache-2.0 | https://github.com/rust-lang/cfg-if |
| `cfg_aliases` | `0.2.2` | MIT | https://github.com/katharostech/cfg_aliases |
| `clipboard-win` | `5.4.1` | BSL-1.0 | https://github.com/DoumanAsh/clipboard-win |
| `clipboard_macos` | `0.1.0` | Apache-2.0 | https://github.com/hecrj/window_clipboard |
| `clipboard_wayland` | `0.2.2` | Apache-2.0 | https://github.com/hecrj/window_clipboard |
| `clipboard_x11` | `0.4.2` | MIT | https://github.com/hecrj/window_clipboard |
| `cocoa` | `0.25.0` | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| `cocoa-foundation` | `0.1.2` | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| `codespan-reporting` | `0.12.0` | Apache-2.0 | https://github.com/brendanzab/codespan |
| `color_quant` | `1.1.0` | MIT | https://github.com/image-rs/color_quant.git |
| `colorchoice` | `1.0.5` | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle.git |
| `combine` | `4.6.8` | MIT | https://github.com/Marwes/combine |
| `concurrent-queue` | `2.5.0` | Apache-2.0 OR MIT | https://github.com/smol-rs/concurrent-queue |
| `configparser` | `3.3.0` | MIT OR LGPL-3.0-or-later | https://github.com/QEDK/configparser-rs |
| `const-oid` | `0.10.2` | Apache-2.0 OR MIT | https://github.com/RustCrypto/formats |
| `core-foundation` | `0.9.4` | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| `core-foundation` | `0.10.1` | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| `core-foundation-sys` | `0.8.7` | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| `core-graphics` | `0.23.2` | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| `core-graphics-types` | `0.1.3` | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| `core-graphics-types` | `0.2.0` | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| `core_maths` | `0.1.1` | MIT | https://github.com/robertbastian/core_maths |
| `cosmic-client-toolkit` | `0.2.0` | MIT | https://github.com/pop-os/cosmic-protocols |
| `cosmic-config` | `1.0.0` | MPL-2.0 | git+https://github.com/pop-os/libcosmic?rev=87ab8179e1bd9880239c340855ae8862034bd0e8#87ab8179e1bd9880239c340855ae8862034bd0e8 |
| `cosmic-config-derive` | `1.0.0` | MPL-2.0 | git+https://github.com/pop-os/libcosmic?rev=87ab8179e1bd9880239c340855ae8862034bd0e8#87ab8179e1bd9880239c340855ae8862034bd0e8 |
| `cosmic-freedesktop-icons` | `0.4.0` | MIT | https://github.com/pop-os/freedesktop-icons |
| `cosmic-protocols` | `0.2.0` | MIT | https://github.com/pop-os/cosmic-protocols |
| `cosmic-settings-daemon` | `0.1.0` | MPL-2.0 | git+https://github.com/pop-os/dbus-settings-bindings#eed01dd3609e90e3c8cd043656734c500956c793 |
| `cosmic-text` | `0.19.0` | MIT OR Apache-2.0 | https://github.com/pop-os/cosmic-text |
| `cosmic-theme` | `1.0.0` | MPL-2.0 | git+https://github.com/pop-os/libcosmic?rev=87ab8179e1bd9880239c340855ae8862034bd0e8#87ab8179e1bd9880239c340855ae8862034bd0e8 |
| `cpufeatures` | `0.3.1` | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| `crc32fast` | `1.5.2` | MIT OR Apache-2.0 | https://github.com/srijs/rust-crc32fast |
| `crossbeam-utils` | `0.8.23` | MIT OR Apache-2.0 | https://github.com/crossbeam-rs/crossbeam |
| `crunchy` | `0.2.4` | MIT | https://github.com/eira-fransham/crunchy |
| `cryoglyph` | `0.1.0` | MIT OR Apache-2.0 OR Zlib | https://github.com/iced-rs/cryoglyph |
| `crypto-common` | `0.2.2` | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| `css-color` | `0.2.8` | MIT OR Apache-2.0 | https://github.com/kalcutter/rust-css-color |
| `csscolorparser` | `0.8.4` | MIT OR Apache-2.0 | https://github.com/mazznoer/csscolorparser-rs |
| `ctor` | `0.10.1` | Apache-2.0 OR MIT | https://github.com/mmastrac/rust-ctor |
| `cursor-icon` | `1.2.0` | MIT OR Apache-2.0 OR Zlib | https://github.com/rust-windowing/cursor-icon |
| `darling` | `0.21.3` | MIT | https://github.com/TedDriggs/darling |
| `darling_core` | `0.21.3` | MIT | https://github.com/TedDriggs/darling |
| `darling_macro` | `0.21.3` | MIT | https://github.com/TedDriggs/darling |
| `data-url` | `0.3.2` | MIT OR Apache-2.0 | https://github.com/servo/rust-url |
| `defmt` | `1.1.1` | MIT OR Apache-2.0 | https://github.com/knurling-rs/defmt |
| `defmt-macros` | `1.1.1` | MIT OR Apache-2.0 | https://github.com/knurling-rs/defmt |
| `defmt-parser` | `1.0.0` | MIT OR Apache-2.0 | https://github.com/knurling-rs/defmt |
| `derive_setters` | `0.1.9` | MIT/Apache-2.0 | https://github.com/Lymia/derive_setters |
| `derive_utils` | `0.16.0` | Apache-2.0 OR MIT | https://github.com/taiki-e/derive_utils |
| `digest` | `0.11.3` | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| `dirs` | `6.0.0` | MIT OR Apache-2.0 | https://github.com/soc/dirs-rs |
| `dirs-sys` | `0.5.0` | MIT OR Apache-2.0 | https://github.com/dirs-dev/dirs-sys-rs |
| `dispatch2` | `0.3.1` | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| `displaydoc` | `0.2.7` | MIT OR Apache-2.0 | https://github.com/yaahc/displaydoc |
| `dlib` | `0.5.3` | MIT | https://github.com/elinorbgr/dlib |
| `dnd` | `0.1.0` | MIT | git+https://github.com/pop-os/window_clipboard.git?tag=sctk-0.20#f68595ee0e62fbd6589f4709b5aaa5c3c7ea5f6c |
| `document-features` | `0.2.12` | MIT OR Apache-2.0 | https://github.com/slint-ui/document-features |
| `downcast-rs` | `1.2.1` | MIT/Apache-2.0 | https://github.com/marcianx/downcast-rs |
| `dpi` | `0.1.2` | Apache-2.0 AND MIT | https://github.com/rust-windowing/winit |
| `drm` | `0.11.1` | MIT | https://github.com/Smithay/drm-rs |
| `drm-ffi` | `0.7.1` | MIT | https://github.com/Smithay/drm-rs |
| `drm-fourcc` | `2.2.0` | MIT | https://github.com/danielzfranklin/drm-fourcc-rs |
| `drm-sys` | `0.6.1` | MIT | https://github.com/Smithay/drm-rs |
| `dtor` | `0.8.1` | Apache-2.0 OR MIT | https://github.com/mmastrac/rust-ctor |
| `endi` | `1.1.1` | MIT | https://github.com/zeenix/endi |
| `enumflags2` | `0.7.12` | MIT OR Apache-2.0 | https://github.com/meithecatte/enumflags2 |
| `enumflags2_derive` | `0.7.12` | MIT OR Apache-2.0 | https://github.com/meithecatte/enumflags2 |
| `env_filter` | `2.0.0` | MIT OR Apache-2.0 | https://github.com/rust-cli/env_logger |
| `env_logger` | `0.11.11` | MIT OR Apache-2.0 | https://github.com/rust-cli/env_logger |
| `equivalent` | `1.0.2` | Apache-2.0 OR MIT | https://github.com/indexmap-rs/equivalent |
| `errno` | `0.3.14` | MIT OR Apache-2.0 | https://github.com/lambda-fairy/rust-errno |
| `error-code` | `3.4.0` | BSL-1.0 | https://github.com/DoumanAsh/error-code |
| `etagere` | `0.2.15` | MIT/Apache-2.0 | https://github.com/nical/etagere |
| `euclid` | `0.22.14` | MIT OR Apache-2.0 | https://github.com/servo/euclid |
| `event-listener` | `5.4.2` | Apache-2.0 OR MIT | https://github.com/smol-rs/event-listener |
| `event-listener-strategy` | `0.5.4` | Apache-2.0 OR MIT | https://github.com/smol-rs/event-listener-strategy |
| `fastrand` | `2.5.0` | Apache-2.0 OR MIT | https://github.com/smol-rs/fastrand |
| `fdeflate` | `0.3.7` | MIT OR Apache-2.0 | https://github.com/image-rs/fdeflate |
| `find-crate` | `0.6.3` | Apache-2.0 OR MIT | https://github.com/taiki-e/find-crate |
| `find-msvc-tools` | `0.1.12` | MIT OR Apache-2.0 | https://github.com/rust-lang/cc-rs |
| `flate2` | `1.1.10` | MIT OR Apache-2.0 | https://github.com/rust-lang/flate2-rs |
| `float-cmp` | `0.9.0` | MIT | https://github.com/mikedilger/float-cmp |
| `float-cmp` | `0.10.0` | MIT | https://github.com/mikedilger/float-cmp |
| `float_next_after` | `1.0.0` | MIT | https://gitlab.com/bronsonbdevost/next_afterf |
| `fluent` | `0.17.0` | Apache-2.0 OR MIT | https://github.com/projectfluent/fluent-rs |
| `fluent-bundle` | `0.16.0` | Apache-2.0 OR MIT | https://github.com/projectfluent/fluent-rs |
| `fluent-langneg` | `0.13.1` | Apache-2.0 OR MIT | https://github.com/projectfluent/fluent-langneg-rs |
| `fluent-syntax` | `0.12.0` | Apache-2.0 OR MIT | https://github.com/projectfluent/fluent-rs |
| `fnv` | `1.0.7` | Apache-2.0 / MIT | https://github.com/servo/rust-fnv |
| `foldhash` | `0.1.5` | Zlib | https://github.com/orlp/foldhash |
| `foldhash` | `0.2.0` | Zlib | https://github.com/orlp/foldhash |
| `font-types` | `0.11.3` | MIT OR Apache-2.0 | https://github.com/googlefonts/fontations |
| `font-types` | `0.12.5` | MIT OR Apache-2.0 | https://github.com/googlefonts/fontations |
| `fontconfig-parser` | `0.5.8` | MIT | https://github.com/Riey/fontconfig-parser |
| `fontdb` | `0.23.0` | MIT | https://github.com/RazrFalcon/fontdb |
| `foreign-types` | `0.5.0` | MIT/Apache-2.0 | https://github.com/sfackler/foreign-types |
| `foreign-types-macros` | `0.2.4` | MIT/Apache-2.0 | https://github.com/sfackler/foreign-types |
| `foreign-types-shared` | `0.3.1` | MIT/Apache-2.0 | https://github.com/sfackler/foreign-types |
| `form_urlencoded` | `1.2.2` | MIT OR Apache-2.0 | https://github.com/servo/rust-url |
| `fsevent-sys` | `4.1.0` | MIT | https://github.com/octplane/fsevent-rust/tree/master/fsevent-sys |
| `futures` | `0.3.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-channel` | `0.3.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-core` | `0.3.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-executor` | `0.3.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-io` | `0.3.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-lite` | `2.6.1` | Apache-2.0 OR MIT | https://github.com/smol-rs/futures-lite |
| `futures-macro` | `0.3.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-sink` | `0.3.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-task` | `0.3.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-util` | `0.3.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `gethostname` | `1.1.0` | Apache-2.0 | https://codeberg.org/swsnr/gethostname.rs.git |
| `getrandom` | `0.2.17` | MIT OR Apache-2.0 | https://github.com/rust-random/getrandom |
| `getrandom` | `0.3.4` | MIT OR Apache-2.0 | https://github.com/rust-random/getrandom |
| `getrandom` | `0.4.3` | MIT OR Apache-2.0 | https://github.com/rust-random/getrandom |
| `gif` | `0.13.3` | MIT OR Apache-2.0 | https://github.com/image-rs/image-gif |
| `gl_generator` | `0.14.0` | Apache-2.0 | https://github.com/brendanzab/gl-rs/ |
| `glam` | `0.25.0` | MIT OR Apache-2.0 | https://github.com/bitshifter/glam-rs |
| `glow` | `0.16.0` | MIT OR Apache-2.0 OR Zlib | https://github.com/grovesNL/glow |
| `glutin_wgl_sys` | `0.6.1` | Apache-2.0 | https://github.com/rust-windowing/glutin |
| `gpu-allocator` | `0.28.0` | MIT OR Apache-2.0 | https://github.com/Traverse-Research/gpu-allocator |
| `gpu-descriptor` | `0.3.2` | MIT OR Apache-2.0 | https://github.com/zakarumych/gpu-descriptor |
| `gpu-descriptor-types` | `0.2.0` | MIT OR Apache-2.0 | https://github.com/zakarumych/gpu-descriptor |
| `grid` | `1.0.1` | MIT | https://github.com/becheran/grid |
| `guillotiere` | `0.6.2` | MIT/Apache-2.0 | https://github.com/nical/guillotiere |
| `half` | `2.7.1` | MIT OR Apache-2.0 | https://github.com/VoidStarKat/half-rs |
| `harfrust` | `0.5.2` | MIT | https://github.com/harfbuzz/harfrust |
| `hashbrown` | `0.15.5` | MIT OR Apache-2.0 | https://github.com/rust-lang/hashbrown |
| `hashbrown` | `0.16.1` | MIT OR Apache-2.0 | https://github.com/rust-lang/hashbrown |
| `hashbrown` | `0.17.1` | MIT OR Apache-2.0 | https://github.com/rust-lang/hashbrown |
| `heck` | `0.4.1` | MIT OR Apache-2.0 | https://github.com/withoutboats/heck |
| `hermit-abi` | `0.5.3` | MIT OR Apache-2.0 | https://github.com/hermit-os/hermit-rs |
| `hex` | `0.4.3` | MIT OR Apache-2.0 | https://github.com/KokaKiwi/rust-hex |
| `hex_color` | `3.0.0` | MIT OR Apache-2.0 | https://github.com/seancroach/hex_color |
| `hexf-parse` | `0.2.1` | CC0-1.0 | https://github.com/lifthrasiir/hexf |
| `hybrid-array` | `0.4.15` | MIT OR Apache-2.0 | https://github.com/RustCrypto/hybrid-array |
| `i18n-config` | `0.4.8` | MIT | https://github.com/kellpossible/cargo-i18n/tree/master/i18n-config |
| `i18n-embed` | `0.16.0` | MIT | https://github.com/kellpossible/cargo-i18n/tree/master/i18n-embed |
| `i18n-embed-fl` | `0.10.1` | MIT | https://github.com/kellpossible/cargo-i18n/tree/master/i18n-embed-fl |
| `i18n-embed-impl` | `0.8.4` | MIT | https://github.com/kellpossible/cargo-i18n/tree/master/i18n-embed |
| `iced` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `iced_accessibility` | `0.1.0` | MPL-2.0 | git+https://github.com/pop-os/libcosmic?rev=87ab8179e1bd9880239c340855ae8862034bd0e8#87ab8179e1bd9880239c340855ae8862034bd0e8 |
| `iced_core` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `iced_debug` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `iced_futures` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `iced_graphics` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `iced_program` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `iced_renderer` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `iced_runtime` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `iced_tiny_skia` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `iced_wgpu` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `iced_widget` | `0.14.2` | MIT | https://github.com/iced-rs/iced |
| `iced_winit` | `0.14.0` | MIT | https://github.com/iced-rs/iced |
| `icu_collections` | `2.3.0` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `icu_locale_core` | `2.3.0` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `icu_normalizer` | `2.3.0` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `icu_normalizer_data` | `2.3.0` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `icu_properties` | `2.3.0` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `icu_properties_data` | `2.3.0` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `icu_provider` | `2.3.1` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `ident_case` | `1.0.1` | MIT/Apache-2.0 | https://github.com/TedDriggs/ident_case |
| `idna` | `1.1.0` | MIT OR Apache-2.0 | https://github.com/servo/rust-url/ |
| `idna_adapter` | `1.2.2` | Apache-2.0 OR MIT | https://github.com/hsivonen/idna_adapter |
| `image` | `0.25.10` | MIT OR Apache-2.0 | https://github.com/image-rs/image |
| `image-webp` | `0.2.4` | MIT OR Apache-2.0 | https://github.com/image-rs/image-webp |
| `imagesize` | `0.13.0` | MIT | https://github.com/Roughsketch/imagesize |
| `indexmap` | `2.14.2` | Apache-2.0 OR MIT | https://github.com/indexmap-rs/indexmap |
| `inotify` | `0.11.5` | ISC | https://github.com/hannobraun/inotify-rs |
| `inotify-sys` | `0.1.8` | ISC | https://github.com/hannobraun/inotify-sys |
| `intl-memoizer` | `0.5.3` | Apache-2.0 OR MIT | https://github.com/projectfluent/fluent-rs |
| `intl_pluralrules` | `7.0.2` | Apache-2.0/MIT | https://github.com/zbraniecki/pluralrules |
| `is_terminal_polyfill` | `1.70.2` | MIT OR Apache-2.0 | https://github.com/polyfill-rs/is_terminal_polyfill |
| `itoa` | `1.0.18` | MIT OR Apache-2.0 | https://github.com/dtolnay/itoa |
| `jiff` | `0.2.37` | Unlicense OR MIT | https://github.com/BurntSushi/jiff |
| `jiff-core` | `0.1.1` | Unlicense OR MIT | https://github.com/BurntSushi/jiff |
| `jiff-static` | `0.2.37` | Unlicense OR MIT | https://github.com/BurntSushi/jiff |
| `jiff-tzdb` | `0.1.8` | Unlicense OR MIT | https://github.com/BurntSushi/jiff |
| `jiff-tzdb-platform` | `0.1.3` | Unlicense OR MIT | https://github.com/BurntSushi/jiff |
| `jni` | `0.22.4` | MIT OR Apache-2.0 | https://github.com/jni-rs/jni-rs |
| `jni-macros` | `0.22.4` | MIT OR Apache-2.0 | https://github.com/jni-rs/jni-rs |
| `jni-sys` | `0.3.1` | MIT OR Apache-2.0 | https://github.com/jni-rs/jni-sys |
| `jni-sys` | `0.4.1` | MIT OR Apache-2.0 | https://github.com/jni-rs/jni-sys |
| `jni-sys-macros` | `0.4.1` | MIT OR Apache-2.0 | https://github.com/jni-rs/jni-sys |
| `jobserver` | `0.1.35` | MIT OR Apache-2.0 | https://github.com/rust-lang/jobserver-rs |
| `js-sys` | `0.3.105` | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/js-sys |
| `kamadak-exif` | `0.6.1` | BSD-2-Clause | https://github.com/kamadak/exif-rs |
| `keyboard-types` | `0.8.3` | MIT OR Apache-2.0 | https://github.com/rust-windowing/keyboard-types |
| `khronos-egl` | `6.0.0` | MIT/Apache-2.0 | https://github.com/timothee-haudebourg/khronos-egl |
| `khronos_api` | `3.1.0` | Apache-2.0 | https://github.com/brendanzab/gl-rs/ |
| `known-folders` | `1.4.2` | Apache-2.0 OR MIT | https://github.com/artichoke/known-folders-rs |
| `kqueue` | `1.2.1` | MIT | https://gitlab.com/rust-kqueue/rust-kqueue |
| `kqueue-sys` | `1.1.2` | MIT | https://gitlab.com/rust-kqueue/rust-kqueue-sys |
| `kurbo` | `0.10.4` | MIT OR Apache-2.0 | https://github.com/linebender/kurbo |
| `kurbo` | `0.11.3` | Apache-2.0 OR MIT | https://github.com/linebender/kurbo |
| `libc` | `0.2.189` | MIT OR Apache-2.0 | https://github.com/rust-lang/libc |
| `libcosmic` | `1.0.0` | MPL-2.0 | git+https://github.com/pop-os/libcosmic?rev=87ab8179e1bd9880239c340855ae8862034bd0e8#87ab8179e1bd9880239c340855ae8862034bd0e8 |
| `libloading` | `0.8.9` | ISC | https://github.com/nagisa/rust_libloading/ |
| `libm` | `0.2.16` | MIT | https://github.com/rust-lang/compiler-builtins |
| `libredox` | `0.1.24` | MIT | https://gitlab.redox-os.org/redox-os/libredox.git |
| `lilt` | `0.8.2` | MIT | https://github.com/cyypherus/lilt |
| `linebender_resource_handle` | `0.1.1` | Apache-2.0 OR MIT | https://github.com/linebender/raw_resource_handle |
| `linux-raw-sys` | `0.4.15` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/sunfishcode/linux-raw-sys |
| `linux-raw-sys` | `0.6.5` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/sunfishcode/linux-raw-sys |
| `linux-raw-sys` | `0.12.1` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/sunfishcode/linux-raw-sys |
| `litemap` | `0.8.3` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `litrs` | `1.0.0` | MIT OR Apache-2.0 | https://github.com/LukasKalbertodt/litrs |
| `lock_api` | `0.4.14` | MIT OR Apache-2.0 | https://github.com/Amanieu/parking_lot |
| `log` | `0.4.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/log |
| `lru` | `0.16.4` | MIT | https://github.com/jeromefroe/lru-rs.git |
| `lyon` | `1.0.19` | MIT OR Apache-2.0 | https://github.com/nical/lyon |
| `lyon_algorithms` | `1.0.21` | MIT OR Apache-2.0 | https://github.com/nical/lyon |
| `lyon_geom` | `1.0.19` | MIT OR Apache-2.0 | https://github.com/nical/lyon |
| `lyon_path` | `1.0.19` | MIT OR Apache-2.0 | https://github.com/nical/lyon |
| `lyon_tessellation` | `1.0.22` | MIT OR Apache-2.0 | https://github.com/nical/lyon |
| `malloc_buf` | `0.0.6` | MIT | https://github.com/SSheldon/malloc_buf |
| `memchr` | `2.8.3` | Unlicense OR MIT | https://github.com/BurntSushi/memchr |
| `memmap2` | `0.8.0` | MIT OR Apache-2.0 | https://github.com/RazrFalcon/memmap2-rs |
| `memmap2` | `0.9.11` | MIT OR Apache-2.0 | https://github.com/RazrFalcon/memmap2-rs |
| `memoffset` | `0.9.1` | MIT | https://github.com/Gilnaa/memoffset |
| `metal` | `0.33.0` | MIT OR Apache-2.0 | https://github.com/gfx-rs/metal-rs |
| `mime` | `0.1.0` | MIT | git+https://github.com/pop-os/window_clipboard.git?tag=sctk-0.20#f68595ee0e62fbd6589f4709b5aaa5c3c7ea5f6c |
| `mime` | `0.3.17` | MIT OR Apache-2.0 | https://github.com/hyperium/mime |
| `mime_guess` | `2.0.5` | MIT | https://github.com/abonander/mime_guess |
| `miniz_oxide` | `0.8.9` | MIT OR Zlib OR Apache-2.0 | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide |
| `miniz_oxide` | `0.9.1` | MIT OR Zlib OR Apache-2.0 | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide |
| `mio` | `1.2.3` | MIT | https://github.com/tokio-rs/mio |
| `moxcms` | `0.8.1` | BSD-3-Clause OR Apache-2.0 | https://github.com/awxkee/moxcms.git |
| `mutate_once` | `0.1.2` | BSD-2-Clause | https://github.com/kamadak/mutate_once-rs |
| `naga` | `28.0.0` | MIT OR Apache-2.0 | https://github.com/gfx-rs/wgpu |
| `ndk` | `0.9.0` | MIT OR Apache-2.0 | https://github.com/rust-mobile/ndk |
| `ndk-context` | `0.1.1` | MIT OR Apache-2.0 | https://github.com/rust-windowing/android-ndk-rs |
| `ndk-sys` | `0.6.0+11769913` | MIT OR Apache-2.0 | https://github.com/rust-mobile/ndk |
| `notify` | `8.2.0` | CC0-1.0 | https://github.com/notify-rs/notify.git |
| `notify-types` | `2.1.0` | MIT OR Apache-2.0 | https://github.com/notify-rs/notify.git |
| `num-traits` | `0.2.19` | MIT OR Apache-2.0 | https://github.com/rust-num/num-traits |
| `num_enum` | `0.7.6` | BSD-3-Clause OR MIT OR Apache-2.0 | https://github.com/illicitonion/num_enum |
| `num_enum_derive` | `0.7.6` | BSD-3-Clause OR MIT OR Apache-2.0 | https://github.com/illicitonion/num_enum |
| `objc` | `0.2.7` | MIT | http://github.com/SSheldon/rust-objc |
| `objc-foundation` | `0.1.1` | MIT | http://github.com/SSheldon/rust-objc-foundation |
| `objc-sys` | `0.3.5` | MIT | https://github.com/madsmtm/objc2 |
| `objc2` | `0.5.2` | MIT | https://github.com/madsmtm/objc2 |
| `objc2` | `0.6.4` | MIT | https://github.com/madsmtm/objc2 |
| `objc2-app-kit` | `0.2.2` | MIT | https://github.com/madsmtm/objc2 |
| `objc2-app-kit` | `0.3.2` | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| `objc2-core-data` | `0.2.2` | MIT | https://github.com/madsmtm/objc2 |
| `objc2-core-foundation` | `0.3.2` | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| `objc2-core-graphics` | `0.3.2` | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| `objc2-core-image` | `0.2.2` | MIT | https://github.com/madsmtm/objc2 |
| `objc2-core-video` | `0.3.2` | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| `objc2-encode` | `4.1.0` | MIT | https://github.com/madsmtm/objc2 |
| `objc2-foundation` | `0.2.2` | MIT | https://github.com/madsmtm/objc2 |
| `objc2-foundation` | `0.3.2` | MIT | https://github.com/madsmtm/objc2 |
| `objc2-metal` | `0.2.2` | MIT | https://github.com/madsmtm/objc2 |
| `objc2-quartz-core` | `0.2.2` | MIT | https://github.com/madsmtm/objc2 |
| `objc2-ui-kit` | `0.3.2` | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| `objc_id` | `0.1.1` | MIT | http://github.com/SSheldon/rust-objc-id |
| `once_cell` | `1.21.4` | MIT OR Apache-2.0 | https://github.com/matklad/once_cell |
| `once_cell_polyfill` | `1.70.2` | MIT OR Apache-2.0 | https://github.com/polyfill-rs/once_cell_polyfill |
| `option-ext` | `0.2.0` | MPL-2.0 | https://github.com/soc/option-ext.git |
| `orbclient` | `0.3.55` | MIT | https://gitlab.redox-os.org/redox-os/orbclient |
| `ordered-float` | `5.5.0` | MIT | https://github.com/reem/rust-ordered-float |
| `ordered-stream` | `0.2.0` | MIT OR Apache-2.0 | https://github.com/danieldg/ordered-stream |
| `ouroboros` | `0.18.5` | MIT OR Apache-2.0 | https://github.com/someguynamedjosh/ouroboros |
| `ouroboros_macro` | `0.18.5` | MIT OR Apache-2.0 | https://github.com/someguynamedjosh/ouroboros |
| `owned_ttf_parser` | `0.25.1` | Apache-2.0 | https://github.com/alexheretic/owned-ttf-parser |
| `palette` | `0.7.7` | MIT OR Apache-2.0 | https://github.com/Ogeon/palette |
| `palette_derive` | `0.7.7` | MIT OR Apache-2.0 | https://github.com/Ogeon/palette |
| `palette_math` | `0.7.7` | MIT OR Apache-2.0 | https://github.com/Ogeon/palette |
| `parking` | `2.2.1` | Apache-2.0 OR MIT | https://github.com/smol-rs/parking |
| `parking_lot` | `0.12.5` | MIT OR Apache-2.0 | https://github.com/Amanieu/parking_lot |
| `parking_lot_core` | `0.9.12` | MIT OR Apache-2.0 | https://github.com/Amanieu/parking_lot |
| `paste` | `1.0.15` | MIT OR Apache-2.0 | https://github.com/dtolnay/paste |
| `percent-encoding` | `2.3.2` | MIT OR Apache-2.0 | https://github.com/servo/rust-url/ |
| `phf` | `0.13.1` | MIT | https://github.com/rust-phf/rust-phf |
| `phf_generator` | `0.13.1` | MIT | https://github.com/rust-phf/rust-phf |
| `phf_macros` | `0.13.1` | MIT | https://github.com/rust-phf/rust-phf |
| `phf_shared` | `0.13.1` | MIT | https://github.com/rust-phf/rust-phf |
| `pico-args` | `0.5.0` | MIT | https://github.com/RazrFalcon/pico-args |
| `pin-project` | `1.1.13` | Apache-2.0 OR MIT | https://github.com/taiki-e/pin-project |
| `pin-project-internal` | `1.1.13` | Apache-2.0 OR MIT | https://github.com/taiki-e/pin-project |
| `pin-project-lite` | `0.2.17` | Apache-2.0 OR MIT | https://github.com/taiki-e/pin-project-lite |
| `pin-utils` | `0.1.0` | MIT OR Apache-2.0 | https://github.com/rust-lang-nursery/pin-utils |
| `piper` | `0.2.5` | MIT OR Apache-2.0 | https://github.com/smol-rs/piper |
| `pkg-config` | `0.3.34` | MIT OR Apache-2.0 | https://github.com/rust-lang/pkg-config-rs |
| `plain` | `0.2.3` | MIT/Apache-2.0 | https://github.com/randomites/plain |
| `png` | `0.17.16` | MIT OR Apache-2.0 | https://github.com/image-rs/image-png |
| `png` | `0.18.1` | MIT OR Apache-2.0 | https://github.com/image-rs/image-png |
| `polling` | `3.11.0` | Apache-2.0 OR MIT | https://github.com/smol-rs/polling |
| `pollster` | `0.4.0` | Apache-2.0/MIT | https://github.com/zesterer/pollster |
| `portable-atomic` | `1.15.0` | Apache-2.0 OR MIT | https://github.com/taiki-e/portable-atomic |
| `portable-atomic-util` | `0.2.8` | Apache-2.0 OR MIT | https://github.com/taiki-e/portable-atomic-util |
| `potential_utf` | `0.1.6` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `ppv-lite86` | `0.2.21` | MIT OR Apache-2.0 | https://github.com/cryptocorrosion/cryptocorrosion |
| `presser` | `0.3.1` | MIT OR Apache-2.0 | https://github.com/EmbarkStudios/presser |
| `proc-macro-crate` | `3.5.0` | MIT OR Apache-2.0 | https://github.com/bkchr/proc-macro-crate |
| `proc-macro-error-attr3` | `3.1.1` | MIT OR Apache-2.0 | https://github.com/gamma0987/proc-macro-error3 |
| `proc-macro-error3` | `3.1.1` | MIT OR Apache-2.0 | https://github.com/gamma0987/proc-macro-error3 |
| `proc-macro2` | `1.0.107` | MIT OR Apache-2.0 | https://github.com/dtolnay/proc-macro2 |
| `proc-macro2-diagnostics` | `0.10.1` | MIT/Apache-2.0 | https://github.com/SergioBenitez/proc-macro2-diagnostics |
| `profiling` | `1.0.18` | MIT OR Apache-2.0 | https://github.com/aclysma/profiling |
| `pxfm` | `0.1.30` | BSD-3-Clause OR Apache-2.0 | https://github.com/awxkee/pxfm |
| `quick-error` | `2.0.1` | MIT/Apache-2.0 | http://github.com/tailhook/quick-error |
| `quick-xml` | `0.41.0` | MIT | https://github.com/tafia/quick-xml |
| `quote` | `1.0.47` | MIT OR Apache-2.0 | https://github.com/dtolnay/quote |
| `r-efi` | `5.3.0` | MIT OR Apache-2.0 OR LGPL-2.1-or-later | https://github.com/r-efi/r-efi |
| `r-efi` | `6.0.0` | MIT OR Apache-2.0 OR LGPL-2.1-or-later | https://github.com/r-efi/r-efi |
| `rand` | `0.8.8` | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| `rand` | `0.9.5` | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| `rand_chacha` | `0.3.1` | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| `rand_chacha` | `0.9.0` | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| `rand_core` | `0.6.4` | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| `rand_core` | `0.9.5` | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| `range-alloc` | `0.1.5` | MIT OR Apache-2.0 | https://github.com/gfx-rs/range-alloc |
| `rangemap` | `1.8.0` | MIT/Apache-2.0 | https://github.com/jeffparsons/rangemap |
| `raw-window-handle` | `0.6.2` | MIT OR Apache-2.0 OR Zlib | https://github.com/rust-windowing/raw-window-handle |
| `read-fonts` | `0.37.0` | MIT OR Apache-2.0 | https://github.com/googlefonts/fontations |
| `read-fonts` | `0.41.0` | MIT OR Apache-2.0 | https://github.com/googlefonts/fontations |
| `redox_event` | `0.4.8` | MIT | https://gitlab.redox-os.org/redox-os/event |
| `redox_syscall` | `0.5.18` | MIT | https://gitlab.redox-os.org/redox-os/syscall |
| `redox_syscall` | `0.9.4` | MIT | https://gitlab.redox-os.org/redox-os/kernel |
| `redox_users` | `0.5.3` | MIT | https://gitlab.redox-os.org/redox-os/users |
| `regex` | `1.13.1` | MIT OR Apache-2.0 | https://github.com/rust-lang/regex |
| `regex-automata` | `0.4.18` | MIT OR Apache-2.0 | https://github.com/rust-lang/regex |
| `regex-syntax` | `0.8.11` | MIT OR Apache-2.0 | https://github.com/rust-lang/regex |
| `renderdoc-sys` | `1.1.0` | MIT OR Apache-2.0 | https://github.com/ebkalderon/renderdoc-rs |
| `resvg` | `0.45.1` | Apache-2.0 OR MIT | https://github.com/linebender/resvg |
| `rfd` | `0.16.0` | MIT | https://github.com/PolyMeilex/rfd |
| `rgb` | `0.8.53` | MIT | https://github.com/kornelski/rust-rgb |
| `ron` | `0.12.2` | MIT OR Apache-2.0 | https://github.com/ron-rs/ron |
| `roxmltree` | `0.20.0` | MIT OR Apache-2.0 | https://github.com/RazrFalcon/roxmltree |
| `rust-embed` | `8.12.0` | MIT | https://pyrossh.dev/repos/rust-embed |
| `rust-embed-impl` | `8.12.0` | MIT | https://pyrossh.dev/repos/rust-embed |
| `rust-embed-utils` | `8.12.0` | MIT | https://pyrossh.dev/repos/rust-embed |
| `rustc-hash` | `1.1.0` | Apache-2.0/MIT | https://github.com/rust-lang-nursery/rustc-hash |
| `rustc-hash` | `2.1.3` | Apache-2.0 OR MIT | https://github.com/rust-lang/rustc-hash |
| `rustc_version` | `0.4.1` | MIT OR Apache-2.0 | https://github.com/djc/rustc-version-rs |
| `rustix` | `0.38.44` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/bytecodealliance/rustix |
| `rustix` | `1.1.5` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/bytecodealliance/rustix |
| `rustversion` | `1.0.23` | MIT OR Apache-2.0 | https://github.com/dtolnay/rustversion |
| `rustybuzz` | `0.20.1` | MIT | https://github.com/harfbuzz/rustybuzz |
| `same-file` | `1.0.6` | Unlicense/MIT | https://github.com/BurntSushi/same-file |
| `scoped-tls` | `1.0.1` | MIT/Apache-2.0 | https://github.com/alexcrichton/scoped-tls |
| `scopeguard` | `1.2.0` | MIT OR Apache-2.0 | https://github.com/bluss/scopeguard |
| `sctk-adwaita` | `0.11.1` | MIT | https://github.com/PolyMeilex/sctk-adwaita |
| `self_cell` | `1.3.0` | Apache-2.0 OR GPL-2.0-only | https://github.com/Voultapher/self_cell |
| `semver` | `1.0.28` | MIT OR Apache-2.0 | https://github.com/dtolnay/semver |
| `serde` | `1.0.229` | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| `serde_core` | `1.0.229` | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| `serde_derive` | `1.0.229` | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| `serde_json` | `1.0.151` | MIT OR Apache-2.0 | https://github.com/serde-rs/json |
| `serde_repr` | `0.1.21` | MIT OR Apache-2.0 | https://github.com/dtolnay/serde-repr |
| `sha2` | `0.11.0` | MIT OR Apache-2.0 | https://github.com/RustCrypto/hashes |
| `shlex` | `2.0.1` | MIT OR Apache-2.0 | https://github.com/comex/rust-shlex |
| `signal-hook-registry` | `1.4.8` | MIT OR Apache-2.0 | https://github.com/vorner/signal-hook |
| `simd-adler32` | `0.3.10` | MIT | https://github.com/mcountryman/simd-adler32 |
| `simd_cesu8` | `1.2.0` | Apache-2.0 OR MIT | https://github.com/seancroach/simd_cesu8 |
| `simdutf8` | `0.1.5` | MIT OR Apache-2.0 | https://github.com/rusticstuff/simdutf8 |
| `simplecss` | `0.2.2` | Apache-2.0 OR MIT | https://github.com/linebender/simplecss |
| `siphasher` | `1.0.3` | MIT/Apache-2.0 | https://github.com/jedisct1/rust-siphash |
| `skrifa` | `0.40.0` | MIT OR Apache-2.0 | https://github.com/googlefonts/fontations |
| `skrifa` | `0.44.0` | MIT OR Apache-2.0 | https://github.com/googlefonts/fontations |
| `slab` | `0.4.12` | MIT | https://github.com/tokio-rs/slab |
| `slotmap` | `1.1.1` | Zlib | https://github.com/orlp/slotmap |
| `smallvec` | `1.16.1` | MIT OR Apache-2.0 | https://github.com/servo/rust-smallvec |
| `smithay-client-toolkit` | `0.20.0` | MIT | https://github.com/smithay/client-toolkit |
| `smithay-clipboard` | `0.8.0` | MIT | https://github.com/smithay/smithay-clipboard |
| `smol_str` | `0.3.6` | MIT OR Apache-2.0 | https://github.com/rust-lang/rust-analyzer/tree/master/lib/smol_str |
| `socket2` | `0.6.5` | MIT OR Apache-2.0 | https://github.com/rust-lang/socket2 |
| `softbuffer` | `0.4.1` | MIT OR Apache-2.0 | https://github.com/rust-windowing/softbuffer |
| `spirv` | `0.3.0+sdk-1.3.268.0` | Apache-2.0 | https://github.com/gfx-rs/rspirv |
| `stable_deref_trait` | `1.2.1` | MIT OR Apache-2.0 | https://github.com/storyyeller/stable_deref_trait |
| `static_assertions` | `1.1.0` | MIT OR Apache-2.0 | https://github.com/nvzqz/static-assertions-rs |
| `strict-num` | `0.1.1` | MIT | https://github.com/RazrFalcon/strict-num |
| `strsim` | `0.11.1` | MIT | https://github.com/rapidfuzz/strsim-rs |
| `svg_fmt` | `0.4.5` | MIT/Apache-2.0 | https://github.com/nical/rust_debug |
| `svgtypes` | `0.15.3` | Apache-2.0 OR MIT | https://github.com/linebender/svgtypes |
| `swash` | `0.2.10` | Apache-2.0 OR MIT | https://github.com/dfrg/swash |
| `syn` | `2.0.119` | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| `syn` | `3.0.6` | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| `synstructure` | `0.14.0` | MIT | https://github.com/mystor/synstructure |
| `sys-locale` | `0.3.2` | MIT OR Apache-2.0 | https://github.com/1Password/sys-locale |
| `taffy` | `0.9.2` | MIT | https://github.com/DioxusLabs/taffy |
| `tempfile` | `3.27.0` | MIT OR Apache-2.0 | https://github.com/Stebalien/tempfile |
| `termcolor` | `1.4.1` | Unlicense OR MIT | https://github.com/BurntSushi/termcolor |
| `thiserror` | `1.0.69` | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| `thiserror` | `2.0.20` | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| `thiserror-impl` | `1.0.69` | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| `thiserror-impl` | `2.0.20` | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| `tiny-skia` | `0.11.4` | BSD-3-Clause | https://github.com/RazrFalcon/tiny-skia |
| `tiny-skia` | `0.12.0` | BSD-3-Clause | https://github.com/linebender/tiny-skia |
| `tiny-skia-path` | `0.11.4` | BSD-3-Clause | https://github.com/RazrFalcon/tiny-skia/tree/master/path |
| `tiny-skia-path` | `0.12.0` | BSD-3-Clause | https://github.com/linebender/tiny-skia/tree/master/path |
| `tiny-xlib` | `0.2.5` | MIT OR Apache-2.0 OR Zlib | https://github.com/rust-windowing/tiny-xlib |
| `tinystr` | `0.8.4` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `tinyvec` | `1.13.3` | Zlib OR Apache-2.0 OR MIT | https://github.com/Lokathor/tinyvec |
| `tokio` | `1.53.1` | MIT | https://github.com/tokio-rs/tokio |
| `tokio-macros` | `2.7.2` | MIT | https://github.com/tokio-rs/tokio |
| `tokio-stream` | `0.1.19` | MIT | https://github.com/tokio-rs/tokio |
| `toml` | `0.5.11` | MIT/Apache-2.0 | https://github.com/toml-rs/toml |
| `toml_datetime` | `1.1.1+spec-1.1.0` | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| `toml_edit` | `0.25.15+spec-1.1.0` | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| `toml_parser` | `1.1.3+spec-1.1.0` | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| `tracing` | `0.1.44` | MIT | https://github.com/tokio-rs/tracing |
| `tracing-attributes` | `0.1.31` | MIT | https://github.com/tokio-rs/tracing |
| `tracing-core` | `0.1.36` | MIT | https://github.com/tokio-rs/tracing |
| `ttf-parser` | `0.25.1` | MIT OR Apache-2.0 | https://github.com/harfbuzz/ttf-parser |
| `type-map` | `0.5.1` | MIT/Apache-2.0 | https://github.com/kardeiz/type-map |
| `typeid` | `1.0.3` | MIT OR Apache-2.0 | https://github.com/dtolnay/typeid |
| `typenum` | `1.20.1` | MIT OR Apache-2.0 | https://github.com/paholg/typenum |
| `uds_windows` | `1.2.1` | MIT | https://github.com/haraldh/rust_uds_windows |
| `uncased` | `0.9.10` | MIT OR Apache-2.0 | https://github.com/SergioBenitez/uncased |
| `unic-langid` | `0.9.6` | MIT OR Apache-2.0 | https://github.com/zbraniecki/unic-locale |
| `unic-langid-impl` | `0.9.6` | MIT OR Apache-2.0 | https://github.com/zbraniecki/unic-locale |
| `unicase` | `2.9.0` | MIT OR Apache-2.0 | https://github.com/seanmonstar/unicase |
| `unicode-bidi` | `0.3.18` | MIT OR Apache-2.0 | https://github.com/servo/unicode-bidi |
| `unicode-bidi-mirroring` | `0.4.0` | MIT/Apache-2.0 | https://github.com/RazrFalcon/unicode-bidi-mirroring |
| `unicode-ccc` | `0.4.0` | MIT/Apache-2.0 | https://github.com/RazrFalcon/unicode-ccc |
| `unicode-ident` | `1.0.25` | (MIT OR Apache-2.0) AND Unicode-3.0 | https://github.com/dtolnay/unicode-ident |
| `unicode-linebreak` | `0.1.5` | Apache-2.0 | https://github.com/axelf4/unicode-linebreak |
| `unicode-properties` | `0.1.4` | MIT/Apache-2.0 | https://github.com/unicode-rs/unicode-properties |
| `unicode-script` | `0.5.8` | MIT OR Apache-2.0 | https://github.com/unicode-rs/unicode-script |
| `unicode-segmentation` | `1.13.3` | MIT OR Apache-2.0 | https://github.com/unicode-rs/unicode-segmentation |
| `unicode-vo` | `0.1.0` | MIT/Apache-2.0 | https://github.com/RazrFalcon/unicode-vo |
| `unicode-width` | `0.2.2` | MIT OR Apache-2.0 | https://github.com/unicode-rs/unicode-width |
| `url` | `2.5.8` | MIT OR Apache-2.0 | https://github.com/servo/rust-url |
| `urlencoding` | `2.1.3` | MIT | https://github.com/kornelski/rust_urlencoding |
| `usvg` | `0.45.1` | Apache-2.0 OR MIT | https://github.com/linebender/resvg |
| `utf8_iter` | `1.0.4` | Apache-2.0 OR MIT | https://github.com/hsivonen/utf8_iter |
| `utf8parse` | `0.2.2` | Apache-2.0 OR MIT | https://github.com/alacritty/vte |
| `uuid` | `1.26.1` | Apache-2.0 OR MIT | https://github.com/uuid-rs/uuid |
| `version_check` | `0.9.5` | MIT/Apache-2.0 | https://github.com/SergioBenitez/version_check |
| `walkdir` | `2.5.0` | Unlicense/MIT | https://github.com/BurntSushi/walkdir |
| `wasi` | `0.11.1+wasi-snapshot-preview1` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/bytecodealliance/wasi |
| `wasip2` | `1.0.4+wasi-0.2.12` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/bytecodealliance/wasi-rs |
| `wasm-bindgen` | `0.2.128` | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen |
| `wasm-bindgen-futures` | `0.4.78` | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/futures |
| `wasm-bindgen-macro` | `0.2.128` | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro |
| `wasm-bindgen-macro-support` | `0.2.128` | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/main/crates/macro-support |
| `wasm-bindgen-shared` | `0.2.128` | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/shared |
| `wasmtimer` | `0.4.3` | MIT | https://github.com/whizsid/wasmtimer-rs |
| `wayland-backend` | `0.3.17` | MIT | https://github.com/smithay/wayland-rs |
| `wayland-client` | `0.31.15` | MIT | https://github.com/smithay/wayland-rs |
| `wayland-csd-frame` | `0.3.0` | MIT | https://github.com/rust-windowing/wayland-csd-frame |
| `wayland-cursor` | `0.31.14` | MIT | https://github.com/smithay/wayland-rs |
| `wayland-protocols` | `0.32.13` | MIT | https://github.com/smithay/wayland-rs |
| `wayland-protocols-experimental` | `20250721.0.1` | MIT | https://github.com/smithay/wayland-rs |
| `wayland-protocols-misc` | `0.3.12` | MIT | https://github.com/smithay/wayland-rs |
| `wayland-protocols-plasma` | `0.3.12` | MIT | https://github.com/smithay/wayland-rs |
| `wayland-protocols-wlr` | `0.3.12` | MIT | https://github.com/smithay/wayland-rs |
| `wayland-scanner` | `0.31.11` | MIT | https://github.com/smithay/wayland-rs |
| `wayland-server` | `0.31.14` | MIT | https://github.com/smithay/wayland-rs |
| `wayland-sys` | `0.31.11` | MIT | https://github.com/smithay/wayland-rs |
| `web-sys` | `0.3.105` | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/web-sys |
| `web-time` | `1.1.0` | MIT OR Apache-2.0 | https://github.com/daxpedda/web-time |
| `weezl` | `0.1.12` | MIT OR Apache-2.0 | https://github.com/image-rs/weezl |
| `wgpu` | `28.0.0` | MIT OR Apache-2.0 | https://github.com/gfx-rs/wgpu |
| `wgpu-core` | `28.0.1` | MIT OR Apache-2.0 | https://github.com/gfx-rs/wgpu |
| `wgpu-core-deps-apple` | `28.0.0` | MIT OR Apache-2.0 | https://github.com/gfx-rs/wgpu |
| `wgpu-core-deps-emscripten` | `28.0.0` | MIT OR Apache-2.0 | https://github.com/gfx-rs/wgpu |
| `wgpu-core-deps-windows-linux-android` | `28.0.0` | MIT OR Apache-2.0 | https://github.com/gfx-rs/wgpu |
| `wgpu-hal` | `28.0.1` | MIT OR Apache-2.0 | https://github.com/gfx-rs/wgpu |
| `wgpu-types` | `28.0.0` | MIT OR Apache-2.0 | https://github.com/gfx-rs/wgpu |
| `winapi` | `0.3.9` | MIT/Apache-2.0 | https://github.com/retep998/winapi-rs |
| `winapi-i686-pc-windows-gnu` | `0.4.0` | MIT/Apache-2.0 | https://github.com/retep998/winapi-rs |
| `winapi-util` | `0.1.11` | Unlicense OR MIT | https://github.com/BurntSushi/winapi-util |
| `winapi-x86_64-pc-windows-gnu` | `0.4.0` | MIT/Apache-2.0 | https://github.com/retep998/winapi-rs |
| `window_clipboard` | `0.4.1` | MIT | https://github.com/hecrj/window_clipboard |
| `windows` | `0.61.3` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows` | `0.62.2` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-collections` | `0.2.0` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-collections` | `0.3.2` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-core` | `0.61.2` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-core` | `0.62.2` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-future` | `0.2.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-future` | `0.3.2` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-implement` | `0.60.2` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-interface` | `0.59.3` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-link` | `0.1.3` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-link` | `0.2.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-numerics` | `0.2.0` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-numerics` | `0.3.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-result` | `0.3.4` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-result` | `0.4.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-strings` | `0.4.2` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-strings` | `0.5.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-sys` | `0.48.0` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-sys` | `0.52.0` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-sys` | `0.59.0` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-sys` | `0.60.2` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-sys` | `0.61.2` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-targets` | `0.48.5` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-targets` | `0.52.6` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-targets` | `0.53.5` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-threading` | `0.1.0` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows-threading` | `0.2.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_aarch64_gnullvm` | `0.48.5` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_aarch64_gnullvm` | `0.52.6` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_aarch64_gnullvm` | `0.53.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_aarch64_msvc` | `0.48.5` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_aarch64_msvc` | `0.52.6` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_aarch64_msvc` | `0.53.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_i686_gnu` | `0.48.5` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_i686_gnu` | `0.52.6` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_i686_gnu` | `0.53.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_i686_gnullvm` | `0.52.6` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_i686_gnullvm` | `0.53.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_i686_msvc` | `0.48.5` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_i686_msvc` | `0.52.6` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_i686_msvc` | `0.53.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_x86_64_gnu` | `0.48.5` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_x86_64_gnu` | `0.52.6` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_x86_64_gnu` | `0.53.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_x86_64_gnullvm` | `0.48.5` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_x86_64_gnullvm` | `0.52.6` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_x86_64_gnullvm` | `0.53.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_x86_64_msvc` | `0.48.5` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_x86_64_msvc` | `0.52.6` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `windows_x86_64_msvc` | `0.53.1` | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| `winit` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winit-android` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winit-appkit` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winit-common` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winit-core` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winit-orbital` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winit-uikit` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winit-wayland` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winit-web` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winit-win32` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winit-x11` | `0.31.0-beta.2` | Apache-2.0 | https://github.com/rust-windowing/winit |
| `winnow` | `1.0.4` | MIT | https://github.com/winnow-rs/winnow |
| `wit-bindgen` | `0.57.1` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/bytecodealliance/wit-bindgen |
| `writeable` | `0.6.4` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `x11-dl` | `2.21.0` | MIT | https://github.com/AltF02/x11-rs.git |
| `x11rb` | `0.13.2` | MIT OR Apache-2.0 | https://github.com/psychon/x11rb |
| `x11rb-protocol` | `0.13.2` | MIT OR Apache-2.0 | https://github.com/psychon/x11rb |
| `xcursor` | `0.3.11` | MIT | https://github.com/esposm03/xcursor-rs |
| `xdg` | `3.0.0` | Apache-2.0 OR MIT | https://github.com/whitequark/rust-xdg |
| `xkbcommon` | `0.7.0` | MIT | https://github.com/rust-x-bindings/xkbcommon-rs |
| `xkbcommon` | `0.8.0` | MIT | https://github.com/rust-x-bindings/xkbcommon-rs |
| `xkbcommon-dl` | `0.4.2` | MIT | https://github.com/rust-windowing/xkbcommon-dl |
| `xkeysym` | `0.2.1` | MIT OR Apache-2.0 OR Zlib | https://github.com/notgull/xkeysym |
| `xml-rs` | `0.8.29` | MIT | https://github.com/kornelski/xml-rs |
| `xmlwriter` | `0.1.0` | MIT | https://github.com/RazrFalcon/xmlwriter |
| `yansi` | `1.0.1` | MIT OR Apache-2.0 | https://github.com/SergioBenitez/yansi |
| `yazi` | `0.2.1` | Apache-2.0 OR MIT | https://github.com/dfrg/yazi |
| `yoke` | `0.8.3` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `yoke-derive` | `0.8.3` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `zbus` | `5.19.0` | MIT | https://github.com/z-galaxy/zbus/ |
| `zbus-lockstep` | `0.5.2` | MIT | https://github.com/luukvanderduim/zbus-lockstep |
| `zbus-lockstep-macros` | `0.5.2` | MIT | https://github.com/luukvanderduim/zbus-lockstep |
| `zbus_macros` | `5.19.0` | MIT | https://github.com/z-galaxy/zbus/ |
| `zbus_names` | `4.3.4` | MIT | https://github.com/z-galaxy/zbus/ |
| `zbus_xml` | `5.2.1` | MIT | https://github.com/z-galaxy/zbus/ |
| `zcheapstr` | `1.1.0` | MIT | https://github.com/z-galaxy/zcheapstr/ |
| `zeno` | `0.3.3` | Apache-2.0 OR MIT | https://github.com/dfrg/zeno |
| `zerocopy` | `0.8.57` | BSD-2-Clause OR Apache-2.0 OR MIT | https://github.com/google/zerocopy |
| `zerocopy-derive` | `0.8.57` | BSD-2-Clause OR Apache-2.0 OR MIT | https://github.com/google/zerocopy |
| `zerofrom` | `0.1.8` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `zerofrom-derive` | `0.1.8` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `zerotrie` | `0.2.5` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `zerovec` | `0.11.8` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `zerovec-derive` | `0.11.6` | Unicode-3.0 | https://github.com/unicode-org/icu4x |
| `zlib-rs` | `0.6.8` | Zlib | https://github.com/trifectatechfoundation/zlib-rs |
| `zmij` | `1.0.23` | MIT | https://github.com/dtolnay/zmij |
| `zune-core` | `0.4.12` | MIT OR Apache-2.0 OR Zlib | https://github.com/etemesi254/zune-image/tree/dev/zune-core |
| `zune-core` | `0.5.3` | MIT OR Apache-2.0 OR Zlib | https://github.com/etemesi254/zune-image |
| `zune-jpeg` | `0.4.21` | MIT OR Apache-2.0 OR Zlib | https://github.com/etemesi254/zune-image/tree/dev/crates/zune-jpeg |
| `zune-jpeg` | `0.5.15` | MIT OR Apache-2.0 OR Zlib | https://github.com/etemesi254/zune-image/tree/dev/crates/zune-jpeg |
| `zvariant` | `5.15.0` | MIT | https://github.com/z-galaxy/zbus/ |
| `zvariant_derive` | `5.15.0` | MIT | https://github.com/z-galaxy/zbus/ |
| `zvariant_utils` | `4.2.0` | MIT | https://github.com/z-galaxy/zbus/ |
