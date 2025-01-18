mod parser;
mod primitives;

use parser::Parser;
use regex::Regex;

pub fn is_match(test_string: &str, glob: &str) -> bool {
    let mut parser = Parser::new(glob);
    let regex_str = parser.to_regex();
    let regex = Regex::new(&regex_str).unwrap();
    regex.is_match(test_string)
}

pub fn to_regex(glob: &str) -> String {
    let mut parser = Parser::new(glob);
    parser.to_regex()
}
