use regex::Regex;

use crate::{
    parser::Parser,
    primitives::{Delimiter, Primitive, AST},
};

fn regex_generator(ast: &AST, ignore_start_end: bool) -> String {
    let mut regex_str = String::new();

    if !ignore_start_end {
        regex_str.push('^');
    }

    for primitive in ast {
        match primitive {
            Primitive::Single => {
                regex_str.push('.');
            }
            Primitive::Any => {
                regex_str.push_str(".*");
            }
            Primitive::Recursive => {
                regex_str.push_str("(?:.*/)*[^/]*");
            }
            Primitive::Literal(str) => {
                regex_str.push_str(&str);
            }
            Primitive::Range(range) => {
                regex_str.push('[');
                regex_str.push_str(range);
                regex_str.push(']');
            }
            Primitive::List(list) => {
                regex_str.push_str("(?:");
                // regex_str.push_str(&list.join("|"));
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        regex_str.push('|');
                    }
                    regex_str.push_str(&regex_generator(item, true));
                }
                regex_str.push(')');
            }
            Primitive::Delimiter(delimiter) => match delimiter {
                Delimiter::SCHEME_PATH => regex_str.push(':'),
                Delimiter::SCHEME_AUTHORITY => regex_str.push_str("://"),
                Delimiter::PATH => regex_str.push('/'),
                Delimiter::PRE_QUERY => regex_str.push_str(r"\?"),
                Delimiter::QUERY => regex_str.push('&'),
                Delimiter::PRE_FRAGMENT => regex_str.push('#'),
            },
        }
    }

    if !ignore_start_end {
        regex_str.push('$');
    }

    regex_str
}

pub fn to_regex_str(glob: &str) -> String {
    let mut parser = Parser::new(glob);
    parser.generate_ast();
    regex_generator(parser.ast(), false)
}

pub fn to_regex(glob: &str) -> Regex {
    let regex_str = to_regex_str(glob);
    Regex::new(&regex_str).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_regex() {
        let glob = "http://example.com/{a,b,c}/path\\?query=value#fragment";
        let regex = to_regex(glob);
        assert!(regex.is_match("http://example.com/a/path?query=value#fragment"));
        assert!(regex.is_match("http://example.com/b/path?query=value#fragment"));
        assert!(regex.is_match("http://example.com/c/path?query=value#fragment"));
        assert!(!regex.is_match("http://example.com/d/path?query=value#fragment"));

        let regex_str = to_regex_str(glob);
        assert_eq!(
            regex_str,
            "^http://example.com/(?:a|b|c)/path\\?query=value#fragment$"
        );
    }
}
