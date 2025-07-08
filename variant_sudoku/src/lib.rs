pub mod board;
pub use board::constraints::Constraint;
pub use board::solution::Solution;
pub use board::sudoku::Sudoku;
pub use board::constructor::builder::SudokuBuilder;
use errors::SudokuError;
mod errors;
