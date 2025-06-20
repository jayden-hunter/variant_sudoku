pub(crate) mod standard;

use std::rc::Rc;

use crate::{
    board::{digit::Digit, sudoku::Cell},
    errors::SudokuError,
    Sudoku,
};

pub trait Constraint {
    fn is_satisfied(&self, sudoku: &Sudoku) -> bool;

    fn get_cell_candidates(&self, sudoku: &Sudoku, cell: &Cell) -> Result<Vec<Digit>, SudokuError>;
}

pub type RcConstraint = Rc<dyn Constraint>;

pub(crate) fn combine_candidates(
    constraints: &[RcConstraint],
    sudoku: &Sudoku,
    cell: &Cell,
) -> Result<Vec<Digit>, SudokuError> {
    if !matches!(sudoku.get_cell(cell)?, Digit::Blank) {
        return Ok(vec![]);
    }
    constraints
        .iter()
        .map(|constraint| constraint.get_cell_candidates(sudoku, cell))
        .try_fold(None, |acc: Option<Vec<Digit>>, x| {
            let x = x?;
            Ok(match acc {
                Some(acc) => Some(acc.into_iter().filter(|d| x.contains(d)).collect()),
                None => Some(x),
            })
        })
        .map(|opt| opt.unwrap_or_default())
}
