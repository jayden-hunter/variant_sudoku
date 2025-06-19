use grid::Grid;

use crate::{board::digit::Digit, errors::SudokuError, Constraint};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct RowIndex(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct ColIndex(usize);

pub type Cell = (usize, usize);

pub struct Sudoku {
    pub board: Grid<Digit>,
    pub(crate) constraints: Vec<Box<dyn Constraint>>,
}

impl Sudoku {
    pub fn solve(&self) -> Sudoku {
        unimplemented!()
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
        Ok(())
    }
}
