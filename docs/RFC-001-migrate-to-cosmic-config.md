# RFC-001: Migrate Glyphie to `cosmic-config`

**Status:** Implemented  
**Date:** 2026-02-07  

---

## 1. Motivation

Glyphie currently uses a **hand-rolled config system** (`config.rs`) that:

- Stores config as a single `config.json` file via `serde_json` + `dirs::config_dir()`
- Uses a global `LazyLock<Mutex<ConfigState>>` with dirty-tracking and manual `flush_config()` calls
- Provides free-function accessors (`load_skin_tone()`, `save_skin_tone()`, etc.)
- Has **no support** for external config watchers, per-field subscriptions, or RON format

All first-party COSMIC applications use `cosmic-config`, the platform's standard config framework. Migrating will:

1. **Align with the COSMIC ecosystem** — configs stored in `~/.config/cosmic/com.aldeastudio.Glyphie/v1/` using RON per-field files
2. **Enable live config watching** — external tools (cosmic-settings, `cosmic-config` CLI) can update Glyphie's config at runtime
3. **Eliminate custom caching/flushing logic** — `cosmic-config` handles persistence atomically
4. **Simplify the code** — replace ~100 lines of boilerplate with a derive macro

---

## 2. Current Architecture

### `config.rs` (136 lines)

```
AppConfig (serde JSON)
├── recent_emojis: VecDeque<String>   // mutable state (recent history)
├── skin_tone: SkinTone               // user preference
├── gender_filter: GenderFilter       // user preference
└── show_subcategories: bool          // user preference

Storage: ~/.config/com.aldeastudio.Glyphie/config.json
Caching: LazyLock<Mutex<ConfigState>> with dirty bit
Flushing: explicit flush_config() before window close / after copy
```

### Call sites in `main.rs`

| Call Site | Function | When |
|-----------|----------|------|
| `init()` | `load_recent_emojis()`, `load_skin_tone()`, `load_gender_filter()`, `load_subcategories()` | App startup |
| `EmojiSelected` | `flush_config()` | After copy + before close |
| `SelectSkinTone` | `save_skin_tone()` | Preference change |
| `SelectGenderFilter` | `save_gender_filter()` | Preference change |
| `ToggleSubcategories` | `save_subcategories()` | Preference change |
| `add_to_recent()` | `save_recent_emojis()` | After emoji use |
| `KeyPressed(Escape)` | `flush_config()` | Before close |
| `copy_selected_emojis()` | `flush_config()` | After multi-copy + before close |

---

## 3. Target Architecture

### 3.1 Split into two config domains (following cosmic-files pattern)

| Domain | Type | cosmic-config API | Purpose |
|--------|------|-------------------|---------|
| **`GlyphieConfig`** | Settings | `Config::new()` + `config_subscription()` | User preferences (skin tone, gender filter, subcategories) |
| **`GlyphieState`** | State | `Config::new_state()` + `config_state_subscription()` | Mutable runtime state (recent emojis) |

**Rationale:** COSMIC convention separates _preferences_ (portable, shareable across machines) from _state_ (ephemeral, machine-specific). Recent emojis are runtime state; skin tone / gender / subcategories are user preferences.

### 3.2 New `config.rs`

```rust
use std::any::TypeId;
use std::collections::VecDeque;

use cosmic::cosmic_config::{
    self, CosmicConfigEntry,
    cosmic_config_derive::CosmicConfigEntry,
};
use cosmic::iced::Subscription;
use serde::{Deserialize, Serialize};

use crate::emoji_data::{GenderFilter, SkinTone};
use crate::APP_ID;

pub const CONFIG_VERSION: u64 = 1;
pub const MAX_RECENT_EMOJIS: usize = 30;

// ── User Preferences ────────────────────────────────────────────

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, CosmicConfigEntry)]
#[version = 1]
pub struct GlyphieConfig {
    pub skin_tone: SkinTone,
    pub gender_filter: GenderFilter,
    pub show_subcategories: bool,
}

impl Default for GlyphieConfig {
    fn default() -> Self {
        Self {
            skin_tone: SkinTone::Default,
            gender_filter: GenderFilter::default(),
            show_subcategories: true,
        }
    }
}

impl GlyphieConfig {
    pub fn load() -> (Option<cosmic_config::Config>, Self) {
        match cosmic_config::Config::new(APP_ID, CONFIG_VERSION) {
            Ok(config_handler) => {
                let config = match Self::get_entry(&config_handler) {
                    Ok(ok) => ok,
                    Err((errs, config)) => {
                        log::info!("errors loading config: {errs:?}");
                        config
                    }
                };
                (Some(config_handler), config)
            }
            Err(err) => {
                log::error!("failed to create config handler: {err}");
                (None, Self::default())
            }
        }
    }

    pub fn subscription() -> Subscription<cosmic_config::Update<Self>> {
        struct ConfigSubscription;
        cosmic_config::config_subscription(
            TypeId::of::<ConfigSubscription>(),
            APP_ID.into(),
            CONFIG_VERSION,
        )
    }
}

// ── Runtime State ───────────────────────────────────────────────

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, CosmicConfigEntry)]
#[version = 1]
pub struct GlyphieState {
    pub recent_emojis: VecDeque<String>,
}

impl Default for GlyphieState {
    fn default() -> Self {
        Self {
            recent_emojis: VecDeque::new(),
        }
    }
}

impl GlyphieState {
    pub fn load() -> (Option<cosmic_config::Config>, Self) {
        match cosmic_config::Config::new_state(APP_ID, CONFIG_VERSION) {
            Ok(config_handler) => {
                let config = match Self::get_entry(&config_handler) {
                    Ok(ok) => ok,
                    Err((errs, config)) => {
                        log::info!("errors loading state: {errs:?}");
                        config
                    }
                };
                (Some(config_handler), config)
            }
            Err(err) => {
                log::error!("failed to create state handler: {err}");
                (None, Self::default())
            }
        }
    }

    pub fn subscription() -> Subscription<cosmic_config::Update<Self>> {
        struct StateSubscription;
        cosmic_config::config_state_subscription(
            TypeId::of::<StateSubscription>(),
            APP_ID.into(),
            CONFIG_VERSION,
        )
    }
}
```

### 3.3 App struct changes

```rust
pub struct CosmicEmojiPicker {
    core: Core,
    // Config handlers (held for writing back)
    config_handler: Option<cosmic_config::Config>,
    state_handler: Option<cosmic_config::Config>,
    // Deserialized config
    config: GlyphieConfig,
    state: GlyphieState,
    // ... rest of runtime fields unchanged
}
```

### 3.4 App integration changes

#### `init()`
```rust
let (config_handler, config) = GlyphieConfig::load();
let (state_handler, state) = GlyphieState::load();
// Use config.skin_tone, config.gender_filter, config.show_subcategories
// Use state.recent_emojis
```

#### New messages
```rust
enum Message {
    // ... existing messages ...
    ConfigChanged(GlyphieConfig),
    StateChanged(GlyphieState),
}
```

#### `update()` — writing config
Replace `save_skin_tone()` + `flush_config()` pattern with:
```rust
Message::SelectSkinTone(tone) => {
    self.config.skin_tone = tone;
    if let Some(handler) = &self.config_handler {
        if let Err(e) = self.config.write_entry(handler) {
            log::error!("failed to write config: {e}");
        }
    }
}
```

#### `update()` — receiving external changes
```rust
Message::ConfigChanged(new_config) => {
    // React to external config changes (e.g. from cosmic-settings)
    let needs_refresh = new_config.gender_filter != self.config.gender_filter;
    self.config = new_config;
    if needs_refresh {
        self.refresh_cache();
    }
}
```

#### `subscription()`
```rust
fn subscription(&self) -> Subscription<Self::Message> {
    let mut subs = vec![
        keyboard_sub,
        GlyphieConfig::subscription().map(|update| {
            // Unpack Update<GlyphieConfig> → Message::ConfigChanged
            Message::ConfigChanged(update.config)
        }),
        GlyphieState::subscription().map(|update| {
            Message::StateChanged(update.config)
        }),
    ];
    // ... existing debounce/feedback subs
}
```

---

## 4. Migration Plan

### Phase 1: Dependency & Scaffolding
- [x] Enable the required `libcosmic` features for `cosmic-config`, subscriptions, and single-instance activation
- [x] Ensure `SkinTone` and `GenderFilter` derive `Eq` (required by `CosmicConfigEntry`)
- [x] Write the new `config.rs` with `GlyphieConfig` + `GlyphieState`

### Phase 2: App Integration
- [x] Add `config_handler`, `state_handler`, `config`, `state` fields to `CosmicEmojiPicker`
- [x] Replace `init()` to use `GlyphieConfig::load()` + `GlyphieState::load()`
- [x] Add `ConfigChanged` / `StateChanged` messages
- [x] Wire subscriptions in `subscription()`
- [x] Update all `update()` arms to write through handlers instead of free functions

### Phase 3: Cleanup
- [x] Remove all free functions (`load_skin_tone`, `save_skin_tone`, `flush_config`, etc.)
- [x] Remove unused `parking_lot`, `dirs`, `once_cell`, and direct `tokio` dependencies
- [x] Keep `serde_json` for embedded emoji data parsing
- [x] Remove the global `CONFIG_CACHE` / `LazyLock<Mutex<ConfigState>>`

### Phase 4: Data Migration (optional)
- [ ] Add one-time migration in `init()` that checks for legacy `~/.config/com.aldeastudio.Glyphie/config.json`, reads it, writes to the new cosmic-config location, and renames/deletes the old file
- [ ] Log migration success/failure

### Phase 5: Validation
- [x] Verify config files appear at `~/.config/cosmic/com.aldeastudio.Glyphie/v1/`
- [ ] Verify state files appear at `~/.local/state/cosmic/com.aldeastudio.Glyphie/v1/`
- [ ] Verify external edits to RON files are picked up at runtime
- [ ] Verify preferences survive app restart
- [ ] Verify recent emojis survive app restart
- [ ] Test fresh install (no existing config)
- [ ] Test upgrade from old config.json (migration path)

---

## 5. Dependency Changes

### `Cargo.toml` diff

```diff
 [dependencies]
-libcosmic = { git = "...", features = ["tokio", "winit", "wgpu"] }
+libcosmic = { git = "...", features = ["a11y", "single-instance", "tokio", "wayland", "winit"] }
-tokio = { version = "1.49", features = ["rt-multi-thread"] }
-dirs = "6.0"
-parking_lot = "0.12"
-once_cell = "1.19"
```

> **Note:** The crate-level `wgpu` feature now maps to `libcosmic/wgpu`, so `--no-default-features` builds without forcing the WGPU renderer. `serde` stays for `cosmic-config` derives, and `serde_json` stays for parsing embedded emoji data.

---

## 6. Storage Format Comparison

### Before (single JSON file)
```
~/.config/com.aldeastudio.Glyphie/config.json
{
  "recent_emojis": ["😀", "👍"],
  "skin_tone": "Default",
  "gender_filter": "All",
  "show_subcategories": true
}
```

### After (cosmic-config per-field RON files)
```
~/.config/cosmic/com.aldeastudio.Glyphie/v1/
├── skin_tone          → "Default"
├── gender_filter      → "All"
├── show_subcategories → true

~/.local/state/cosmic/com.aldeastudio.Glyphie/v1/
└── recent_emojis      → ["😀", "👍"]
```

---

## 7. Risk Assessment

| Risk | Severity | Mitigation |
|------|----------|------------|
| Users lose preferences on upgrade | Medium | Phase 4 legacy migration reads old JSON and writes to new location |
| `cosmic-config` not available in pinned libcosmic rev | Low | Verify the feature exists at pinned commit `a9f64c33` before starting |
| `CosmicConfigEntry` derive requires `Eq` on all fields | Low | `SkinTone` and `GenderFilter` already derive `Eq` |
| Subscription overhead for a small app | Negligible | cosmic-config uses inotify, near-zero idle cost |

---

## 8. Open Questions

1. **Should `recent_emojis` cap (`MAX_RECENT_EMOJIS`) be configurable?** Currently hardcoded at 30. Could be promoted to a `GlyphieConfig` field.
2. **Version bumping strategy:** Do we start at `CONFIG_VERSION = 1` and handle future schema changes via cosmic-config's built-in versioning?
3. **Should we expose any config via `cosmic-settings` integration?** (Future consideration, not in scope for this RFC.)

---

## 9. Estimated Effort

| Phase | Est. Lines Changed | Effort |
|-------|-------------------|--------|
| Phase 1: Scaffolding | ~120 new | 1-2 hrs |
| Phase 2: Integration | ~80 modified | 2-3 hrs |
| Phase 3: Cleanup | ~100 deleted | 30 min |
| Phase 4: Migration | ~30 new | 1 hr |
| Phase 5: Validation | — | 1-2 hrs |
| **Total** | **~200 net** | **~6-8 hrs** |
