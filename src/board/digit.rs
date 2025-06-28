use std::fmt::{Debug, Display};

pub type Candidates = Vec<Symbol>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Digit(pub Candidates);

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Debug)]
pub struct Symbol(pub char);

impl Symbol {
    pub fn get_number(&self) -> Option<u32> {
        self.0.to_digit(10)
    }

    #[allow(dead_code)]
    pub fn from_num(num: u8) -> Self {
        let c = num as char;
        Self(c)
    }
}

impl Digit {
    #[allow(dead_code)]
    pub(crate) fn try_candidates(&self) -> Option<&Candidates> {
        match self.0.len() {
            1 => None,
            _ => Some(&self.0),
        }
    }

    pub(crate) fn is_solved(&self) -> bool {
        self.0.len() == 1
    }

    pub(crate) fn try_get_solved(&self) -> Option<&Symbol> {
        if self.is_solved() {
            return self.0.first();
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn try_get_candidates(&self) -> Option<&Candidates> {
        if self.is_solved() {
            return None;
        }
        Some(&self.0)
    }

    pub(crate) fn get_char(&self) -> char {
        match self.try_get_solved() {
            Some(s) => s.0,
            None => '.',
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(dead_code)]
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
