use log::trace;
use std::{any::Any, collections::HashSet, rc::Rc};

use crate::{
    board::{
        constraints::{combine_constraints, RcConstraint},
        digit::Digit,
        sudoku::Cell,
    },
    errors::SudokuError,
    Constraint, Sudoku,
};

pub(crate) enum HouseUnique {
    RowUnique,
    ColUnique,
    BoxUnique,
    CustomUnique(Vec<Cell>),
}

pub(crate) struct Standard {
    child_constraints: [RcConstraint; 3],
}

impl Default for Standard {
    fn default() -> Self {
        Self {
            child_constraints: [
                HouseUnique::RowUnique,
                HouseUnique::ColUnique,
                HouseUnique::BoxUnique,
            ],
        }
    }
}

impl Constraint for Standard {
    fn filter_cell_candidates(&self, sudoku: &mut Sudoku, cell: &Cell) -> Result<(), SudokuError> {
        combine_constraints(&self.child_constraints, sudoku, cell)
    }
    fn use_strategies(&self, sudoku: &mut Sudoku) -> Result<(), SudokuError> {
        // Standard strategies of sudoku all rely on Houses, so we can just call the first
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

type House = Vec<Cell>;
impl HouseUnique {
    fn get_houses(&self, sudoku: &Sudoku) -> Vec<House> {
        let house = match self {
            HouseUnique::RowUnique => get_row_houses(sudoku),
            HouseUnique::ColUnique => get_col_houses(sudoku),
            HouseUnique::BoxUnique => get_box_houses(sudoku),
            HouseUnique::CustomUnique(cells) => cells,
        };
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
        type Houses = HashSet<Vec<Cell>>;
        // Get all houses of the sudoku.
        let houses: Houses = sudoku
            .constraints
            .iter()
            .filter_map(|f| f.as_any().downcast_ref::<HouseUnique>())
            .map(|f| f.get_houses(sudoku))
            .collect::<Result<Houses, SudokuError>>()?;

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

fn get_box_houses(sudoku: &Sudoku) -> Vec<House> {
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
