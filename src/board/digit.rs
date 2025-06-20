use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Digit {
    Symbol(Symbol),
    Candidates(Vec<Symbol>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Symbol(pub(crate) char);

impl Symbol {
    pub fn get_number(&self) -> Option<u8> {
        self.0.to_digit(10).map(|c| c as u8)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Digit::Symbol(s) => s.fmt(f),
            Digit::Candidates(_) => write!(f, " "),
        }
    }
}
