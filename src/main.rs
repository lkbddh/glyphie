#![deny(unsafe_code)]

mod localize;

mod clipboard;
mod config;
mod emoji_data;
mod picker_state;
mod theme;
mod views;

use indexmap::IndexSet;
use std::borrow::Cow;
use std::time::{Duration, Instant};

use cosmic::app::{Core, CosmicFlags, Settings, Task};
use cosmic::iced::keyboard::{Key, Modifiers};
use cosmic::iced::window::UserAttention;
use cosmic::iced::{clipboard as iced_clipboard, event, time, window, Length, Size, Subscription};
use cosmic::iced_core::keyboard::key::Named;
use cosmic::widget::{self, container, text_input};
use cosmic::{executor, Application, ApplicationExt, Element};
use std::sync::LazyLock;

use config::{GlyphieConfig, GlyphieState, MAX_RECENT_EMOJIS};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use emoji_data::{
    apply_skin_tone, get_emoji_gender, search_emojis, strip_skin_tone, EmojiCategory, GenderFilter,
    SkinTone, EMOJIS, EMOJIS_BY_CATEGORY,
};
use picker_state::{active_copy_index, next_highlight_position};
use theme::{
    subtle_border_style, EMOJI_BUTTON_SIZE, GRID_SPACING, HEADER_BUTTON_PADDING, HEADER_ICON_SIZE,
    KEYBOARD_GRID_COLUMNS, KEYBOARD_GRID_COLUMNS_DELTA, MAX_SELECTION, SEARCH_DEBOUNCE_MS,
    SELECTION_FEEDBACK_MS, SPACING_MD, SPACING_SM, SPACING_XS, TIMER_POLL_MS, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

pub const APP_ID: &str = "com.aldeastudio.Glyphie";

static SEARCH_INPUT_ID: LazyLock<widget::Id> = LazyLock::new(widget::Id::unique);
pub(crate) static EMOJI_SCROLLABLE_ID: LazyLock<widget::Id> = LazyLock::new(widget::Id::unique);
pub(crate) static TRAY_SCROLLABLE_ID: LazyLock<widget::Id> = LazyLock::new(widget::Id::unique);

pub(crate) fn horizontal_divider() -> Element<'static, Message> {
    container(widget::Space::new(Length::Fill, Length::Fixed(1.0)))
        .width(Length::Fill)
        .class(subtle_border_style())
        .into()
}

pub(crate) static ICON_CARET_UP: &[u8] = include_bytes!("../assets/caret-up.svg");
pub(crate) static ICON_CARET_DOWN: &[u8] = include_bytes!("../assets/caret-down.svg");
pub(crate) static ICON_COPY: &[u8] = include_bytes!("../assets/copy.svg");

macro_rules! svg_icon {
    ($path:literal) => {
        widget::icon::from_svg_bytes(include_bytes!($path)).symbolic(true)
    };
}

static CATEGORY_ICONS: LazyLock<CategoryIcons> = LazyLock::new(|| CategoryIcons {
    recent: svg_icon!("../assets/clock-bold.svg"),
    smileys: svg_icon!("../assets/smiley-bold.svg"),
    people: svg_icon!("../assets/hand-waving-bold.svg"),
    animals: svg_icon!("../assets/paw-print-bold.svg"),
    food: svg_icon!("../assets/hamburger-bold.svg"),
    travel: svg_icon!("../assets/airplane-bold.svg"),
    activities: svg_icon!("../assets/trophy-bold.svg"),
    objects: svg_icon!("../assets/lightbulb-bold.svg"),
    symbols: svg_icon!("../assets/heart-bold.svg"),
    flags: svg_icon!("../assets/flag-bold.svg"),
});

struct CategoryIcons {
    recent: widget::icon::Handle,
    smileys: widget::icon::Handle,
    people: widget::icon::Handle,
    animals: widget::icon::Handle,
    food: widget::icon::Handle,
    travel: widget::icon::Handle,
    activities: widget::icon::Handle,
    objects: widget::icon::Handle,
    symbols: widget::icon::Handle,
    flags: widget::icon::Handle,
}

#[inline]
pub(crate) fn category_icon(cat: EmojiCategory) -> widget::icon::Handle {
    let icons = &*CATEGORY_ICONS;
    match cat {
        EmojiCategory::Recent => icons.recent.clone(),
        EmojiCategory::SmileysEmotion => icons.smileys.clone(),
        EmojiCategory::PeopleBody => icons.people.clone(),
        EmojiCategory::AnimalsNature => icons.animals.clone(),
        EmojiCategory::FoodDrink => icons.food.clone(),
        EmojiCategory::TravelPlaces => icons.travel.clone(),
        EmojiCategory::Activities => icons.activities.clone(),
        EmojiCategory::Objects => icons.objects.clone(),
        EmojiCategory::Symbols => icons.symbols.clone(),
        EmojiCategory::Flags => icons.flags.clone(),
    }
}

fn main() -> cosmic::iced::Result {
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("warn,cosmic::app=error,glyphie=info,sctk_adwaita=off"),
    )
    .init();

    let settings = Settings::default()
        .size_limits(
            cosmic::iced::Limits::NONE
                .min_width(WINDOW_WIDTH)
                .min_height(WINDOW_HEIGHT)
                .max_width(WINDOW_WIDTH)
                .max_height(WINDOW_HEIGHT),
        )
        .size(Size::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .resizable(Some(0.0));

    cosmic::app::run_single_instance::<CosmicEmojiPicker>(settings, GlyphieFlags)
}

#[derive(Clone, Debug, Default)]
pub(crate) struct GlyphieFlags;

impl CosmicFlags for GlyphieFlags {
    type SubCommand = String;
    type Args = Vec<String>;
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    SearchChanged(String),
    CategorySelected(EmojiCategory),
    EmojiSelected(usize),
    SelectSkinTone(SkinTone),
    SelectGenderFilter(GenderFilter),
    TogglePreferences,
    ToggleSubcategories(bool),
    KeyPressed {
        key: Key,
        modifiers: Modifiers,
        captured: bool,
    },
    ToggleMultiselectMode,
    ToggleEmojiSelection(usize),
    RemoveFromSelection(usize),
    ClearSelection,
    CopyActiveEmoji,
    CopySelectedEmojis,
    ToggleSettingsMenu,
    TimerTick,
    ConfigChanged(GlyphieConfig),
    StateChanged(GlyphieState),
}

pub(crate) struct CosmicEmojiPicker {
    pub(crate) core: Core,
    // cosmic-config handlers
    config_handler: Option<cosmic_config::Config>,
    state_handler: Option<cosmic_config::Config>,
    pub(crate) config: GlyphieConfig,
    pub(crate) state: GlyphieState,
    // UI state
    pub(crate) search_query: String,
    pub(crate) pending_search: Option<String>,
    pub(crate) last_search_change: Option<Instant>,
    pub(crate) selected_category: EmojiCategory,
    pub(crate) show_preferences: bool,
    pub(crate) cached_indices: Vec<usize>,
    pub(crate) highlighted_index: Option<usize>,
    pub(crate) multiselect_mode: bool,
    pub(crate) selected_indices: IndexSet<usize>,
    pub(crate) selection_rejected_idx: Option<usize>,
    pub(crate) selection_rejected_at: Option<Instant>,
    pub(crate) show_settings_menu: bool,
}

impl Application for CosmicEmojiPicker {
    type Executor = executor::Default;
    type Flags = GlyphieFlags;
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(mut core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        core.window.show_minimize = false;
        core.window.show_maximize = false;

        let (config_handler, config) = GlyphieConfig::load();
        let (state_handler, state) = GlyphieState::load();

        let mut app = CosmicEmojiPicker {
            core,
            config_handler,
            state_handler,
            config,
            state,
            search_query: String::new(),
            pending_search: None,
            last_search_change: None,
            selected_category: EmojiCategory::Recent,
            show_preferences: false,
            cached_indices: Vec::new(),
            highlighted_index: None,
            multiselect_mode: false,
            selected_indices: IndexSet::new(),
            selection_rejected_idx: None,
            selection_rejected_at: None,
            show_settings_menu: false,
        };
        app.refresh_cache();
        let set_title = app.set_window_title("Glyphie".into());
        (
            app,
            Task::batch([set_title, text_input::focus(SEARCH_INPUT_ID.clone())]),
        )
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        let menu_label = fl!("menu");
        let menu_btn = widget::button::custom(
            widget::icon::from_name("open-menu-symbolic")
                .size(HEADER_ICON_SIZE)
                .icon(),
        )
        .on_press(Message::ToggleSettingsMenu)
        .padding(HEADER_BUTTON_PADDING)
        .name(menu_label.clone())
        .class(if self.show_settings_menu {
            cosmic::theme::Button::Suggested
        } else {
            cosmic::theme::Button::HeaderBar
        });

        vec![widget::tooltip(
            menu_btn,
            widget::text(menu_label),
            widget::tooltip::Position::Bottom,
        )
        .into()]
    }

    fn header_end(&self) -> Vec<Element<'_, Self::Message>> {
        if self.show_settings_menu {
            return vec![];
        }

        let multiselect_label = fl!("multiselect");
        let multiselect_btn = widget::button::custom(
            widget::icon::from_name("view-grid-symbolic")
                .size(HEADER_ICON_SIZE)
                .icon(),
        )
        .on_press(Message::ToggleMultiselectMode)
        .padding(HEADER_BUTTON_PADDING)
        .name(multiselect_label.clone())
        .class(if self.multiselect_mode {
            cosmic::theme::Button::Suggested
        } else {
            cosmic::theme::Button::HeaderBar
        });

        vec![widget::tooltip(
            multiselect_btn,
            widget::text(multiselect_label),
            widget::tooltip::Position::Bottom,
        )
        .into()]
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let emoji_count = self.cached_indices.len();

        if self.show_settings_menu {
            return self.view_settings_menu();
        }

        let display_query = self.pending_search.as_ref().unwrap_or(&self.search_query);
        let search_row: Element<Message> = container(
            text_input(fl!("search-placeholder"), display_query)
                .id(SEARCH_INPUT_ID.clone())
                .on_input(Message::SearchChanged)
                .on_submit(|_| Message::CopyActiveEmoji)
                .width(Length::Fill)
                .padding(SPACING_SM),
        )
        .padding([0.0, 0.0, SPACING_XS, 0.0])
        .width(Length::Fill)
        .into();

        let category_bar = self.view_category_bar();
        let skin_tone_bar = self.view_skin_tone_bar(emoji_count);
        let emoji_content = self.view_emoji_content_with(&self.cached_indices);
        let selection_tray = self.view_selection_tray();

        let emoji_card = container(
            widget::column()
                .push(skin_tone_bar)
                .push(emoji_content)
                .spacing(SPACING_SM),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .class(cosmic::theme::Container::Card);

        let mut content = widget::column().push(search_row);

        content = content.push(category_bar).push(emoji_card);

        if let Some(tray) = selection_tray {
            content = content.push(tray);
        }

        let content = content.spacing(0).width(Length::Fill).height(Length::Fill);

        container(content)
            .padding([0.0, SPACING_XS, SPACING_MD, SPACING_XS])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::SearchChanged(query) => {
                log::debug!("Search query changed: {query}");
                self.pending_search = Some(query);
                self.last_search_change = Some(Instant::now());
            }
            Message::CategorySelected(category) => {
                self.selected_category = category;
                self.search_query.clear();
                self.pending_search = None;
                self.last_search_change = None;
                self.highlighted_index = None;
                self.refresh_cache();
                return cosmic::iced_widget::scrollable::scroll_to(
                    EMOJI_SCROLLABLE_ID.clone(),
                    cosmic::iced_widget::scrollable::AbsoluteOffset::default(),
                );
            }
            Message::EmojiSelected(idx) => {
                return self.copy_single_emoji(idx);
            }
            Message::SelectSkinTone(tone) => {
                self.config.skin_tone = tone;
                self.save_config();
            }
            Message::SelectGenderFilter(filter) => {
                self.config.gender_filter = filter;
                self.save_config();
                self.refresh_cache();
            }
            Message::TogglePreferences => {
                self.show_preferences = !self.show_preferences;
            }
            Message::ToggleSubcategories(enabled) => {
                self.config.show_subcategories = enabled;
                self.save_config();
            }
            Message::KeyPressed {
                key,
                modifiers,
                captured,
            } => match key {
                Key::Named(Named::Escape) => {
                    if self.show_settings_menu {
                        self.show_settings_menu = false;
                    } else if self.show_preferences {
                        self.show_preferences = false;
                    } else if self.pending_search.is_some() || !self.search_query.is_empty() {
                        self.search_query.clear();
                        self.pending_search = None;
                        self.last_search_change = None;
                        self.refresh_cache();
                    } else if !self.selected_indices.is_empty() {
                        self.selected_indices.clear();
                    } else {
                        return Self::close_picker();
                    }
                }
                Key::Named(Named::Enter) if !captured => {
                    return self.copy_active_emoji();
                }
                Key::Character(ref c) if modifiers.control() && c.as_str() == "c" => {
                    return self.copy_active_emoji();
                }
                Key::Named(Named::ArrowDown) => {
                    return self.move_highlight(KEYBOARD_GRID_COLUMNS_DELTA);
                }
                Key::Named(Named::ArrowUp) => {
                    return self.move_highlight(-KEYBOARD_GRID_COLUMNS_DELTA);
                }
                Key::Named(Named::ArrowRight) if !captured => {
                    return self.move_highlight(1);
                }
                Key::Named(Named::ArrowLeft) if !captured => {
                    return self.move_highlight(-1);
                }
                _ => {}
            },
            Message::ToggleMultiselectMode => {
                self.multiselect_mode = !self.multiselect_mode;
                if !self.multiselect_mode {
                    self.selected_indices.clear();
                }
            }
            Message::ToggleEmojiSelection(idx) => {
                if self.selected_indices.contains(&idx) {
                    self.selected_indices.shift_remove(&idx);
                } else if self.selected_indices.len() < MAX_SELECTION {
                    self.selected_indices.insert(idx);
                } else {
                    self.selection_rejected_idx = Some(idx);
                    self.selection_rejected_at = Some(Instant::now());
                }
            }
            Message::RemoveFromSelection(idx) => {
                self.selected_indices.shift_remove(&idx);
            }
            Message::ClearSelection => {
                self.selected_indices.clear();
            }
            Message::CopyActiveEmoji => {
                return self.copy_active_emoji();
            }
            Message::CopySelectedEmojis => {
                return self.copy_selected_emojis();
            }
            Message::ToggleSettingsMenu => {
                self.show_settings_menu = !self.show_settings_menu;
            }
            Message::TimerTick => {
                let search_ready = self
                    .last_search_change
                    .is_some_and(|t| t.elapsed() >= Duration::from_millis(SEARCH_DEBOUNCE_MS));
                let reject_ready = self
                    .selection_rejected_at
                    .is_some_and(|t| t.elapsed() >= Duration::from_millis(SELECTION_FEEDBACK_MS));

                if !search_ready && !reject_ready {
                    return Task::none();
                }

                if search_ready {
                    if let Some(query) = self.pending_search.take() {
                        self.search_query = query;
                        self.refresh_cache();
                        self.highlight_first_result_if_searching();
                    }
                    self.last_search_change = None;
                }
                if reject_ready {
                    self.selection_rejected_idx = None;
                    self.selection_rejected_at = None;
                }
            }
            Message::ConfigChanged(new_config) => {
                let needs_refresh = new_config.gender_filter != self.config.gender_filter;
                self.config = new_config;
                if needs_refresh {
                    self.refresh_cache();
                }
            }
            Message::StateChanged(new_state) => {
                self.state = new_state;
                if self.selected_category == EmojiCategory::Recent {
                    self.refresh_cache();
                }
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let keyboard = event::listen_with(|event, status, _| {
            if let cosmic::iced::Event::Keyboard(cosmic::iced::keyboard::Event::KeyPressed {
                key,
                modifiers,
                ..
            }) = event
            {
                let captured = matches!(status, event::Status::Captured);
                Some(Message::KeyPressed {
                    key,
                    modifiers,
                    captured,
                })
            } else {
                None
            }
        });

        let mut subs = vec![
            keyboard,
            GlyphieConfig::subscription().map(|update| Message::ConfigChanged(update.config)),
            GlyphieState::subscription().map(|update| Message::StateChanged(update.config)),
        ];

        // Use a single fixed-interval poll for all timers so the subscription
        // identity (hash) stays stable across frames. time::every(remaining)
        // would produce a new hash each call, causing iced to drop + recreate
        // the subscription on every state change and never actually fire.
        let needs_timer = self.last_search_change.is_some() || self.selection_rejected_at.is_some();

        if needs_timer {
            subs.push(
                time::every(Duration::from_millis(TIMER_POLL_MS)).map(|_| Message::TimerTick),
            );
        }

        Subscription::batch(subs)
    }

    fn dbus_activation(&mut self, _msg: cosmic::dbus_activation::Message) -> Task<Self::Message> {
        self.focus_picker()
    }
}

impl CosmicEmojiPicker {
    pub(crate) fn get_display_emoji(&self, idx: usize) -> Cow<'_, str> {
        let Some(emoji) = EMOJIS.get(idx) else {
            log::warn!("Invalid emoji index: {idx}");
            return Cow::Borrowed("");
        };
        if emoji.has_skin_tones && self.config.skin_tone != SkinTone::Default {
            Cow::Owned(apply_skin_tone(&emoji.glyph, self.config.skin_tone))
        } else {
            Cow::Borrowed(&emoji.glyph)
        }
    }

    fn focus_picker(&self) -> Task<Message> {
        let focus_input = text_input::focus(SEARCH_INPUT_ID.clone());
        let Some(id) = self.core.main_window_id() else {
            return focus_input;
        };

        Task::batch([
            window::gain_focus(id),
            window::request_user_attention(id, Some(UserAttention::Informational)),
            focus_input,
        ])
    }

    fn copy_single_emoji(&mut self, idx: usize) -> Task<Message> {
        let display_emoji = self.get_display_emoji(idx).into_owned();
        let base_emoji = EMOJIS
            .get(idx)
            .map(|e| strip_skin_tone(&e.glyph))
            .unwrap_or_default();

        self.add_to_recent(&base_emoji);
        Self::copy_text_and_close(display_emoji)
    }

    fn copy_active_emoji(&mut self) -> Task<Message> {
        if !self.selected_indices.is_empty() {
            return self.copy_selected_emojis();
        }

        let Some(idx) = active_copy_index(self.highlighted_index, &self.cached_indices) else {
            return Task::none();
        };

        self.copy_single_emoji(idx)
    }

    fn move_highlight(&mut self, delta: isize) -> Task<Message> {
        if self.show_settings_menu || self.cached_indices.is_empty() {
            return Task::none();
        }

        let current_pos = self.highlighted_index.and_then(|idx| {
            self.cached_indices
                .iter()
                .position(|&candidate| candidate == idx)
        });
        let Some(next_pos) = next_highlight_position(self.cached_indices.len(), current_pos, delta)
        else {
            return Task::none();
        };

        self.highlighted_index = self.cached_indices.get(next_pos).copied();
        Self::scroll_highlight_into_view(next_pos)
    }

    fn scroll_highlight_into_view(position: usize) -> Task<Message> {
        let row = u16::try_from(position / KEYBOARD_GRID_COLUMNS).unwrap_or(u16::MAX);
        let row_height = EMOJI_BUTTON_SIZE + f32::from(GRID_SPACING);
        let y = (f32::from(row) * row_height - EMOJI_BUTTON_SIZE).max(0.0);

        cosmic::iced_widget::scrollable::scroll_to(
            EMOJI_SCROLLABLE_ID.clone(),
            cosmic::iced_widget::scrollable::AbsoluteOffset { x: 0.0, y },
        )
    }

    fn highlight_first_result_if_searching(&mut self) {
        if self.search_query.trim().is_empty() {
            self.highlighted_index = None;
        } else {
            self.highlighted_index = self.cached_indices.first().copied();
        }
    }

    fn reconcile_highlight(&mut self) {
        let Some(highlighted) = self.highlighted_index else {
            return;
        };

        if !self.cached_indices.contains(&highlighted) {
            self.highlighted_index = self.cached_indices.first().copied();
        }
    }

    fn add_to_recent(&mut self, emoji: &str) {
        self.state.recent_emojis.retain(|e| e != emoji);
        self.state.recent_emojis.push_front(emoji.to_string());
        if self.state.recent_emojis.len() > MAX_RECENT_EMOJIS {
            self.state.recent_emojis.pop_back();
        }
        self.save_state();
        if self.selected_category == EmojiCategory::Recent {
            self.refresh_cache();
        }
    }

    fn copy_selected_emojis(&mut self) -> Task<Message> {
        if self.selected_indices.is_empty() {
            return Task::none();
        }

        let mut combined = String::new();
        let mut base_emojis = Vec::with_capacity(self.selected_indices.len());

        for &idx in &self.selected_indices {
            let display = self.get_display_emoji(idx);
            combined.push_str(&display);
            if let Some(e) = EMOJIS.get(idx) {
                base_emojis.push(strip_skin_tone(&e.glyph));
            }
        }

        self.selected_indices.clear();

        // Batch-update recent emojis, then write state once
        for base_emoji in base_emojis.into_iter().rev() {
            self.state.recent_emojis.retain(|e| e != &base_emoji);
            self.state.recent_emojis.push_front(base_emoji);
        }
        while self.state.recent_emojis.len() > MAX_RECENT_EMOJIS {
            self.state.recent_emojis.pop_back();
        }
        self.save_state();
        if self.selected_category == EmojiCategory::Recent {
            self.refresh_cache();
        }

        Self::copy_text_and_close(combined)
    }

    fn copy_text_and_close(text: String) -> Task<Message> {
        match clipboard::copy_to_clipboard(&text) {
            Ok(()) => Self::close_picker(),
            Err(err) => {
                log::warn!("failed to copy with wl-copy: {err}; falling back to runtime clipboard");
                iced_clipboard::write(text)
            }
        }
    }

    fn close_picker() -> Task<Message> {
        cosmic::iced::exit()
    }

    fn save_config(&self) {
        if let Some(handler) = &self.config_handler {
            if let Err(e) = self.config.write_entry(handler) {
                log::error!("failed to write config: {e}");
            }
        }
    }

    fn save_state(&self) {
        if let Some(handler) = &self.state_handler {
            if let Err(e) = self.state.write_entry(handler) {
                log::error!("failed to write state: {e}");
            }
        }
    }

    fn refresh_cache(&mut self) {
        self.cached_indices = self.compute_filtered_indices();
        self.reconcile_highlight();
    }

    fn compute_filtered_indices(&self) -> Vec<usize> {
        let gender_filter = self.config.gender_filter;

        if !self.search_query.trim().is_empty() {
            // Use trigram-indexed search for fast candidate lookup
            let indices = search_emojis(&self.search_query)
                .into_iter()
                .filter(|&i| gender_filter.matches(get_emoji_gender(i)))
                .collect();
            boost_recent_matches(indices, &self.state.recent_emojis)
        } else if self.selected_category == EmojiCategory::Recent {
            self.state
                .recent_emojis
                .iter()
                .filter_map(|emoji_str| EMOJIS.iter().position(|e| e.glyph == *emoji_str))
                .filter(|&i| gender_filter.matches(get_emoji_gender(i)))
                .collect()
        } else {
            EMOJIS_BY_CATEGORY
                .get(&self.selected_category)
                .map(|indices| {
                    indices
                        .iter()
                        .copied()
                        .filter(|&i| gender_filter.matches(get_emoji_gender(i)))
                        .collect()
                })
                .unwrap_or_default()
        }
    }
}

fn boost_recent_matches(
    indices: Vec<usize>,
    recent_emojis: &std::collections::VecDeque<String>,
) -> Vec<usize> {
    let mut ranked: Vec<(usize, usize)> = indices.into_iter().enumerate().collect();
    ranked.sort_by_key(|(search_rank, idx)| {
        let recent_rank = EMOJIS
            .get(*idx)
            .and_then(|emoji| {
                recent_emojis
                    .iter()
                    .position(|recent| recent == &emoji.glyph)
            })
            .unwrap_or(usize::MAX);

        (recent_rank, *search_rank)
    });

    ranked.into_iter().map(|(_, idx)| idx).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn recent_matches_are_promoted_above_search_order() {
        let indices = search_emojis("face");
        assert!(indices.len() >= 2);

        let first = indices[0];
        let second = indices[1];
        let recent_emojis = VecDeque::from([EMOJIS[second].glyph.clone()]);

        let boosted = boost_recent_matches(vec![first, second], &recent_emojis);

        assert_eq!(boosted, vec![second, first]);
    }

    #[test]
    fn search_order_is_preserved_when_no_recent_matches() {
        let indices = search_emojis("face");
        assert!(indices.len() >= 2);

        let original = vec![indices[0], indices[1]];
        let boosted = boost_recent_matches(original.clone(), &VecDeque::new());

        assert_eq!(boosted, original);
    }
}
