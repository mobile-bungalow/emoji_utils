mod picker_struct;
extern crate regex;
use picker_struct::{EmojiData, EMOJI_DATA};
use regex::{escape, Regex};

pub use picker_struct::*;

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
    pub current_emojis: Option<Vec<&'static EmojiData>>,
    pub search_string: Option<String>,
    pub language: Language,
}

impl EmojiUtil {
    pub fn new(lang: Language) -> EmojiUtil {
        EmojiUtil {
            current_emojis: Some(EMOJI_DATA.iter().collect()),
            search_string: None,
            language: lang,
        }
    }

    pub fn search(&mut self, search_string: String) {
        self.current_emojis = emoji_search(search_string, &self.language)
    }

    pub fn clear_search(&mut self) {
        self.search_string = None;
        self.current_emojis = Some(EMOJI_DATA.iter().collect())
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
}

// Search The emoji structure for emojis with
// tags matching the search string
fn emoji_search(search_string: String, lang: &Language) -> Option<Vec<&'static EmojiData>> {
    if let Ok(re) = Regex::new(&escape(&search_string)) {
        Some(
            EMOJI_DATA
                .iter()
                .enumerate()
                .filter(|(_, datum)| datum.tags.iter().any(|tag| re.is_match(tag)))
                .map(|(i, _)| translate(i, lang))
                .collect(),
        )
    } else {
        None
    }
}

// runs the given text through
fn translate(index: usize, lang: &Language) -> &'static EmojiData {
    match lang {
        _ => &EMOJI_DATA[index],
        // TODO: run other languages through a lookup
        // table which substitutes their text for the correct
        // tag in another language. table should be lazy static
        // to avoid too much allocation.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::{Debug, Formatter, Result};

    impl PartialEq for EmojiData {
        fn eq(&self, other: &Self) -> bool {
            self.emoji == other.emoji
        }
    }
    impl Debug for EmojiData {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "({})", self.emoji)
        }
    }

    #[test]
    fn emoji_search_test() {
        let mut eu = EmojiUtil::new(Language::En);
        eu.search(String::from("thumb"));
        assert_eq!(
            Some(vec![
                &EmojiData {
                    emoji: "👍",
                    tags: &[]
                },
                &EmojiData {
                    emoji: "👎",
                    tags: &[]
                }
            ]),
            eu.current_emojis
        );
    }

    #[test]
    fn clear_search() {
        let mut eu = EmojiUtil::new(Language::En);
        let orig_clone = eu.current_emojis.clone();
        eu.search(String::from("thumb"));
        assert_eq!(
            Some(vec![
                &EmojiData {
                    emoji: "👍",
                    tags: &[]
                },
                &EmojiData {
                    emoji: "👎",
                    tags: &[]
                }
            ]),
            eu.current_emojis
        );
        eu.clear_search();
        assert_eq!(orig_clone, eu.current_emojis);
    }
}
