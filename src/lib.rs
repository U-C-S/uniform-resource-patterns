use regex::Regex;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn is_match(haystack: &str, glob: &str) {}

fn convert_to_regex(pattern: &str) {
    // https://{meow,purr}.cat.com
    // (meow|purr)\.cat\.com - valid regex
    let list_regex = Regex::new(r"\{(?<middle>.*)\}").unwrap();
    let mut lists = vec![];

    for (_, [middle]) in list_regex.captures_iter(pattern).map(|c| c.extract()) {

    }
}

enum Primitives {
    Empty,
    Literal,
    Any,
    Single,
    List,
    Range,
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
