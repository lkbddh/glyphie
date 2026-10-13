use crate::fl;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Emoji {
    #[serde(rename = "e")]
    pub glyph: String,
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "c")]
    pub category: EmojiCategory,
    #[serde(rename = "s")]
    pub has_skin_tones: bool,
    #[serde(rename = "sub")]
    pub subcategory: Option<String>,
    #[serde(rename = "st")]
    pub search_text: String,
}

impl Emoji {
    /// The gender a People & Body emoji shows, read from its characters.
    /// Groups that mix both are neutral.
    pub fn gender(&self) -> Gender {
        if self.category != EmojiCategory::PeopleBody {
            return Gender::Neutral;
        }
        let feminine = self.glyph.chars().any(is_feminine);
        let masculine = self.glyph.chars().any(is_masculine);
        match (feminine, masculine) {
            (true, false) => Gender::Feminine,
            (false, true) => Gender::Masculine,
            _ => Gender::Neutral,
        }
    }
}

// The ♀ and ♂ signs that gender a ZWJ sequence, plus the people Unicode lists
// under "Emoji With Explicit Gender Appearance" (UTS #51).
fn is_feminine(c: char) -> bool {
    matches!(
        c,
        '\u{2640}'        // ♀ female sign
            | '\u{1F467}' // 👧 girl
            | '\u{1F469}' // 👩 woman
            | '\u{1F46D}' // 👭 women holding hands
            | '\u{1F475}' // 👵 old woman
            | '\u{1F478}' // 👸 princess
            | '\u{1F483}' // 💃 woman dancing
            | '\u{1F930}' // 🤰 pregnant woman
            | '\u{1F931}' // 🤱 breast-feeding
            | '\u{1F936}' // 🤶 Mrs. Claus
            | '\u{1F9D5}' // 🧕 woman with headscarf
    )
}

fn is_masculine(c: char) -> bool {
    matches!(
        c,
        '\u{2642}'        // ♂ male sign
            | '\u{1F385}' // 🎅 Santa Claus
            | '\u{1F466}' // 👦 boy
            | '\u{1F468}' // 👨 man
            | '\u{1F46C}' // 👬 men holding hands
            | '\u{1F474}' // 👴 old man
            | '\u{1F57A}' // 🕺 man dancing
            | '\u{1F934}' // 🤴 prince
            | '\u{1FAC3}' // 🫃 pregnant man
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EmojiCategory {
    Recent,
    SmileysEmotion,
    PeopleBody,
    AnimalsNature,
    FoodDrink,
    TravelPlaces,
    Activities,
    Objects,
    Symbols,
    Flags,
}

struct CategoryNames {
    recent: String,
    smileys: String,
    people: String,
    animals: String,
    food: String,
    travel: String,
    activities: String,
    objects: String,
    symbols: String,
    flags: String,
}

/// Cached category names to avoid per-frame allocation from fl!()
static CATEGORY_NAMES: LazyLock<CategoryNames> = LazyLock::new(|| CategoryNames {
    recent: fl!("category-recent"),
    smileys: fl!("category-smileys"),
    people: fl!("category-people"),
    animals: fl!("category-animals"),
    food: fl!("category-food"),
    travel: fl!("category-travel"),
    activities: fl!("category-activities"),
    objects: fl!("category-objects"),
    symbols: fl!("category-symbols"),
    flags: fl!("category-flags"),
});

impl EmojiCategory {
    pub const fn all() -> &'static [EmojiCategory] {
        &[
            EmojiCategory::Recent,
            EmojiCategory::SmileysEmotion,
            EmojiCategory::PeopleBody,
            EmojiCategory::AnimalsNature,
            EmojiCategory::FoodDrink,
            EmojiCategory::TravelPlaces,
            EmojiCategory::Activities,
            EmojiCategory::Objects,
            EmojiCategory::Symbols,
            EmojiCategory::Flags,
        ]
    }

    pub fn name(&self) -> &str {
        let names = &*CATEGORY_NAMES;
        match self {
            EmojiCategory::Recent => &names.recent,
            EmojiCategory::SmileysEmotion => &names.smileys,
            EmojiCategory::PeopleBody => &names.people,
            EmojiCategory::AnimalsNature => &names.animals,
            EmojiCategory::FoodDrink => &names.food,
            EmojiCategory::TravelPlaces => &names.travel,
            EmojiCategory::Activities => &names.activities,
            EmojiCategory::Objects => &names.objects,
            EmojiCategory::Symbols => &names.symbols,
            EmojiCategory::Flags => &names.flags,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum SkinTone {
    #[default]
    Default,
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
}

impl SkinTone {
    pub const fn all() -> &'static [SkinTone] {
        &[
            SkinTone::Default,
            SkinTone::Light,
            SkinTone::MediumLight,
            SkinTone::Medium,
            SkinTone::MediumDark,
            SkinTone::Dark,
        ]
    }

    pub const fn modifier(self) -> &'static str {
        match self {
            SkinTone::Default => "",
            SkinTone::Light => "\u{1F3FB}",
            SkinTone::MediumLight => "\u{1F3FC}",
            SkinTone::Medium => "\u{1F3FD}",
            SkinTone::MediumDark => "\u{1F3FE}",
            SkinTone::Dark => "\u{1F3FF}",
        }
    }

    pub const fn color(self) -> (f32, f32, f32) {
        match self {
            SkinTone::Default => (1.0, 0.82, 0.0),       // Yellow
            SkinTone::Light => (1.0, 0.87, 0.77),        // #FFDFC4
            SkinTone::MediumLight => (0.94, 0.76, 0.60), // #F0C19B
            SkinTone::Medium => (0.81, 0.64, 0.46),      // #CF9F76
            SkinTone::MediumDark => (0.55, 0.36, 0.24),  // #8D5C3D
            SkinTone::Dark => (0.36, 0.22, 0.15),        // #5C3726
        }
    }
}

/// Apply skin tone modifier to an emoji.
/// Handles ZWJ sequences by applying modifier after each human base character.
/// The `has_skin_tones` field from JSON data determines IF we apply this.
pub fn apply_skin_tone(emoji: &str, skin_tone: SkinTone) -> String {
    if skin_tone == SkinTone::Default {
        return emoji.to_string();
    }

    let modifier = skin_tone.modifier();
    let in_zwj_sequence = emoji.contains('\u{200D}');
    let mut result = String::with_capacity(emoji.len() + modifier.len() * 4);
    let mut skip_next_variation_selector = false;

    for c in emoji.chars() {
        if skip_next_variation_selector {
            skip_next_variation_selector = false;
            if c == '\u{FE0F}' {
                continue;
            }
        }

        // Skip existing skin tone modifiers (we'll replace them)
        if is_skin_tone_modifier(c) {
            continue;
        }

        result.push(c);

        // Don't add modifier after variation selectors or ZWJ
        if c == '\u{FE0F}' || c == '\u{200D}' {
            continue;
        }

        // Apply modifier after human/body emoji base characters. The 🤝 joining
        // 🧑‍🤝‍🧑 is never toned itself; only the two people are.
        if is_human_emoji_base(c) && !(in_zwj_sequence && c == '\u{1F91D}') {
            result.push_str(modifier);
            skip_next_variation_selector = true;
        }
    }

    result
}

/// Check if a character is a Unicode `Emoji_Modifier_Base` that accepts skin tone modifiers.
/// Matches the Unicode 18.0 `Emoji_Modifier_Base` property, except 🫹 and 🫺
/// (U+1FAF9, U+1FAFA): the bundled font has no toned forms for them yet.
#[inline]
fn is_human_emoji_base(c: char) -> bool {
    matches!(c,
        '\u{261D}' |                // ☝ index pointing up
        '\u{26F9}' |                // ⛹ person bouncing ball
        '\u{270A}'..='\u{270D}' |   // ✊–✍ fists, writing hand
        '\u{1F385}' |               // 🎅 Santa
        '\u{1F3C2}'..='\u{1F3C4}' | // 🏂–🏄 snowboarder, surfer
        '\u{1F3C7}' |               // 🏇 horse racing
        '\u{1F3CA}'..='\u{1F3CC}' | // 🏊–🏌 swimming, golfing
        '\u{1F442}'..='\u{1F443}' | // 👂👃 ear, nose
        '\u{1F446}'..='\u{1F450}' | // 👆–👐 pointing, hands
        '\u{1F466}'..='\u{1F478}' | // 👦–👸 boy..princess
        '\u{1F47C}' |               // 👼 baby angel
        '\u{1F481}'..='\u{1F483}' | // 💁–💃 tipping hand, guard, dancer
        '\u{1F485}'..='\u{1F487}' | // 💅–💇 nail polish, haircut
        '\u{1F48F}' |               // 💏 kiss
        '\u{1F491}' |               // 💑 couple with heart
        '\u{1F4AA}' |               // 💪 flexed biceps
        '\u{1F574}'..='\u{1F575}' | // 🕴🕵 levitating, detective
        '\u{1F57A}' |               // 🕺 man dancing
        '\u{1F590}' |               // 🖐 raised hand splayed
        '\u{1F595}'..='\u{1F596}' | // 🖕🖖 middle finger, vulcan
        '\u{1F645}'..='\u{1F647}' | // 🙅–🙇 gestures
        '\u{1F64B}'..='\u{1F64F}' | // 🙋–🙏 raising hand..folded hands
        '\u{1F6A3}' |               // 🚣 rowing
        '\u{1F6B4}'..='\u{1F6B6}' | // 🚴–🚶 cycling, walking
        '\u{1F6C0}' |               // 🛀 person in bath
        '\u{1F6CC}' |               // 🛌 person in bed
        '\u{1F90C}' |               // 🤌 pinched fingers
        '\u{1F90F}' |               // 🤏 pinching hand
        '\u{1F918}'..='\u{1F91F}' | // 🤘–🤟 horn sign..love-you
        '\u{1F926}' |               // 🤦 facepalm
        '\u{1F930}'..='\u{1F939}' | // 🤰–🤹 pregnant..juggling
        '\u{1F93C}'..='\u{1F93E}' | // 🤼–🤾 wrestling, water polo, handball
        '\u{1F977}' |               // 🥷 ninja
        '\u{1F9B5}'..='\u{1F9B6}' | // 🦵🦶 leg, foot
        '\u{1F9B8}'..='\u{1F9B9}' | // 🦸🦹 superhero, villain
        '\u{1F9BB}' |               // 🦻 ear with hearing aid
        '\u{1F9CD}'..='\u{1F9CF}' | // 🧍–🧏 standing, kneeling, deaf
        '\u{1F9D1}'..='\u{1F9DD}' | // 🧑–🧝 person..elf
        '\u{1FAC3}'..='\u{1FAC5}' | // 🫃–🫅 pregnant person, etc.
        '\u{1FAF0}'..='\u{1FAF8}'   // 🫰–🫸 hand gestures
    )
}

#[inline]
fn is_skin_tone_modifier(c: char) -> bool {
    matches!(c, '\u{1F3FB}'..='\u{1F3FF}')
}

/// Strip skin tone modifiers from an emoji to get the base form
#[inline]
pub fn strip_skin_tone(emoji: &str) -> String {
    emoji
        .chars()
        .filter(|&c| !is_skin_tone_modifier(c))
        .collect()
}

/// Gender of an individual emoji (intrinsic property)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum Gender {
    #[default]
    Neutral,
    Feminine,
    Masculine,
}

/// User's gender filter preference (what to show)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum GenderFilter {
    #[default]
    All,
    Feminine,
    Masculine,
}

/// Cached gender filter labels to avoid per-frame allocation
static GENDER_LABELS: LazyLock<Vec<(GenderFilter, String)>> = LazyLock::new(|| {
    vec![
        (GenderFilter::All, fl!("gender-all")),
        (GenderFilter::Feminine, fl!("gender-female")),
        (GenderFilter::Masculine, fl!("gender-male")),
    ]
});

impl GenderFilter {
    pub fn all_with_labels() -> &'static [(GenderFilter, String)] {
        &GENDER_LABELS
    }

    /// Check if emoji matches this gender filter
    #[inline]
    pub fn matches(self, emoji_gender: Gender) -> bool {
        match self {
            GenderFilter::All => true,
            GenderFilter::Feminine => matches!(emoji_gender, Gender::Feminine | Gender::Neutral),
            GenderFilter::Masculine => matches!(emoji_gender, Gender::Masculine | Gender::Neutral),
        }
    }
}

// Load emojis from embedded JSON at runtime (much faster compilation)
pub static EMOJIS: LazyLock<Vec<Emoji>> = LazyLock::new(|| {
    const EMOJI_JSON: &str = include_str!("../data/emojis.json");
    match serde_json::from_str(EMOJI_JSON) {
        Ok(emojis) => emojis,
        Err(e) => {
            log::error!("Failed to parse embedded emoji data: {e}. Using empty fallback.");
            Vec::new()
        }
    }
});

/// Returns true if the emoji data loaded successfully (non-empty)
pub fn emoji_data_loaded() -> bool {
    !EMOJIS.is_empty()
}

// O(1) lookup by category -> indices
pub static EMOJIS_BY_CATEGORY: LazyLock<HashMap<EmojiCategory, Vec<usize>>> = LazyLock::new(|| {
    let mut map: HashMap<EmojiCategory, Vec<usize>> = HashMap::new();
    for (i, emoji) in EMOJIS.iter().enumerate() {
        map.entry(emoji.category).or_default().push(i);
    }
    map
});

#[inline]
pub fn get_emoji_gender(idx: usize) -> Gender {
    EMOJIS.get(idx).map_or(Gender::Neutral, Emoji::gender)
}

/// Trigram index for fast substring search
/// Maps each 3-character sequence to the set of emoji indices containing it
pub static TRIGRAM_INDEX: LazyLock<HashMap<[u8; 3], Vec<usize>>> = LazyLock::new(|| {
    let mut index: HashMap<[u8; 3], Vec<usize>> = HashMap::new();

    for (i, emoji) in EMOJIS.iter().enumerate() {
        let bytes = emoji.search_text.as_bytes();
        if bytes.len() >= 3 {
            for window in bytes.windows(3) {
                let trigram: [u8; 3] = [window[0], window[1], window[2]];
                index.entry(trigram).or_default().push(i);
            }
        }
        // Index the first 1-2 bytes with space padding for short queries.
        // Note: this makes 1-2 char searches prefix-only on search_text
        // (e.g. "a" finds "angry" but not "banana"). This is intentional —
        // very short substring queries would return too many results.
        if bytes.len() >= 2 {
            let bigram: [u8; 3] = [b' ', bytes[0], bytes[1]];
            index.entry(bigram).or_default().push(i);
        }
        if !bytes.is_empty() {
            let unigram: [u8; 3] = [b' ', b' ', bytes[0]];
            index.entry(unigram).or_default().push(i);
        }
    }

    // Deduplicate and sort each posting list
    for list in index.values_mut() {
        list.sort_unstable();
        list.dedup();
    }

    index
});

/// Check if `haystack` contains `needle` starting at a word boundary.
/// A word boundary is the start of the string or a position after a space.
/// This prevents "art" from matching "heart" while still matching "art" and "artistic".
#[inline]
fn word_boundary_match(haystack: &str, needle: &str) -> bool {
    word_boundary_position(haystack, needle).is_some()
}

fn word_boundary_position(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return None;
    }
    let h = haystack.as_bytes();
    let n = needle.as_bytes();
    if n.len() > h.len() {
        return None;
    }
    for i in 0..=(h.len() - n.len()) {
        if i > 0 && h[i - 1] != b' ' {
            continue;
        }
        if &h[i..i + n.len()] == n {
            return Some(i);
        }
    }
    None
}

pub fn normalize_search_query(query: &str) -> String {
    query
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn query_trigrams(query: &str) -> Vec<[u8; 3]> {
    let bytes = query.as_bytes();

    if bytes.len() == 1 {
        vec![[b' ', b' ', bytes[0]]]
    } else if bytes.len() == 2 {
        vec![[b' ', bytes[0], bytes[1]]]
    } else {
        bytes.windows(3).map(|w| [w[0], w[1], w[2]]).collect()
    }
}

fn emoji_search_score(idx: usize, query: &str, terms: &[&str]) -> (usize, usize, usize) {
    let Some(emoji) = EMOJIS.get(idx) else {
        return (usize::MAX, usize::MAX, idx);
    };

    let name = emoji.name.as_str();
    let search_text = emoji.search_text.as_str();

    let class = if emoji.glyph == query {
        0
    } else if name == query {
        1
    } else if name.starts_with(query) {
        2
    } else if search_text.starts_with(query) {
        3
    } else if word_boundary_match(name, query) {
        4
    } else if word_boundary_match(search_text, query) {
        5
    } else if terms.iter().all(|term| word_boundary_match(name, term)) {
        6
    } else {
        7
    };

    let first_match = terms
        .iter()
        .filter_map(|term| word_boundary_position(search_text, term))
        .min()
        .unwrap_or(usize::MAX);

    (class, first_match, idx)
}

/// Search emojis using the trigram index
/// Returns indices of emojis whose `search_text` contains the query terms at word boundaries.
pub fn search_emojis(query: &str) -> Vec<usize> {
    let query = normalize_search_query(query);

    if query.is_empty() {
        return Vec::new();
    }

    if !query.is_ascii() {
        let exact: Vec<usize> = EMOJIS
            .iter()
            .enumerate()
            .filter_map(|(idx, emoji)| (emoji.glyph == query).then_some(idx))
            .collect();
        if !exact.is_empty() {
            return exact;
        }
    }

    let terms: Vec<&str> = query.split(' ').collect();

    let trigrams: Vec<[u8; 3]> = terms.iter().flat_map(|term| query_trigrams(term)).collect();

    if trigrams.is_empty() {
        return Vec::new();
    }

    // Get candidates from first trigram
    let Some(first_list) = TRIGRAM_INDEX.get(&trigrams[0]) else {
        return Vec::new();
    };

    // Intersect with remaining trigrams
    let mut candidates: Vec<usize> = first_list.clone();

    for trigram in &trigrams[1..] {
        let Some(list) = TRIGRAM_INDEX.get(trigram) else {
            return Vec::new();
        };
        // Intersect sorted lists
        candidates.retain(|idx| list.binary_search(idx).is_ok());
        if candidates.is_empty() {
            return Vec::new();
        }
    }

    // Final verification with word-boundary matching
    candidates.retain(|&i| {
        terms
            .iter()
            .all(|term| word_boundary_match(&EMOJIS[i].search_text, term))
    });

    candidates.sort_by_key(|&idx| emoji_search_score(idx, &query, &terms));

    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_search_query_case_and_whitespace() {
        assert_eq!(
            normalize_search_query("  GrInNiNg   FACE  "),
            "grinning face"
        );
    }

    #[test]
    fn search_ranks_exact_name_first() {
        let results = search_emojis("  GrInNiNg   FACE  ");

        assert!(!results.is_empty());
        assert_eq!(EMOJIS[results[0]].name, "grinning face");
    }

    #[test]
    fn search_matches_multi_word_terms_without_exact_phrase() {
        let results = search_emojis("cat smile");

        assert!(results.iter().any(|&idx| {
            let emoji = &EMOJIS[idx];
            emoji.name.contains("cat") && emoji.search_text.contains("smile")
        }));
    }

    #[test]
    fn search_matches_emoji_literal() {
        let results = search_emojis("😀");

        assert_eq!(
            results.first().map(|&idx| EMOJIS[idx].glyph.as_str()),
            Some("😀")
        );
    }

    #[test]
    fn search_finds_unicode_17_and_18_emojis() {
        for (query, glyph) in [("orca", "🫍"), ("pickle", "🫝")] {
            let results = search_emojis(query);

            assert_eq!(
                results.first().map(|&idx| EMOJIS[idx].glyph.as_str()),
                Some(glyph)
            );
        }
    }

    #[test]
    fn skin_tone_applies_to_wrestlers_and_bunny_ears() {
        assert_eq!(apply_skin_tone("🤼", SkinTone::Light), "🤼🏻");
        assert_eq!(
            apply_skin_tone("👯\u{200D}♀\u{FE0F}", SkinTone::Dark),
            "👯🏿\u{200D}♀\u{FE0F}"
        );
    }

    #[test]
    fn skin_tone_replaces_existing_modifier() {
        assert_eq!(apply_skin_tone("👋🏽", SkinTone::Light), "👋🏻");
    }

    #[test]
    fn skin_tone_removes_variation_selector_after_modified_base() {
        assert_eq!(apply_skin_tone("☝️", SkinTone::Light), "☝🏻");
        assert_eq!(apply_skin_tone("⛹️‍♀️", SkinTone::Dark), "⛹🏿‍♀️");
    }

    #[test]
    fn skin_tone_leaves_handshake_in_people_holding_hands_untoned() {
        assert_eq!(
            apply_skin_tone("🧑\u{200D}🤝\u{200D}🧑", SkinTone::Light),
            "🧑🏻\u{200D}🤝\u{200D}🧑🏻"
        );
        assert_eq!(apply_skin_tone("🤝", SkinTone::Light), "🤝🏻");
    }

    #[test]
    fn gender_follows_the_characters_in_the_emoji() {
        let gender = |name: &str| {
            EMOJIS
                .iter()
                .find(|emoji| emoji.name == name)
                .map(Emoji::gender)
        };

        for name in [
            "woman walking",
            "princess",
            "mermaid",
            "woman in manual wheelchair",
            "Mrs. Claus",
        ] {
            assert_eq!(gender(name), Some(Gender::Feminine), "{name}");
        }
        for name in [
            "man walking",
            "prince",
            "merman",
            "pregnant man",
            "Santa Claus",
        ] {
            assert_eq!(gender(name), Some(Gender::Masculine), "{name}");
        }
        // Mixed groups, people with no gendered form, and the ♀ sign on its own.
        for name in [
            "speaking head",
            "person walking",
            "woman and man holding hands",
            "family: man, girl",
            "person in tuxedo",
            "female sign",
        ] {
            assert_eq!(gender(name), Some(Gender::Neutral), "{name}");
        }
    }

    #[test]
    fn gender_filters_keep_neutral_emojis_visible() {
        assert!(GenderFilter::Feminine.matches(Gender::Neutral));
        assert!(GenderFilter::Masculine.matches(Gender::Neutral));
        assert!(!GenderFilter::Feminine.matches(Gender::Masculine));
        assert!(!GenderFilter::Masculine.matches(Gender::Feminine));
    }
}
