use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RujeAtom {
    Symbol(String),
    Keyword(String),
    Int(i64),
    Float(f64),
    String(String),
    Character(char),
}

impl fmt::Display for RujeAtom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RujeAtom::Symbol(value) => write!(f, "{}", value),
            RujeAtom::Keyword(value) => write!(f, ":{}", value),
            RujeAtom::Int(value) => write!(f, "{}", value),
            RujeAtom::Float(value) => write!(f, "{}", value),
            RujeAtom::String(value) => write!(f, "\"{}\"", value),
            RujeAtom::Character(value) => write!(f, "'{}'", value),
        }
    }
}

impl RujeAtom {
    pub fn new_symbol<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        RujeAtom::Symbol(value.into())
    }

    pub fn new_keyword<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        RujeAtom::Keyword(value.into())
    }
}
