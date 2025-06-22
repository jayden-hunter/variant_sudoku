use log::debug;

use crate::{
    board::{digit::Symbol, sudoku::Cell},
    Sudoku,
};

pub(crate) fn naked_single(sudoku: &Sudoku) -> Option<(Cell, Symbol)> {
    for (cell, candidates) in sudoku.indexed_iter() {
        // If there is only one candidate, set it
        if candidates.0.len() == 1 {
            let digit = candidates.0[0];
            debug!("{:?} solved by naked_single: {:?}", cell, digit);
            return Some((cell, digit));
        }
    }
    None
}
