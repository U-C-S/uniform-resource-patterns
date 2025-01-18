use regex::Regex;
use std::cell::Cell;

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

type AST = Vec<Primitive>;

enum Primitive {
    Literal(String),   // a
    Any,               // *
    Recursive,         // **
    Single,            // ?
    List(Vec<String>), // { }
    Range(String),     // [ ]
}

struct Parser {
    current: Cell<usize>,
    source: String,
    ast: AST,
}

impl Parser {
    pub fn new(glob: &str) -> Parser {
        Parser {
            source: glob.to_string(),
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

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current.get() + 1)
    }

    fn parse(&mut self) {
        loop {
            if self.is_eol() {
                break;
            }

            match self.char() {
                '\\' => {
                    self.advance();
                    self.parse_literal();
                }
                '{' => self.parse_list(),
                '[' => self.parse_range(),
                '*' => {
                    if self.peek() == Some('*') {
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
    }

    pub fn to_regex(&mut self) -> String {
        // https://{meow,purr}.cat.com
        // (meow|purr)\.cat\.com - valid regex
        // let list_regex = Regex::new(r"\{(?<middle>.*)\}").unwrap();
        self.parse();
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

    fn parse_range(&mut self) {
        self.advance(); // Move past the `[` character

        let mut range = String::new();
        let mut is_valid = false;

        while !self.is_eol() {
            if let ']' = self.char() {
                is_valid = true;
                break;
            } else {
                range.push(self.char());
            }
            self.advance();
        }

        if is_valid {
            self.ast.push(Primitive::Range(range));
        } else {
            panic!("Malformed range: missing closing `]`");
        }
    }

    fn parse_list(&mut self) {
        self.advance();

        let mut list: Vec<String> = vec![];
        let mut is_valid = false;
        let mut current_item = String::new();

        loop {
            if self.is_eol() {
                break;
            }

            match self.char() {
                ',' => {
                    if !current_item.is_empty() {
                        list.push(current_item);
                        current_item = String::new();
                    }
                }
                '}' => {
                    if !current_item.is_empty() {
                        list.push(current_item);
                    }
                    is_valid = true;
                    break;
                }
                c => current_item = format!("{}{}", current_item, c),
            }

            self.advance();
        }

        if is_valid {
            self.ast.push(Primitive::List(list));
        } else {
            panic!("Malformed range: missing closing `]`");
        }
    }

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
                Primitive::Range(range) => {
                    regex_str.push('[');
                    regex_str.push_str(range);
                    regex_str.push(']');
                }
                Primitive::List(list) => {
                    regex_str.push_str("(?:");
                    regex_str.push_str(&list.join("|"));
                    regex_str.push(')');
                }
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
