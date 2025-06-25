use grid::Grid;
use log::{debug, trace};

use crate::{
    board::{
        constraints::RcConstraint,
        digit::{Candidates, Digit, Symbol},
        solution::{Solution, SolutionString},
    },
    errors::SudokuError,
};
use std::{
    fmt::{self, Debug, Display},
    rc::Rc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

pub(crate) type DidUpdateGrid = bool;
type Board = Grid<Digit>;
type Constraints = Vec<RcConstraint>;

#[derive(Clone)]
pub struct Sudoku {
    pub(crate) board: Board,
    pub(crate) constraints: Constraints,
}

impl Sudoku {
    pub fn solve(&mut self) -> Solution {
        loop {
            let mut did_update = false;
            for constraints in self.constraints.clone() {
                did_update |= constraints.use_strategies(self).unwrap();
            }
            if self.board.iter().all(Digit::is_solved) {
                return Solution::UniqueSolution(self.clone());
            }
            if !did_update {
                return Solution::NoSolution;
            }
        }
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub(crate) fn indexed_candidate_iter(&self) -> impl Iterator<Item = (Cell, &Candidates)> {
        self.board
            .indexed_iter()
            .map(|(cell, digit)| {
                (
                    Cell {
                        row: cell.0,
                        col: cell.1,
                    },
                    digit,
                )
            })
            .filter_map(|(c, d)| d.try_get_candidates().map(|v| (c, v)))
    }

    pub fn to_string_line(&self) -> SolutionString {
        SolutionString(self.board.iter().map(Digit::get_char).collect())
    }

    pub(crate) fn valid_symbols() -> Candidates {
        vec![
            Symbol('1'),
            Symbol('2'),
            Symbol('3'),
            Symbol('4'),
            Symbol('5'),
            Symbol('6'),
            Symbol('7'),
            Symbol('8'),
            Symbol('9'),
        ]
    }

    pub(crate) fn new(givens: Grid<Option<Symbol>>, constraints: Constraints) -> Self {
        let (rows, cols) = givens.size();
        let board = Grid::init(rows, cols, Digit(Self::valid_symbols()));
        let mut sudoku = Sudoku { board, constraints };
        for (cell, symbol) in givens
            .indexed_iter()
            .filter_map(|(c, f)| f.as_ref().map(|v| (Cell { row: c.0, col: c.1 }, v)))
        {
            sudoku.place_digit(&cell, symbol).unwrap();
        }
        sudoku
    }

    /// Makes the Symbol the only one in that cell.
    pub fn place_digit(
        &mut self,
        cell: &Cell,
        symbol: &Symbol,
    ) -> Result<DidUpdateGrid, SudokuError> {
        let before = self.get_cell(cell)?;
        let digit = Digit(vec![*symbol]);
        if *before == digit {
            return Ok(false);
        }
        debug!("Placing Symbol {symbol:?} in {cell:?} -> Beforehand is {before:?} (Entropy is now {:.2})", self.get_entropy());
        *self.get_cell_mut(cell)? = digit.clone();
        self.notify(cell)
    }

    /// Removes the candidate as an option from that cell.
    pub fn remove_candidate(
        &mut self,
        cell: &Cell,
        symbol_to_remove: &Symbol,
    ) -> Result<DidUpdateGrid, SudokuError> {
        trace!("Removing {symbol_to_remove:?} from {cell:?}");
        let cell_mut = self.get_cell_mut(cell)?;
        if !cell_mut.0.contains(symbol_to_remove) {
            return Ok(false);
        }
        cell_mut.0.retain(|f| f != symbol_to_remove);
        let candidates_left = &self.get_cell(cell)?.0;
        trace!(
            "Candidates left after removal: {candidates_left:?}, Entropy is now {:.2}",
            self.get_entropy()
        );
        self.notify(cell)
    }

    pub(crate) fn notify(&mut self, cell: &Cell) -> Result<DidUpdateGrid, SudokuError> {
        let mut did_update = false;
        for constraint in self.constraints.clone().iter() {
            did_update |= constraint.notify_update(self, cell)?;
        }
        Ok(did_update)
    }

    pub(crate) fn get_entropy(&self) -> f64 {
        let maximum = Self::valid_symbols().len() * self.board.rows() * self.board.cols();
        let mut candidate_count = 0;
        for (_, d) in self.indexed_iter() {
            candidate_count += d.0.len() - 1;
        }
        candidate_count as f64 / maximum as f64
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
        writeln!(f, "Entropy: {:.2}", self.get_entropy())?;
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
