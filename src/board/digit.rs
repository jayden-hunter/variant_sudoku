#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Digit {
    #[default]
    Blank,
    Number(u8),
    Symbol(char),
}

impl From<char> for Digit {
    fn from(c: char) -> Self {
        if c.is_digit(10) {
            Digit::Number(c.to_digit(10).unwrap() as u8)
        } else if c.is_alphabetic() {
            Digit::Symbol(c)
        } else {
            Digit::Blank
        }
    }
}
