use std::cell::Cell;

use regex::Regex;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn is_match(haystack: &str, glob: &str) {}

enum Primitive {
    Empty,
    Literal(String),
    Any,
    Single,
    List(Vec<String>),
    Range(String),
}

struct Span {
    pub start: u32,
    pub end: u32,
}

struct Parser {
    pos: Cell<u32>,
}

impl Parser {
    fn to_regex(&self, pattern: &str) {
        // https://{meow,purr}.cat.com
        // (meow|purr)\.cat\.com - valid regex
        let list_regex = Regex::new(r"\{(?<middle>.*)\}").unwrap();
        let mut lists: Vec<Primitive> = vec![];

        loop {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
