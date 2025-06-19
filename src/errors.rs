use std::fmt;

#[derive(Debug)]
pub enum SudokuError {
    OutOfBoundsAccess(usize, usize),
    UnsupportedConstraint(String),
}

// Implement Display for SudokuError so it can be used with serde::de::Error::custom
impl fmt::Display for SudokuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
