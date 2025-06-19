mod board;
pub(crate) use board::constraints::base::Constraint;
pub use board::sudoku::Sudoku;
pub use errors::SudokuError;
mod errors;
