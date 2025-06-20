pub(crate) mod standard;

use std::rc::Rc;

use crate::{
    board::{digit::Digit, sudoku::Cell},
    errors::SudokuError,
    Sudoku,
};

pub trait Constraint {
    // fn is_satisfied(&self, sudoku: &Sudoku) -> bool;

    fn filter_cell_candidates(&self, sudoku: &mut Sudoku, cell: &Cell) -> Result<(), SudokuError>;
}

pub type RcConstraint = Rc<dyn Constraint>;

pub(crate) fn combine_candidates(
    constraints: &[RcConstraint],
    sudoku: &mut Sudoku,
    cell: &Cell,
) -> Result<(), SudokuError> {
    if matches!(sudoku.get_cell(cell)?, Digit::Symbol(_)) {
        return Ok(());
    }
    for constraint in constraints {
        constraint.filter_cell_candidates(sudoku, cell)?
    }
    Ok(())
}
