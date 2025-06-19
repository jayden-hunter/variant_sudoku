use crate::{board::digit::Digit, Sudoku};

pub trait Constraint {
    fn is_satisfied(&self, sudoku: &Sudoku) -> bool;

    fn get_cell_candidates(&self, sudoku: &Sudoku, row: usize, col: usize) -> Vec<Digit>;
}
