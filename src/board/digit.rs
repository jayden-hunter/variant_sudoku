use std::fmt::Display;

pub type Candidates = Vec<Symbol>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Digit {
    Symbol(Symbol),
    Candidates(Candidates),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Symbol(pub(crate) char);

impl Symbol {
    #[allow(dead_code)]
    pub fn get_number(&self) -> Option<u8> {
        self.0.to_digit(10).map(|c| c as u8)
    }

    pub fn from_num(num: u8) -> Self {
        let c = num as char;
        Self(c)
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

impl Digit {
    pub(crate) fn try_candidates(&self) -> Option<&Candidates> {
        match self {
            Digit::Symbol(_) => None,
            Digit::Candidates(symbols) => Some(symbols),
        }
    }
}

pub(crate) fn intersect_candidates(c: Vec<&Candidates>) -> Candidates {
    if c.is_empty() {
        return Vec::new();
    }

    let first = c[0];
    first
        .iter()
        .cloned()
        .filter(|sym| c.iter().all(|cand| cand.contains(sym)))
        .collect()
}
