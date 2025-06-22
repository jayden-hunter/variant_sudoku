pub(crate) mod standard;

use std::{any::Any, rc::Rc};

use crate::{
    board::{digit::Digit, sudoku::Cell},
    errors::SudokuError,
    Sudoku,
};

pub trait Constraint: Any {
    // fn is_satisfied(&self, sudoku: &Sudoku) -> bool;

    fn filter_cell_candidates(&self, sudoku: &mut Sudoku, cell: &Cell) -> Result<(), SudokuError>;

    fn propogate_change(
        &self,
        sudoku: &mut Sudoku,
        cell: &Cell,
        digit: &Digit,
    ) -> Result<(), SudokuError>;

    fn as_any(&self) -> &dyn Any;

    fn use_strategies(&self, sudoku: &mut Sudoku) -> Result<(), SudokuError>;
}

pub type RcConstraint = Rc<dyn Constraint>;

pub(crate) fn combine_constraints(
    constraints: &[RcConstraint],
    sudoku: &mut Sudoku,
    cell: &Cell,
) -> Result<(), SudokuError> {
    if sudoku.get_cell(cell)?.is_solved() {
        return Ok(());
    }
    for constraint in constraints {
        constraint.filter_cell_candidates(sudoku, cell)?
    }
    Ok(())
}
