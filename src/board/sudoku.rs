use grid::Grid;
use log::{debug, trace, warn};

use crate::{
    board::{
        constraints::{
            standard::{get_box_size, HouseSet, HouseUnique},
            RcConstraint,
        },
        digit::{Candidates, Digit, Symbol},
        solution::{Solution, SolutionString},
    },
    errors::SudokuError,
};
use std::{
    collections::HashSet,
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
    board: Board,
    pub(crate) valid_symbols: HashSet<Symbol>,
    pub(crate) constraints: Constraints,
}

impl Sudoku {
    pub fn empty() -> Self {
        let digit = Digit(vec![Symbol('0')]);
        Sudoku {
            board: Grid::init(9, 9, digit),
            valid_symbols: HashSet::new(),
            constraints: Vec::new(),
        }
    }

    pub fn solve(&mut self) -> Result<Solution, SudokuError> {
        loop {
            debug!("Sudoku after iteration: {:?}", self.to_string_line());
            if self.is_solved() {
                return Ok(Solution::UniqueSolution(self.clone()));
            }
            if self.is_unsolveable() {
                debug!(
                    "Unsolveable, here is sudoku at end: {:?}",
                    self.to_string_line()
                );
                return Ok(Solution::NoSolution);
            }
            let mut did_update = false;
            for constraint in self.constraints.clone() {
                did_update |= constraint.use_strategies(self)?;
                if did_update {
                    break;
                }
            }
            if !did_update {
                debug!(
                "No Updates this round, here is sudoku at end: {:?}",
                self.to_string_line()
            );
            return Ok(Solution::NoSolution);
            }

        }
    }

    pub fn is_solved(&self) -> bool {
        self.board.iter().all(Digit::is_solved)
    }

    pub fn is_unsolveable(&self) -> bool {
        self.board.iter().any(|d| d.0.is_empty())
    }

    pub fn size(&self) -> (usize, usize) {
    pub fn size(&self) -> (usize, usize) {
        (self.board.rows(), self.board.cols())
    }

    pub fn get_cell(&self, cell: &Cell) -> Result<&Digit, SudokuError> {
    pub fn get_cell(&self, cell: &Cell) -> Result<&Digit, SudokuError> {
        self.board
            .get(cell.row, cell.col)
            .ok_or(SudokuError::OutOfBoundsAccess(*cell))
    }

    pub fn get_cell_mut(&mut self, cell: &Cell) -> Result<&mut Digit, SudokuError> {
    pub fn get_cell_mut(&mut self, cell: &Cell) -> Result<&mut Digit, SudokuError> {
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
    pub(crate) fn indexed_candidates(&self) -> Vec<(Cell, &Candidates)> {
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
            .collect()
    }

    pub fn to_string_line(&self) -> SolutionString {
        SolutionString(self.board.iter().map(Digit::get_char).collect())
    }

    pub(crate) fn new(givens: Grid<Option<Symbol>>, constraints: Constraints) -> Self {
        let distinct_symbols = givens.rows().max(givens.cols());
        let mut valid_symbols: HashSet<Symbol> = givens.iter().filter_map(|f| *f).collect();
        let remaining_options = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
            .chars()
            .map(Symbol);
        for i in remaining_options {
            if valid_symbols.len() == distinct_symbols {
                break;
            }
            valid_symbols.insert(i);
        }
        Self::new_with_valid_digits(givens, constraints, valid_symbols)
    }

    pub(crate) fn new_with_valid_digits(
        givens: Grid<Option<Symbol>>,
        constraints: Constraints,
        valid_symbols: HashSet<Symbol>,
    ) -> Self {
        debug!("Givens {givens:?}");
        let (rows, cols) = givens.size();
        let all_symbols_digit = Digit(valid_symbols.iter().cloned().collect());
        let board = Grid::init(rows, cols, all_symbols_digit);
        let mut sudoku = Sudoku {
            board,
            valid_symbols,
            constraints,
        };
        debug!(
            "New Sudoku created, with size {:?}, and valid symbols: {:?}",
            sudoku.size(),
            sudoku.valid_symbols
        );
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
        debug!(
            "Placing {symbol:?} in {cell:?} -> Beforehand is {before:?} (Entropy is now {:.2})",
            self.get_entropy()
        );
        *self.get_cell_mut(cell)? = digit.clone();
        self.notify(cell)?;
        Ok(true)
    }

    /// Removes the candidate as an option from that cell.
    pub fn remove_candidate(
        &mut self,
        cell: &Cell,
        symbol_to_remove: &Symbol,
    ) -> Result<DidUpdateGrid, SudokuError> {
        debug!("Removing {symbol_to_remove:#} from {cell:?}");
        let cell_mut = self.get_cell_mut(cell)?;
        // if cell_mut.0.len() == 1 {
        //     warn!("Cannot remove {symbol_to_remove:#} from {cell:?}, it is already solved with {cell_mut:?}");
        //     return Ok(false);
        // }
        if !cell_mut.0.contains(symbol_to_remove) {
            return Ok(false);
        }
        cell_mut.0.retain(|f| f != symbol_to_remove);
        let candidates_left = &self.get_cell(cell)?.0;
        trace!(
            "Candidates left after removal: {candidates_left:?}, Entropy is now {:.2}",
            self.get_entropy()
        );
        if candidates_left.is_empty() {
            warn!("No candidates left in {cell:?}, this is unsolveable");
            return Ok(true);
        }
        
        self.notify(cell)?;
        Ok(true)
    }

    // Keeps the candidate as an option from that cell (similar to an intersection)
    pub fn keep_candidates<I>(&mut self, cells: I, symbols_to_keep: &Candidates) -> Result<DidUpdateGrid, SudokuError>
    where
        I: IntoIterator<Item = Cell> + Clone,
    {
        let mut did_update = false;
        let mut needs_notification = vec![];
        for cell in cells.clone() {
            let cell_mut = self.get_cell_mut(&cell)?;
            let before = cell_mut.clone();
            debug!("Keeping Only {symbols_to_keep:?} from {cell:?}. Before {before:?}");
            cell_mut.0.retain(|f| symbols_to_keep.contains(f));
            if before != *cell_mut {
                did_update = true;
                needs_notification.push(cell);
            }
        }
        debug!(
            "Candidates left after keeping: {:?}, Entropy is now {:.2}. Did update: {}",
            symbols_to_keep,
            self.get_entropy(),
            did_update
        );
        for cell in needs_notification {
            did_update |= self.notify(&cell)?;
        }
        Ok(did_update)
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
                return -1.0;
            }
            candidate_count += d.0.len() - 1;
        }
        candidate_count as f64 / maximum as f64
    }

    /// Returns an empty vector if the sudoku has no regions.
    #[allow(dead_code)]
    pub(crate) fn get_houses(&self) -> HouseSet {
        self.constraints
            .iter()
            .filter_map(|c| c.as_any().downcast_ref::<HouseUnique>())
            .flat_map(|house| house.get_houses(self))
            .collect()
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            writeln!(f, "{}", self.to_string_line())?;
            return Ok(());
        }
        let (board_rows, board_cols) = self.size();
        let (box_height, box_width) = get_box_size((board_rows, board_cols)).unwrap();

        for (i, row) in self.board.iter_rows().enumerate() {
            if i % box_height == 0 && i != 0 {
                // Horizontal divider
                let mut line = String::new();
                for col in 0..board_cols {
                    if col % box_width == 0 && col != 0 {
                        line.push_str("+-");
                    }
                    line.push_str("--");
                }
                writeln!(f, "{line}")?;
            }

            for (j, digit) in row.enumerate() {
                if j % box_width == 0 && j != 0 {
                    write!(f, "| ")?;
                }
                write!(f, "{digit} ")?;
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
