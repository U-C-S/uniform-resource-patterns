mod parser;
mod primitives;
mod regex_gen;
mod validate;

use regex_gen::to_regex;
pub use regex_gen::to_regex_str;

pub fn is_match(test_string: &str, glob: &str) -> bool {
    let regex = to_regex(glob);
    regex.is_match(test_string)
}
