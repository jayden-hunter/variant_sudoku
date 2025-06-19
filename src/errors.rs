#[derive(Debug)]
pub enum SudokuError {
    OutOfBoundsAccess(usize, usize),
}
