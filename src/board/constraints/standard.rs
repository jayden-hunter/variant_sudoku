use log::trace;
use std::{any::Any, collections::HashSet, rc::Rc};

use crate::{
    board::{
        constraints::{combine_constraints, RcConstraint},
        digit::Digit,
        solver::house::HOUSE_STRATEGIES,
        sudoku::Cell,
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

pub(crate) struct Standard {
    child_constraints: [RcConstraint; 3],
}

impl Default for Standard {
    fn default() -> Self {
        Self {
            child_constraints: [
                Rc::new(HouseUnique::Row),
                Rc::new(HouseUnique::Col),
                Rc::new(HouseUnique::Box),
            ],
        }
    }
}

impl Constraint for Standard {
    fn filter_cell_candidates(&self, sudoku: &mut Sudoku, cell: &Cell) -> Result<(), SudokuError> {
        combine_constraints(&self.child_constraints, sudoku, cell)
    }
    fn use_strategies(&self, sudoku: &mut Sudoku) -> Result<(), SudokuError> {
        // Standard strategies of sudoku all rely on Houses, so we can just call the first and be done with it.
        self.child_constraints[0].use_strategies(sudoku)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
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
    fn filter_cell_candidates(&self, sudoku: &mut Sudoku, cell: &Cell) -> Result<(), SudokuError> {
        let houses = self.get_houses(sudoku);
        let mut filtered_candidates = match sudoku.get_cell(cell)?.try_candidates() {
            Some(v) => v.clone(),
            None => return Ok(()),
        };
        let candidates_before = filtered_candidates.len();

        for house in &houses {
            if house.contains(cell) {
                for house_cell in house {
                    if let Some(symbol) = sudoku.get_cell(house_cell)?.try_get_solved() {
                        filtered_candidates.retain(|&d| d != *symbol);
                    }
                }
                break; // Only need to check the house containing the cell
            }
        }
        let candidates_after = filtered_candidates.len();
        *sudoku.get_cell_mut(cell)? = Digit(filtered_candidates);
        if candidates_before != candidates_after {
            trace!(
                "HouseUnique filtered {candidates_before} down to {candidates_after} for {cell:?}."
            );
        }
        Ok(())
    }

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
