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

    fn propogate_change(
        &self,
        sudoku: &mut Sudoku,
        _cell: &Cell,
        _digit: &Digit,
    ) -> Result<(), SudokuError> {
        // Collect cells first to avoid borrow checker issues
        let cells: Vec<Cell> = sudoku.indexed_iter().map(|(cell, _)| cell).collect();
        for cell in cells {
            self.filter_cell_candidates(sudoku, &cell)?
        }
        Ok(())
    }
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
