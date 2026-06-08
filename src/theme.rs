use cosmic::iced::font::Family;
use cosmic::iced::Color;
use std::sync::LazyLock;

// Noto Color Emoji provides comprehensive Unicode emoji coverage
pub static EMOJI_FONT: LazyLock<cosmic::iced::Font> = LazyLock::new(|| cosmic::iced::Font {
    family: Family::Name("Noto Color Emoji"),
    ..Default::default()
});

// UI element sizes
pub const SKIN_TONE_CIRCLE_SIZE: f32 = 18.0;
pub const SKIN_TONE_RING_GAP: f32 = 2.0;
pub const SKIN_TONE_RING_WIDTH: f32 = 2.0;

// Layout constants
pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 12.0;

// Button sizes
pub const EMOJI_BUTTON_SIZE: f32 = 44.0;
pub const CATEGORY_BUTTON_H: f32 = 32.0;
pub const CATEGORY_ICON_SIZE: u16 = 20;

// Font sizes
pub const FONT_SIZE_XS: f32 = 11.0;
pub const FONT_SIZE_SM: f32 = 12.0;
pub const FONT_SIZE_MD: f32 = 13.0;
pub const FONT_SIZE_EMOJI: f32 = 24.0;
pub const FONT_SIZE_EMPTY_ICON: f32 = 48.0;

pub fn text_muted() -> cosmic::theme::Text {
    cosmic::theme::Text::Custom(|theme| {
        let color = theme.cosmic().on_bg_color();
        cosmic::iced_widget::text::Style {
            color: Some(Color::from_rgba(color.red, color.green, color.blue, 0.6)),
        }
    })
}

pub fn text_subtle() -> cosmic::theme::Text {
    cosmic::theme::Text::Custom(|theme| {
        let color = theme.cosmic().on_bg_color();
        cosmic::iced_widget::text::Style {
            color: Some(Color::from_rgba(color.red, color.green, color.blue, 0.5)),
        }
    })
}

// Grid (u16 required by flex_row API)
pub const GRID_SPACING: u16 = 2;
pub const KEYBOARD_GRID_COLUMNS: usize = 6;
pub const KEYBOARD_GRID_COLUMNS_DELTA: isize = 6;

// Selection tray
pub const MAX_SELECTION: usize = 20;
pub const CHIP_EMOJI_SIZE: f32 = 18.0;
pub const CHIP_HEIGHT: f32 = 30.0;
pub const CHIP_PADDING_H: f32 = 8.0;
pub const CHIP_PADDING_V: f32 = 4.0;
pub const TRAY_SCROLLABLE_HEIGHT: f32 = 38.0;

// Timing
pub const SEARCH_DEBOUNCE_MS: u64 = 80;
pub const SELECTION_FEEDBACK_MS: u64 = 150;
pub const TIMER_POLL_MS: u64 = 50;

// Header button styling (matches native COSMIC header buttons)
pub const HEADER_ICON_SIZE: u16 = 20;
pub const HEADER_BUTTON_PADDING: f32 = 6.0;

// Window dimensions in logical pixels
// COSMIC/iced automatically scales these based on display scale factor
pub const WINDOW_WIDTH: f32 = 330.0;
pub const WINDOW_HEIGHT: f32 = 480.0;

// Container styles (theme-aware)
pub fn subtle_border_style() -> cosmic::theme::Container<'static> {
    cosmic::theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        let color = cosmic.on_bg_color();
        cosmic::iced_widget::container::Style {
            background: Some(cosmic::iced::Background::Color(Color::from_rgba(
                color.red,
                color.green,
                color.blue,
                0.15,
            ))),
            ..Default::default()
        }
    })
}

/// Create accent-tinted button style with given background alpha
fn accent_button_style(theme: &cosmic::Theme, bg_alpha: f32) -> cosmic::widget::button::Style {
    let cosmic = theme.cosmic();
    let accent = cosmic.accent_color();
    cosmic::widget::button::Style {
        background: Some(cosmic::iced::Background::Color(Color::from_rgba(
            accent.red,
            accent.green,
            accent.blue,
            bg_alpha,
        ))),
        border_radius: cosmic.corner_radii.radius_m.into(),
        text_color: Some(Color::from_rgb(accent.red, accent.green, accent.blue)),
        ..Default::default()
    }
}

/// Transparent button style for circular elements (e.g. skin tone dots)
/// that should not show rectangular hover/pressed backgrounds.
pub fn transparent_button() -> cosmic::theme::Button {
    fn transparent(_: &cosmic::Theme) -> cosmic::widget::button::Style {
        cosmic::widget::button::Style {
            background: None,
            ..Default::default()
        }
    }
    cosmic::theme::Button::Custom {
        active: Box::new(|_, t| transparent(t)),
        disabled: Box::new(|_| cosmic::widget::button::Style {
            background: None,
            ..Default::default()
        }),
        hovered: Box::new(|_, t| transparent(t)),
        pressed: Box::new(|_, t| transparent(t)),
    }
}

/// Subtle chip button for selection tray pills
pub fn chip_button() -> cosmic::theme::Button {
    fn chip_style(theme: &cosmic::Theme, alpha: f32) -> cosmic::widget::button::Style {
        let cosmic = theme.cosmic();
        let color = cosmic.on_bg_color();
        cosmic::widget::button::Style {
            background: Some(cosmic::iced::Background::Color(Color::from_rgba(
                color.red,
                color.green,
                color.blue,
                alpha,
            ))),
            border_radius: cosmic.corner_radii.radius_s.into(),
            ..Default::default()
        }
    }
    cosmic::theme::Button::Custom {
        active: Box::new(|_, theme| chip_style(theme, 0.12)),
        disabled: Box::new(|_| cosmic::widget::button::Style::default()),
        hovered: Box::new(|_, theme| chip_style(theme, 0.22)),
        pressed: Box::new(|_, theme| chip_style(theme, 0.30)),
    }
}

/// Accent-tinted button style for selected filter buttons (gender, etc.)
pub fn accent_tint_button() -> cosmic::theme::Button {
    cosmic::theme::Button::Custom {
        active: Box::new(|_, theme| accent_button_style(theme, 0.15)),
        disabled: Box::new(|_| cosmic::widget::button::Style::default()),
        hovered: Box::new(|_, theme| accent_button_style(theme, 0.25)),
        pressed: Box::new(|_, theme| accent_button_style(theme, 0.3)),
    }
}
