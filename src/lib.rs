mod picker_struct;
extern crate regex;
use picker_struct::{EmojiData, EMOJI_DATA};
use regex::{escape, Regex};

/// the language for the emoji searcher
pub enum Language {
    /// english
    En,
    /// spanish
    Es,
    /// german
    Ger,
    /// french
    Fr,
    /// chinese
    Zh,
    /// japanese
    Ja,
}

/// the main interface with the static emoji
/// data.
pub struct EmojiUtil {
    current_emojis: Option<Vec<&'static str>>,
    search_string: Option<String>,
    language: Language,
}

impl EmojiUtil {
    pub fn new() -> EmojiUtil {
        EmojiUtil::default()
    }

    pub fn current_emojis() {}

    pub fn set_search_string() {}

    pub fn clear_search() {}

    pub fn set_language() {}
}

impl Default for EmojiUtil {
    fn default() -> Self {
        EmojiUtil {
            current_emojis: Some(EMOJI_DATA.iter().map(|datum| datum.emoji).collect()),
            search_string: None,
            language: Language::En,
        }
    }
}

// Search The emoji structure for emojis with
// tags matching the search string
fn emoji_search(search_string: String, lang: Language) -> Option<Vec<&'static str>> {
    if let Ok(re) = Regex::new(&escape(&search_string)) {
        let matches: Vec<&EmojiData> = EMOJI_DATA
            .iter()
            .filter(|datum| datum.tags.iter().any(|tag| re.is_match(tag)))
            .collect();
        if matches.len() > 0 {
            Some(matches.iter().map(|x| x.emoji).collect())
        } else {
            None
        }
    } else {
        None
    }
}

// runs the given text through
fn translate(tag: &'static str, lang: Language) -> &'static str {
    match lang {
        Language::En => tag,
        // TODO: run other languages through a lookup
        // table which substitutes their text for the correct
        // tag in another language.
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn emoji_search_test() {
        assert_eq!(
            Some(vec!["👍", "👎"]),
            emoji_search("thumb".to_string(), Language::En)
        );
    }
}
