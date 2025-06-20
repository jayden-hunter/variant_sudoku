use grid::Grid;

use crate::{
    board::{
        constraints::RcConstraint,
        digit::Digit,
        solution::{Solution, SolutionString},
        solver::ALL_STRATEGIES,
    },
    errors::SudokuError,
};
use std::{
    fmt::{self, Debug, Display},
    rc::Rc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Cell {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone)]
pub struct Sudoku {
    pub(crate) board: Grid<Digit>,
    pub(crate) constraints: Vec<RcConstraint>,
}

impl Sudoku {
    pub fn solve(&self) -> Solution {
        if self.board.iter().all(|d| !matches!(d, Digit::Blank)) {
            return Solution::UniqueSolution(self.clone());
        }
        for (strategy, _difficulty) in ALL_STRATEGIES {
            if let Some((cell, digit)) = strategy(self) {
                let mut next_board = self.clone();
                *next_board.get_cell_mut(&cell).unwrap() = digit;
                return next_board.solve();
            }
        }
        Solution::NoSolution
    }

    pub(crate) fn get_cell(&self, cell: &Cell) -> Result<&Digit, SudokuError> {
        self.board
            .get(cell.row, cell.col)
            .ok_or(SudokuError::OutOfBoundsAccess(*cell))
    }

    pub(crate) fn get_cell_mut(&mut self, cell: &Cell) -> Result<&mut Digit, SudokuError> {
        self.board
            .get_mut(cell.row, cell.col)
            .ok_or(SudokuError::OutOfBoundsAccess(*cell))
    }

    pub(crate) fn indexed_iter(&self) -> impl Iterator<Item = (Cell, &Digit)> {
        self.board.indexed_iter().map(|(cell, digit)| {
            (
                Cell {
                    row: cell.0,
                    col: cell.1,
                },
                digit,
            )
        })
    }

    pub fn to_string_line(&self) -> SolutionString {
        SolutionString(
            self.board
                .iter()
                .map(Digit::to_string)
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            writeln!(f, "{}", self.to_string_line())?;
            return Ok(());
        }
        for (i, row) in self.board.iter_rows().enumerate() {
            if i % 3 == 0 && i != 0 {
                writeln!(f, "------+-------+------")?;
            }
            for (j, digit) in row.enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "| ")?;
                }
                write!(f, "{} ", digit)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Constraints: {}", self.constraints.len())?;
        Ok(())
    }
}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
            && self.constraints.len() == other.constraints.len()
            && self
                .constraints
                .iter()
                .zip(other.constraints.iter())
                .all(|(a, b)| Rc::ptr_eq(a, b))
    }
}

impl Eq for Sudoku {}

impl Debug for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sudoku")
            .field("board", &self.board)
            .field("constraint_count", &self.constraints.len())
            .finish()
    }
}
