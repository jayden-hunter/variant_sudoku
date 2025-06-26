use std::{any::Any, collections::HashSet};

use crate::{
    board::{
        digit::Symbol,
        solver::house::HOUSE_STRATEGIES,
        sudoku::{Cell, DidUpdateGrid},
    },
    errors::SudokuError,
    Constraint, Sudoku,
};

pub(crate) type HouseSet = HashSet<House>;

#[allow(dead_code)]
pub(crate) enum HouseUnique {
    Row,
    Col,
    Box,
    Custom(Vec<House>),
}

pub(crate) type House = Vec<Cell>;
impl HouseUnique {
    fn get_houses(&self, sudoku: &Sudoku) -> Vec<House> {
        match self {
            HouseUnique::Row => get_row_houses(sudoku),
            HouseUnique::Col => get_col_houses(sudoku),
            HouseUnique::Box => get_box_houses(sudoku),
            HouseUnique::Custom(cells) => cells.to_vec(),
        }
    }
}

impl Constraint for HouseUnique {
    fn use_strategies(&self, sudoku: &mut Sudoku) -> Result<DidUpdateGrid, SudokuError> {
        // Get all houses of the sudoku.
        let houses: HouseSet = sudoku
            .constraints
            .iter()
            .filter_map(|f| f.as_any().downcast_ref::<HouseUnique>())
            .flat_map(|f| f.get_houses(sudoku))
            .collect();
        for (strategy, _) in HOUSE_STRATEGIES {
            let did_update = strategy(sudoku, &houses)?;
            if did_update {
                return Ok(true)
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
        let mut did_update = false;
        // Check if the cell is solved.
        let digit = sudoku.get_cell(cell)?.clone();
        let symbol_to_remove_from_house = match digit.try_get_solved() {
            Some(s) => s,
            None => return Ok(did_update),
        };
        let houses = self.get_houses(sudoku);
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

fn get_row_houses(sudoku: &Sudoku) -> Vec<House> {
    let (rows, cols) = sudoku.size();
    (0..cols)
        .map(|row| (0..rows).map(|col| Cell { row, col }).collect())
        .collect()
}

fn get_col_houses(sudoku: &Sudoku) -> Vec<House> {
    let (rows, cols) = sudoku.size();
    (0..rows)
        .map(|col| (0..cols).map(|row| Cell { row, col }).collect())
        .collect()
}

fn get_box_houses(_sudoku: &Sudoku) -> Vec<House> {
    let mut houses = vec![];
    for box_row in 0..3 {
        for box_col in 0..3 {
            let house: House = (0..3)
                .flat_map(|r| {
                    (0..3).map(move |c| Cell {
                        row: box_row * 3 + r,
                        col: box_col * 3 + c,
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
            .map_or(false, |c| c.contains(symbol))
        {
            cells.push(*cell);
        }
    }
    Ok(cells)
}
