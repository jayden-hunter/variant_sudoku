use std::rc::Rc;

use crate::{
    board::{digit::Digit, sudoku},
    Sudoku,
};

pub trait Constraint {
    fn is_satisfied(&self, sudoku: &Sudoku) -> bool;

    fn get_cell_candidates(&self, sudoku: &Sudoku, row: usize, col: usize) -> Vec<Digit>;
}

pub type RcConstraint = Rc<dyn Constraint>;

pub(crate) fn combine_candidates(
    candidates: &Vec<RcConstraint>,
    sudoku: &Sudoku,
    row: usize,
    col: usize,
) -> Vec<Digit> {
    candidates
        .iter()
        .map(|constraint| constraint.get_cell_candidates(sudoku, row, col))
        .fold(None, |acc: Option<Vec<Digit>>, x| match acc {
            Some(acc) => Some(acc.into_iter().filter(|d| x.contains(d)).collect()),
            None => Some(x),
        })
        .unwrap_or_else(Vec::new)
}
