use std::{any::Any, collections::HashSet};

use crate::{
    board::{solver::house::HOUSE_STRATEGIES, sudoku::Cell},
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
    fn use_strategies(&self, sudoku: &mut Sudoku) -> Result<(), SudokuError> {
        // Get all houses of the sudoku.
        let houses: HouseSet = sudoku
            .constraints
            .iter()
            .filter_map(|f| f.as_any().downcast_ref::<HouseUnique>())
            .flat_map(|f| f.get_houses(sudoku))
            .collect();
        for (strategy, _) in HOUSE_STRATEGIES {
            strategy(sudoku, &houses)?;
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn notify_update(&self, sudoku: &mut Sudoku, cell: &Cell) -> Result<(), SudokuError> {
        // Check if the cell is solved.
        let digit = sudoku.get_cell(cell)?.clone();
        let symbol_to_remove_from_house = match digit.try_get_solved() {
            Some(s) => s,
            None => return Ok(()),
        };
        let houses = self.get_houses(sudoku);
        let cells = houses
            .iter()
            .filter(|c| c.contains(cell))
            .flatten()
            .filter(|f| *f != cell);
        for c in cells {
            sudoku.remove_candidate(c, symbol_to_remove_from_house)?;
        }
        Ok(())
    }
}

fn get_row_houses(sudoku: &Sudoku) -> Vec<House> {
    let rows = sudoku.board.rows();
    let cols = sudoku.board.cols();
    (0..cols)
        .map(|row| (0..rows).map(|col| Cell { row, col }).collect())
        .collect()
}

fn get_col_houses(sudoku: &Sudoku) -> Vec<House> {
    let rows = sudoku.board.rows();
    let cols = sudoku.board.cols();
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
