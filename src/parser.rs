use std::cell::Cell;

use crate::primitives::{Primitive, AST};

pub struct Parser {
    pos: Cell<usize>,
    glob_pattern: String,
    ast: AST,
}

impl Parser {
    pub fn new(glob: &str) -> Parser {
        Parser {
            glob_pattern: glob.to_string(),
            pos: Cell::new(0),
            ast: vec![],
        }
    }

    pub fn generate_ast(&mut self) -> &AST {
        self.parse();
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
}
