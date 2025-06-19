use grid::Grid;

use crate::{
    board::{constraints::base::RcConstraint, digit::Digit},
    errors::SudokuError,
    Constraint,
};
use std::{fmt, rc::Rc};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct RowIndex(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct ColIndex(usize);

pub type Cell = (usize, usize);

#[derive(Clone)]
pub struct Sudoku {
    pub board: Grid<Digit>,
    pub(crate) constraints: Vec<RcConstraint>,
}

impl Sudoku {
    pub fn solve(&self) -> Sudoku {
        // For each cell in the grid, see if there is the cell has only one candidate.
        let mut next_board = self.board.clone();
        for ((row, col), digit) in self.board.indexed_iter() {
            if digit.get_number().is_none() {
                // Get candidates for the cell
                let candidates = self
                    .constraints
                    .iter()
                    .flat_map(|constraint| constraint.get_cell_candidates(&self, row, col))
                    .collect::<Vec<Digit>>();

                // If there is only one candidate, set it
                if candidates.len() == 1 {
                    *next_board.get_mut(row, col).unwrap() = candidates[0];
                    let next_sudoku_state = Sudoku {
                        board: next_board,
                        constraints: self.constraints.clone(),
                    };
                    return next_sudoku_state.solve();
                }
            }
        }
        self.clone()
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Result<&Digit, SudokuError> {
        self.board
            .get(row, col)
            .ok_or(SudokuError::OutOfBoundsAccess(row, col))
    }

    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> Result<&mut Digit, SudokuError> {
        self.board
            .get_mut(row, col)
            .ok_or(SudokuError::OutOfBoundsAccess(row, col))
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let thin = f.alternate();
        for (i, row) in self.board.iter_rows().enumerate() {
            if !thin && i % 3 == 0 && i != 0 {
                writeln!(f, "------+-------+------")?;
            }
            for (j, digit) in row.enumerate() {
                if !thin {
                    if j % 3 == 0 && j != 0 {
                        write!(f, "| ")?;
                    }
                    write!(f, "{} ", digit)?;
                } else {
                    write!(f, "{}", digit)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "Constraints: {}", self.constraints.len())?;
        Ok(())
    }
}
