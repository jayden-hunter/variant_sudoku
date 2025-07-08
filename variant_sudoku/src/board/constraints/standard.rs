use std::{any::Any, collections::HashSet};

use log::trace;

use crate::{
    board::{
        digit::Symbol,
        solver::house::HOUSE_STRATEGIES,
        sudoku::{Cell, DidUpdateGrid},
    },
    errors::SudokuError,
    Constraint, Sudoku,
};

pub(crate) type House = Vec<Cell>;
pub(crate) type HouseSet = HashSet<House>;

// Actual houses are stored in the Sudoku for easy access for other constraints.
pub struct HouseUnique;

#[allow(dead_code)]
pub(crate) enum Division {
    Row,
    Col,
    Box,
    Custom(Vec<House>),
}


impl Division {
    pub(crate) fn get_houses(&self, rows: usize, cols: usize) -> Vec<House> {
        match self {
            Division::Row => get_row_houses(rows, cols),
            Division::Col => get_col_houses(rows, cols),
            Division::Box => get_box_houses(rows, cols),
            Division::Custom(cells) => cells.to_vec(),
        }
    }
}

impl Constraint for HouseUnique {
    fn use_strategies(&self, sudoku: &mut Sudoku) -> Result<DidUpdateGrid, SudokuError> {
        // Get all houses of the sudoku.
        let houses = sudoku.houses.clone();
        for (strategy, _) in HOUSE_STRATEGIES {
            let did_update = strategy(sudoku, &houses)?;
            if did_update {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn notify_update(
        &self,
        sudoku: &mut Sudoku,
        cell: &Cell,
    ) -> Result<DidUpdateGrid, SudokuError> {
        trace!("HouseUnique Notify Update for cell {cell:?}");
        let mut did_update = false;
        // Check if the cell is solved.
        let digit = sudoku.get_cell(cell)?.clone();
        let symbol_to_remove_from_house = match digit.try_get_solved() {
            Some(s) => s,
            None => return Ok(false),
        };
        let houses = sudoku.houses.clone();
        let cells = houses
            .iter()
            .filter(|c| c.contains(cell))
            .flatten()
            .filter(|f| *f != cell);
        for c in cells {
            did_update |= sudoku.remove_candidate(c, symbol_to_remove_from_house)?;
        }
        Ok(did_update)
    }
}

fn get_row_houses(rows: usize, cols: usize) -> Vec<House> {
    (0..cols)
        .map(|row| (0..rows).map(|col| Cell { row, col }).collect())
        .collect()
}

fn get_col_houses(rows: usize, cols: usize) -> Vec<House> {
    (0..rows)
        .map(|col| (0..cols).map(|row| Cell { row, col }).collect())
        .collect()
}

pub(crate) fn get_box_size(rows: usize, cols: usize) -> Result<(usize, usize), SudokuError> {
    let ok = match (rows, cols) {
        (9, 9) => (3, 3),
        (4, 4) => (2, 2),
        (6, 6) => (2, 3), //2 rows, 3 cols
        v => {
            return Err(SudokuError::UnsupportedConstraint(format!(
                "Invalid BoxUnique with grid of size {v:?}"
            )))
        }
    };
    Ok(ok)
}

fn get_box_houses(rows: usize, cols: usize) -> Vec<House> {
    let mut houses = vec![];
    let (box_row_size, box_col_size) = get_box_size(rows, cols).unwrap();
    let num_box_rows = rows / box_row_size;
    let num_box_cols = cols / box_col_size;
    for box_row in 0..num_box_rows {
        for box_col in 0..num_box_cols {
            let house: House = (0..box_row_size)
                .flat_map(|r| {
                    (0..box_col_size).map(move |c| Cell {
                        row: box_row * box_row_size + r,
                        col: box_col * box_col_size + c,
                    })
                })
                .collect();
            houses.push(house);
        }
    }
    houses
}

pub(crate) fn get_house_candidates(
    sudoku: &Sudoku,
    house: &House,
) -> Result<HashSet<Symbol>, SudokuError> {
    let mut candidates = HashSet::new();
    for cell in house {
        let digit = sudoku.get_cell(cell)?;
        if let Some(c) = digit.try_get_candidates() {
            candidates.extend(c.iter().cloned());
        }
    }
    Ok(candidates)
}

pub(crate) fn get_cells_in_house(
    sudoku: &Sudoku,
    house: &House,
    symbol: &Symbol,
) -> Result<Vec<Cell>, SudokuError> {
    let mut cells = vec![];
    for cell in house {
        if sudoku
            .get_cell(cell)?
            .try_get_candidates()
            .is_some_and(|c| c.contains(symbol))
        {
            cells.push(*cell);
        }
    }
    Ok(cells)
}
