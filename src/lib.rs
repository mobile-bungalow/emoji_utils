pub mod picker_struct;
extern crate regex;
use picker_struct::{EmojiData, EMOJI_DATA};
use regex::{escape, Regex};

// Search The emoji structure
pub fn emoji_search(search_string: String) -> Option<Vec<&'static str>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn emoji_search_test() {
        println!("{:?}", emoji_search("green".to_string()));
    }
}
