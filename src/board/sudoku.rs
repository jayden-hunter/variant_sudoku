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
    collections::{HashSet}, fmt::{self, Debug, Display}, rc::Rc
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
    board: Board,
    valid_symbols: HashSet<Symbol>,
    pub(crate) constraints: Constraints,
}

impl Sudoku {
    pub fn solve(&mut self) -> Solution {
        loop {
            let mut did_update = false;
            for constraint in self.constraints.clone() {
                did_update |= constraint.use_strategies(self).unwrap();
                if self.is_solved() {
                    return Solution::UniqueSolution(self.clone())
                }
            }
            if !did_update {
                return Solution::NoSolution;
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        self.board.iter().all(Digit::is_solved)
    }

    pub(crate) fn size(&self) -> (usize, usize) {
        (self.board.rows(), self.board.cols())
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

    pub(crate) fn new(givens: Grid<Option<Symbol>>, constraints: Constraints) -> Self {
        let distinct_symbols = givens.rows().max(givens.cols());
        let mut valid_symbols: HashSet<Symbol> = givens.iter().filter_map(|f| *f).collect();
        let remaining_options = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars().into_iter().map(|f| Symbol(f));
        for i in remaining_options {
            if valid_symbols.len() == distinct_symbols {
                break;
            }
            valid_symbols.insert(i);
        }
        Self::new_with_valid_digits(givens, constraints, valid_symbols)
    }

    pub(crate) fn new_with_valid_digits(givens: Grid<Option<Symbol>>, constraints: Constraints, valid_symbols: HashSet<Symbol>) -> Self {
        debug!("Givens {givens:?}");
        let (rows, cols) = givens.size();
        let all_symbols_digit = Digit(valid_symbols.iter().cloned().collect());
        let board = Grid::init(rows, cols,all_symbols_digit);
        let mut sudoku = Sudoku { board, valid_symbols: valid_symbols, constraints };
        debug!("New Sudoku created, with size {:?}, and valid symbols: {:?}", sudoku.size(), sudoku.valid_symbols);
        for (cell, symbol) in givens
            .indexed_iter()
            .filter_map(|(c, f)| f.as_ref().map(|v| (Cell { row: c.0, col: c.1 }, v)))
        {
            sudoku.place_digit(&cell, symbol);
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
        debug!("Placing {symbol:?} in {cell:?} -> Beforehand is {before:?} (Entropy is now {:.2})", self.get_entropy());
        *self.get_cell_mut(cell)? = digit.clone();
        self.notify(cell)?;
        return Ok(true)
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
        self.notify(cell)?;
        Ok(true)
    }

    // Keeps the candidate as an option from that cell (similar to an intersection)
    pub fn keep_candidates(
        &mut self,
        cell: &Cell,
        symbols_to_keep: &Candidates,
    ) -> Result<DidUpdateGrid, SudokuError> {
        let cell_mut = self.get_cell_mut(cell)?;
        let before = cell_mut.clone();
        debug!("Keeping Only {symbols_to_keep:?} from {cell:?}. Before {before:?}");
        cell_mut.0.retain(|f| symbols_to_keep.contains(f));
        if before == *cell_mut {
            return Ok(false);
        }
        self.notify(cell)?;
        Ok(true)
    }

    pub(crate) fn notify(&mut self, cell: &Cell) -> Result<DidUpdateGrid, SudokuError> {
        let mut did_update = false;
        for constraint in self.constraints.clone().iter() {
            did_update |= constraint.notify_update(self, cell)?;
        }
        Ok(did_update)
    }

    pub(crate) fn get_entropy(&self) -> f64 {
        let maximum = self.valid_symbols.len() * self.board.rows() * self.board.cols();
        let mut candidate_count = 0;
        for (_, d) in self.indexed_iter() {
            if d.0.is_empty() {
                return -1.0
            }
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
