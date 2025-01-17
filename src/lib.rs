use std::cell::Cell;

use regex::Regex;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn is_match(haystack: &str, glob: String) -> bool {
    let parser = Parser::new(glob);
    let regex = parser.to_regex().unwrap();
    regex.is_match(haystack)
}

enum Primitive {
    Literal(String),   // a
    Any,               // *
    Recursive,         // **
    Single,            // ?
    List(Vec<String>), // [ ]
    Range(String),     // { }
    Seperator,         // /
}

// struct Span {
//     pub start: u32,
//     pub end: u32,
// }

struct Parser {
    // start: Cell<usize>,
    current: Cell<usize>,
    source: String,
}

impl Parser {
    pub fn new(glob: String) -> Parser {
        Parser {
            source: glob,
            current: Cell::new(0),
        }
    }

    pub fn to_regex(&self) -> Result<Regex, regex::Error> {
        // https://{meow,purr}.cat.com
        // (meow|purr)\.cat\.com - valid regex
        // let list_regex = Regex::new(r"\{(?<middle>.*)\}").unwrap();
        let mut ast: Vec<Primitive> = vec![];

        loop {
            if self.is_eol() {
                break;
            }

            let c = self.peek();

            match c {
                '{' => {}
                '}' => {}
                '[' => {}
                '*' => {
                    ast.push(Primitive::Any);
                }
                '?' => {
                    ast.push(Primitive::Single);
                }
                _ => {
                    // if the previous AST is a literal, then we can combine them
                    if let Some(Primitive::Literal(literal)) = ast.last() {
                        let new_ast = Primitive::Literal(format!("{}{}", literal, c));
                        ast.pop();
                        ast.push(new_ast);
                    } else {
                        // otherwise, we just add the literal
                        ast.push(Primitive::Literal(c.to_string()));
                    }
                }
            }

            self.advance();
        }

        let regex = regex_generator(&ast);
        Regex::new(&regex)
    }

    fn is_eol(&self) -> bool {
        self.current.get() >= self.source.len()
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current.get()).unwrap()
    }

    fn advance(&self) {
        self.current.set(self.current.get() + 1);
    }
}

fn regex_generator(ast: &Vec<Primitive>) -> String {
    let mut regex_str = String::new();

    regex_str.push('^');
    for primitive in ast {
        match primitive {
            Primitive::Single => {
                regex_str.push('.');
            }
            Primitive::Any => {
                regex_str.push_str(".*");
            }
            Primitive::Literal(str) => {
                regex_str.push_str(str);
            }
            _ => todo!("To be implemented!"),
        }
    }
    regex_str.push('$');

    regex_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_match("world-big-cat", String::from("world-*-cat"));
        assert_eq!(result, true);
    }
}
