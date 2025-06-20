use crate::board::sudoku::Cell;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum SudokuError {
    #[error("Out of Bounds Access at cell {0:?}")]
    OutOfBoundsAccess(Cell),
    #[error("Constraint {0} not Supported by this library")]
    UnsupportedConstraint(String),
}
