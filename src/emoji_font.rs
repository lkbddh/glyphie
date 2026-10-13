//! The emoji font shipped inside the binary, so the picker draws every emoji in
//! `data/emojis.json` whatever version of Noto Color Emoji the system has.

use cosmic::iced::advanced::graphics::text::cosmic_text::fontdb;
use cosmic::iced::advanced::graphics::text::font_system;
use std::sync::Arc;

/// Noto Color Emoji 2.057, the Unicode 18.0 release.
static BYTES: &[u8] = include_bytes!("../assets/fonts/NotoColorEmoji.ttf");

/// A private family name. Most systems have their own "Noto Color Emoji", often
/// an older one, and a second face under that name loses to it.
pub const FAMILY: &str = "Glyphie Emoji";

fn register_in(db: &mut fontdb::Database) {
    let mut staging = fontdb::Database::new();
    staging.load_font_source(fontdb::Source::Binary(Arc::new(BYTES)));
    for face in staging.faces() {
        let mut face = face.clone();
        face.id = fontdb::ID::dummy();
        face.families = vec![(FAMILY.to_string(), fontdb::Language::English_UnitedStates)];
        db.push_face_info(face);
    }
}

/// Adds the bundled font to the app's font system. Call it before any text is
/// laid out.
pub fn register() {
    match font_system().write() {
        Ok(mut fonts) => register_in(fonts.raw().db_mut()),
        Err(err) => log::error!("failed to register the bundled emoji font: {err}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emoji_data::{apply_skin_tone, SkinTone, EMOJIS};
    use cosmic::iced::advanced::graphics::text::cosmic_text::{
        Attrs, Buffer, Family, FontSystem, Metrics, Shaping,
    };

    /// Tells whether the bundled font, on its own, shapes a text into one real glyph.
    fn shaper() -> impl FnMut(&str) -> bool {
        let mut db = fontdb::Database::new();
        register_in(&mut db);
        let mut fonts = FontSystem::new_with_locale_and_db("en-US".into(), db);
        let mut buffer = Buffer::new(&mut fonts, Metrics::new(24.0, 32.0));

        move |text| {
            let attrs = Attrs::new().family(Family::Name(FAMILY));
            buffer.set_text(text, &attrs, Shaping::Advanced, None);
            buffer.shape_until_scroll(&mut fonts, false);
            let mut glyphs = buffer.layout_runs().flat_map(|run| run.glyphs.iter());
            matches!((glyphs.next(), glyphs.next()), (Some(glyph), None) if glyph.glyph_id != 0)
        }
    }

    // Everything the picker can show or copy has to come out of the bundled font
    // as one real glyph. This fails when the data moves ahead of the font.
    #[test]
    fn bundled_font_draws_every_emoji_and_skin_tone() {
        let mut draws = shaper();

        let mut undrawn = Vec::new();
        for emoji in EMOJIS.iter() {
            let tones = if emoji.has_skin_tones {
                SkinTone::all()
            } else {
                &[SkinTone::Default]
            };
            for &tone in tones {
                let text = apply_skin_tone(&emoji.glyph, tone);
                if !draws(&text) {
                    undrawn.push(text);
                }
            }
        }

        assert!(
            undrawn.is_empty(),
            "the bundled font cannot draw {undrawn:?}"
        );
    }

    // 🫹 and 🫺 are left out of `is_human_emoji_base` because the font has no
    // toned forms for them. When a font update makes this fail, add them there.
    #[test]
    fn bundled_font_still_lacks_toned_thumb_signs() {
        let mut draws = shaper();

        for thumb in ['\u{1FAF9}', '\u{1FAFA}'] {
            assert!(draws(&thumb.to_string()));
            for tone in '\u{1F3FB}'..='\u{1F3FF}' {
                assert!(!draws(&format!("{thumb}{tone}")));
            }
        }
    }
}
