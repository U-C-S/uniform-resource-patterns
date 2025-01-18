mod parser;
mod primitives;

use parser::Parser;
use regex::Regex;

pub fn is_match(haystack: &str, glob: &str) -> bool {
    let mut parser = Parser::new(glob);
    let regex_str = parser.to_regex();
    let regex = Regex::new(&regex_str).unwrap();
    regex.is_match(haystack)
}

pub fn to_regex(glob: &str) -> String {
    let mut parser = Parser::new(glob);
    parser.to_regex()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_exp() {
        // let result = is_match("world-big-cat", String::from("world-*-cat"));
        // assert_eq!(result, true);

        let test_gen_map = [
            ["world-big-cat", "world-*-cat"],
            ["/meow/h/ja/ddd/ada/dad", "/**"],
        ];

        for [sample, pattern] in test_gen_map {
            assert_eq!(is_match(sample, pattern), true)
        }
    }

    #[test]
    fn recursive_regex_comp() {
        // assert_eq!(to_regex("/**".to_string()), String::from("^/(?:.*/)*$"));
        let regex = Regex::new("^/(?:.*/)*$").unwrap();
        assert_eq!(regex.is_match("/meow/h/ja/ddd/ada/"), true)
    }

    #[test]
    fn escape_char_test() {
        assert_eq!(to_regex(r"meow\?"), String::from("^meow?$"));

        // assert_eq!("meow\\?".len(), 7)
    }

    #[test]
    fn test_range_parsing() {
        assert_eq!(to_regex("[a-z]*"), String::from("^[a-z].*$"));

        assert_eq!(to_regex("[0-9]?"), String::from("^[0-9].$"));

        assert_eq!(to_regex("file[abc].txt"), String::from("^file[abc].txt$"));

        // Malformed range should panic
        let result = std::panic::catch_unwind(|| to_regex("[a-z"));
        assert!(result.is_err());
    }

    #[test]
    fn test_lists() {
        assert_eq!(
            to_regex("{super,spider,iron}man"),
            "^(?:super|spider|iron)man$"
        );
        assert_eq!(is_match("superman", "{super,spider,iron}man$"), true)
    }
}
