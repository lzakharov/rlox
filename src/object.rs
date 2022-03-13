use std::fmt;

#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{}", x),
            Object::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}
