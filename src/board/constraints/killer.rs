use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;
use log::{debug, trace};

use crate::{
    board::{
        constraints::standard::House,
        digit::Symbol,
        sudoku::{Cell, DidUpdateGrid},
    },
    errors::SudokuError,
    Constraint, Sudoku,
};

/// This constraint is only responsible for making sure the numbers all add up.
/// Uniqueness is handled by a seperate `HouseUnique` constraint, for those Cells.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Killer {
    cages: Vec<Cage>,
}

impl Killer {
    pub(crate) fn new(cages: Vec<Cage>) -> Self {
        debug!("Killer Created");
        Self { cages }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Cage {
    pub(crate) cells: House,
    marking: KillerMarking,
}

impl Cage {
    pub(crate) fn new(cells: House, marking: KillerMarking) -> Self {
        Self { cells, marking }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum KillerMarking {
    None,
    Sum(u32),
}

impl Constraint for Killer {
    fn notify_update(
        &self,
        sudoku: &mut Sudoku,
        cell: &Cell,
    ) -> Result<DidUpdateGrid, SudokuError> {
        let cage = match self.cages.iter().find(|c| c.cells.contains(cell)) {
            Some(c) => c,
            None => {
                trace!("Cell {cell:?} is not part of any Killer Cage");
                return Ok(false);
            }
        };
        let digit = sudoku.get_cell(cell)?.clone();
        let mut did_update = false;
        if let Some(s) = digit.try_get_solved() {
            for c in cage.cells.iter().filter(|c| *c != cell) {
                debug!("Removing {s:?} from cell {c:?} in cage {cage:?}, because it already exists in {cell:?}");
                did_update |= sudoku.remove_candidate(c, s)?;
            }
        }
        Ok(did_update)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn use_strategies(&self, sudoku: &mut Sudoku) -> Result<DidUpdateGrid, SudokuError> {
        debug!("Killer Notify Update");
        let mut did_update = false;
        for cage in &self.cages {
            did_update |= cage.notify_cage(sudoku)?;
            if did_update {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Cage {
    fn notify_cage(&self, sudoku: &mut Sudoku) -> Result<DidUpdateGrid, SudokuError> {
        trace!("KillerCage Notify Update");
        let mut did_update = false;
        let possible_candidates = self.marking.get_possible_candidates(self, sudoku)?;
        let unsolved_cells: Vec<Cell> = self
            .cells
            .iter()
            .filter(|c| sudoku.get_cell(c).is_ok_and(|f| !f.is_solved()))
            .map(|c| *c)
            .collect();
        did_update |= sudoku.keep_candidates(unsolved_cells, &possible_candidates)?;
        Ok(did_update)
    }
    fn get_sum_options(&self, sudoku: &mut Sudoku, cage_sum: u32) -> HashSet<Symbol> {
        debug!("Getting Sum Options for Cage: {self:?} with sum {cage_sum}");
        let candidates: HashSet<&Symbol> = self
            .cells
            .iter()
            .filter_map(|cell| {
                sudoku
                    .get_cell(cell)
                    .ok()
                    .and_then(|c| c.try_get_candidates())
            })
            .flatten()
            .collect();
        let solved = self
            .cells
            .iter()
            .filter_map(|cell| sudoku.get_cell(cell).ok().and_then(|c| c.try_get_solved()))
            .collect::<Vec<_>>();
        let num_options = self.cells.len() - solved.len();
        let cage_sum_without_placed = cage_sum
            - solved
                .iter()
                .map(|s| s.get_number().unwrap_or(0))
                .sum::<u32>();
        trace!("Need to come up with {num_options} options for cage sum {cage_sum}, reduced to {cage_sum_without_placed}. Candidate Cells: {candidates:?}");
        let mut keep_digits = HashSet::new();
        // Generate num_options of candidates.
        for digits in candidates.iter().combinations(num_options) {
            trace!("Checking digits: {digits:?}");
            if digits.iter().filter_map(|f| f.get_number()).sum::<u32>() == cage_sum_without_placed
            {
                keep_digits.extend(digits.iter().map(|&&s| s));
            }
        }
        debug!("Valid Digits for {keep_digits:?}");
        keep_digits
    }
}

impl KillerMarking {
    /// Remove impossible candidates for all cells in this cage.
    /// For instance, in a 2-cell cage with sum 4, the candidates are only 1 and 3.
    fn get_possible_candidates(
        &self,
        cage: &Cage,
        sudoku: &mut Sudoku,
    ) -> Result<Vec<Symbol>, SudokuError> {
        let ok = match self {
            KillerMarking::None => sudoku.valid_symbols.clone(),
            KillerMarking::Sum(sum) => cage.get_sum_options(sudoku, *sum),
        };
        debug!("{self:?} Possible Candidates: {ok:?}");
        Ok(ok.iter().cloned().collect())
    }
}
