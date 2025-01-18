pub type AST = Vec<Primitive>;

pub enum Primitive {
    Literal(String),   // a
    Any,               // *
    Recursive,         // **
    Single,            // ?
    List(Vec<String>), // { }
    Range(String),     // [ ]
}
