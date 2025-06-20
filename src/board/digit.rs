use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellData {
    Digit(Digit),
    Candidates(Vec<Digit>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Digit(pub(crate) char);

impl Digit {
    pub fn get_number(&self) -> Option<u8> {
        self.0.to_digit(10).map(|c| c as u8)
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for CellData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellData::Digit(digit) => digit.fmt(f),
            CellData::Candidates(_) => write!(f, " "),
        }
    }
}
