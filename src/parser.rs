use std::cell::Cell;

use crate::primitives::{Delimiter, Primitive, AST};

#[derive(Debug)]
pub struct Parser {
    glob_pattern: String,

    // State
    pos: Cell<usize>,

    // Outputs
    ast: AST,
    delimiter_order: Vec<Delimiter>,
}

impl Parser {
    pub fn new(glob: &str) -> Parser {
        Parser {
            glob_pattern: glob.to_string(),
            pos: Cell::new(0),
            ast: vec![],
            delimiter_order: vec![],
        }
    }

    pub fn generate_ast(&mut self) {
        self.parse();
    }

    pub fn ast(&self) -> &AST {
        &self.ast
    }
}

impl Parser {
    fn is_eol(&self) -> bool {
        self.pos.get() >= self.glob_pattern.len()
    }

    fn char(&self) -> char {
        self.glob_pattern.chars().nth(self.pos.get()).unwrap()
    }

    fn advance(&self) {
        self.pos.set(self.pos.get() + 1);
    }

    fn peek(&self) -> Option<char> {
        self.glob_pattern.chars().nth(self.pos.get() + 1)
    }

    fn peek_multiple(&self, n: usize) -> Option<&str> {
        let start = self.pos.get() + 1;
        let end = start + n;
        if end <= self.glob_pattern.len() {
            Some(&self.glob_pattern[start..end])
        } else {
            None
        }
    }

    fn parse(&mut self) {
        loop {
            if self.is_eol() {
                break;
            }

            match self.char() {
                '\\' => {
                    self.advance();
                    if self.char() == '?' {
                        self.push_delimiter(Delimiter::PRE_QUERY);
                    } else {
                        self.parse_literal();
                    }
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
                ':' => {
                    if self.peek_multiple(2) == Some("//") {
                        self.push_delimiter(Delimiter::SCHEME_AUTHORITY);
                        self.advance();
                        self.advance();
                    } else {
                        self.push_delimiter(Delimiter::SCHEME_PATH);
                    }
                }
                '/' => self.push_delimiter(Delimiter::PATH),
                '&' => self.push_delimiter(Delimiter::QUERY),
                '#' => self.push_delimiter(Delimiter::PRE_FRAGMENT),
                _ => self.parse_literal(),
            }

            self.advance();
        }
    }

    fn push_delimiter(&mut self, delimiter: Delimiter) {
        self.delimiter_order.push(delimiter.clone());
        self.ast.push(Primitive::Delimiter(delimiter));
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

        let mut patterns_list: Vec<String> = vec![];
        let mut is_valid = false;
        let mut current_item = String::new();

        loop {
            if self.is_eol() {
                break;
            }

            match self.char() {
                ',' => {
                    if !current_item.is_empty() {
                        patterns_list.push(current_item);
                        current_item = String::new();
                    }
                }
                '}' => {
                    if !current_item.is_empty() {
                        patterns_list.push(current_item);
                    }
                    is_valid = true;
                    break;
                }
                c => current_item = format!("{}{}", current_item, c),
            }

            self.advance();
        }

        if is_valid {
            let processed_list: Vec<AST> = patterns_list
                .into_iter()
                .map(|item| {
                    let mut x = Parser::new(&item);
                    x.generate_ast();
                    x.ast
                })
                .collect();
            self.ast.push(Primitive::List(processed_list));
        } else {
            panic!("Malformed range: missing closing `]`");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{Delimiter, Primitive};

    macro_rules! assert_ast_eq {
        ($pattern:expr, $expected:expr) => {{
            let mut parser = Parser::new($pattern);
            parser.generate_ast();
            let ast = parser.ast();
            assert_eq!(&$expected, ast, "\nAST mismatch for pattern: {}", $pattern);
        }};
    }

    #[test]
    fn parses_basic_uri() {
        assert_ast_eq!(
            "http://example.com/{a,b,c}/path\\?query=value#fragment",
            vec![
                Primitive::Literal("http".into()),
                Primitive::Delimiter(Delimiter::SCHEME_AUTHORITY),
                Primitive::Literal("example.com".into()),
                Primitive::Delimiter(Delimiter::PATH),
                Primitive::List(vec![
                    vec![Primitive::Literal("a".into())],
                    vec![Primitive::Literal("b".into())],
                    vec![Primitive::Literal("c".into())]
                ]),
                Primitive::Delimiter(Delimiter::PATH),
                Primitive::Literal("path".into()),
                Primitive::Delimiter(Delimiter::PRE_QUERY),
                Primitive::Literal("query=value".into()),
                Primitive::Delimiter(Delimiter::PRE_FRAGMENT),
                Primitive::Literal("fragment".into()),
            ]
        );
    }

    #[test]
    fn parses_wildcards_and_ranges() {
        assert_ast_eq!(
            "https://*/**/[a-z]\\?file.txt",
            vec![
                Primitive::Literal("https".into()),
                Primitive::Delimiter(Delimiter::SCHEME_AUTHORITY),
                Primitive::Any,
                Primitive::Delimiter(Delimiter::PATH),
                Primitive::Recursive,
                Primitive::Delimiter(Delimiter::PATH),
                Primitive::Range("a-z".into()),
                Primitive::Delimiter(Delimiter::PRE_QUERY),
                Primitive::Literal("file.txt".into()),
            ]
        );
    }

    #[test]
    fn parse_no_path() {
        assert_ast_eq!(
            "{http,https}://example.com",
            vec![
                Primitive::List(vec![
                    vec![Primitive::Literal("http".into())],
                    vec![Primitive::Literal("https".into())]
                ]),
                Primitive::Delimiter(Delimiter::SCHEME_AUTHORITY),
                Primitive::Literal("example.com".into()),
            ]
        );
    }
}
