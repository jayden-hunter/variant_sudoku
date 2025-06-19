use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Digit {
    #[default]
    Blank,
    Number(u8),
    Symbol(char),
}

impl Digit {
    pub fn get_number(&self) -> Option<u8> {
        match self {
            Digit::Number(n) => Some(*n),
            _ => None,
        }
    }
}

impl From<char> for Digit {
    fn from(c: char) -> Self {
        if c.is_ascii_digit() {
            Digit::Number(c.to_digit(10).unwrap() as u8)
        } else if c.is_ascii_alphabetic() {
            Digit::Symbol(c)
        } else {
            Digit::Blank
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Digit::Blank => write!(f, "."),
            Digit::Number(n) => write!(f, "{}", n),
            Digit::Symbol(c) => write!(f, "{}", c),
        }
    }
}
