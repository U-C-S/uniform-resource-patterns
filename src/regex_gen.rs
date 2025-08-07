use regex::Regex;

use crate::{
    parser::Parser,
    primitives::{Primitive, AST},
};

fn regex_generator(ast: &AST) -> String {
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
            Primitive::Delimiter(_delimiter) => todo!(),
        }
    }
    regex_str.push('$');

    regex_str
}

pub fn to_regex_str(glob: &str) -> String {
    let mut parser = Parser::new(glob);
    let ast = parser.generate_ast();
    regex_generator(ast)
}

pub fn to_regex(glob: &str) -> Regex {
    let regex_str = to_regex_str(glob);
    Regex::new(&regex_str).unwrap()
}
