pub type AST = Vec<Primitive>;

#[derive(PartialEq, Eq, Debug)]
pub enum Primitive {
    Literal(String),   // a
    Any,               // *
    Recursive,         // **
    Single,            // ?
    List(Vec<String>), // {a,b,c} allow list of primitives
    Range(String),     // [ ]
    Delimiter(Delimiter),
}

#[derive(PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
pub enum Delimiter {
    /// Delimiter between scheme and Path, i.e. `:` (Colon)
    SCHEME_PATH,
    /// Delimiter between scheme and authority, i.e. `://`
    SCHEME_AUTHORITY,
    /// Path segement delimiter, i.e. `/` (Slash)
    PATH,
    /// Delimiter between intial URI components and query component, i.e `?` (Question mark)
    PRE_QUERY,
    /// Delimiter between each attribute–value pairs in the query component, i.e `&` (Ampersand)
    QUERY,
    /// Delimiter between intial URI components and fragment component, i.e `#` (Hash)
    PRE_FRAGMENT,
}
