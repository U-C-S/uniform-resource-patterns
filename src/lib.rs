use std::cell::Cell;

use regex::Regex;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn is_match(haystack: &str, glob: String) -> bool {
    let mut parser = Parser::new(glob);
    let regex_str = parser.to_regex();
    let regex = Regex::new(&regex_str).unwrap();
    regex.is_match(haystack)
}

pub fn to_regex(glob: String) -> String {
    let mut parser = Parser::new(glob);
    parser.to_regex()
}

enum Primitive {
    Literal(String), // a
    Any,             // *
    Recursive,       // **
    Single,          // ?
                     // List(Vec<ListPrimitive>), // { }
                     // Range(String),            // [ ]
                     // Seperator,                // /
}

// enum ListPrimitive {
//     Literal(String), // a
//     Any,             // *
//     Recursive,       // **
//     Single,          // ?
//     Seperator,       // /
// }

// struct Span {
//     pub start: u32,
//     pub end: u32,
// }

struct Parser {
    // start: Cell<usize>,
    current: Cell<usize>,
    source: String,
    ast: Vec<Primitive>,
}

impl Parser {
    pub fn new(glob: String) -> Parser {
        Parser {
            source: glob,
            current: Cell::new(0),
            ast: vec![],
        }
    }

    fn is_eol(&self) -> bool {
        self.current.get() >= self.source.len()
    }

    fn char(&self) -> char {
        self.source.chars().nth(self.current.get()).unwrap()
    }

    fn advance(&self) {
        self.current.set(self.current.get() + 1);
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current.get() + 1).unwrap()
    }

    pub fn to_regex(&mut self) -> String {
        // https://{meow,purr}.cat.com
        // (meow|purr)\.cat\.com - valid regex
        // let list_regex = Regex::new(r"\{(?<middle>.*)\}").unwrap();

        loop {
            if self.is_eol() {
                break;
            }

            match self.char() {
                '\\' => {
                    self.advance();
                    self.parse_literal();
                }
                '{' => self.parse_group(),
                '*' => {
                    if self.peek() == '*' {
                        self.advance();
                        self.ast.push(Primitive::Recursive);
                    } else {
                        self.ast.push(Primitive::Any);
                    }
                }
                '?' => {
                    self.ast.push(Primitive::Single);
                }
                _ => self.parse_literal(),
            }

            self.advance();
        }

        self.regex_generator()
    }

    fn parse_literal(&mut self) {
        let c = self.char();
        // if the previous AST is a literal, then we can combine them
        if let Some(Primitive::Literal(literal)) = self.ast.last() {
            let new_ast = Primitive::Literal(format!("{}{}", literal, c));
            self.ast.pop();
            self.ast.push(new_ast);
        } else {
            // otherwise, we just add the literal
            self.ast.push(Primitive::Literal(c.to_string()));
        }
    }

    fn parse_group(&self) {}

    fn regex_generator(&self) -> String {
        let mut regex_str = String::new();

        regex_str.push('^');
        for primitive in &self.ast {
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
                // Primitive::Seperator => {
                //     regex_str.push_str(&str);
                // }
                _ => todo!("To be implemented"),
            }
        }
        regex_str.push('$');

        regex_str
    }
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
            assert_eq!(is_match(sample, String::from(pattern)), true)
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
        assert_eq!(to_regex(r"meow\?".to_string()), String::from("^meow?$"));

        // assert_eq!("meow\\?".len(), 7)
    }
}
