use std::rc::Rc;

use crate::{
    board::{
        constraints::base::{combine_candidates, RcConstraint},
        digit::Digit,
        sudoku::Cell,
    },
    Constraint, Sudoku,
};

pub(crate) struct RowUnique;
pub(crate) struct ColUnique;
pub(crate) struct BoxUnique;

pub(crate) struct Standard {
    child_constraints: Vec<RcConstraint>,
}

impl Default for Standard {
    fn default() -> Self {
        Self {
            child_constraints: vec![Rc::new(RowUnique), Rc::new(ColUnique), Rc::new(BoxUnique)],
        }
    }
}

impl Constraint for Standard {
    fn is_satisfied(&self, sudoku: &Sudoku) -> bool {
        self.child_constraints
            .iter()
            .all(|constraint| constraint.is_satisfied(sudoku))
    }

    fn get_cell_candidates(&self, sudoku: &Sudoku, cell: &Cell) -> Vec<Digit> {
        combine_candidates(&self.child_constraints, sudoku, cell)
    }
}

type House = Vec<Cell>;
pub trait HouseUnique {
    fn get_houses(&self, sudoku: &Sudoku) -> Vec<House>;

    fn is_house_satisfied(&self, sudoku: &Sudoku, house: &House) -> bool {
        let mut seen_digits = vec![];
        for cell in house {
            if let Some(digit) = sudoku.get_cell(cell).unwrap().get_number() {
                if seen_digits.contains(&digit) {
                    return false; // Duplicate found
                }
                seen_digits.push(digit);
            }
        }
        true // All cells in the house are unique
    }
}

impl<T: ?Sized + HouseUnique> Constraint for T {
    fn is_satisfied(&self, sudoku: &Sudoku) -> bool {
        self.get_houses(sudoku)
            .iter()
            .all(|house| self.is_house_satisfied(sudoku, house))
    }

    fn get_cell_candidates(&self, sudoku: &Sudoku, cell: &Cell) -> Vec<Digit> {
        let houses = self.get_houses(sudoku);
        let mut candidates = (1..=9).map(Digit::Number).collect::<Vec<_>>();

        for house in &houses {
            if house.contains(cell) {
                for house_cell in house {
                    if let Some(digit) = sudoku.get_cell(house_cell).unwrap().get_number() {
                        candidates.retain(|&d| d != Digit::Number(digit));
                    }
                }
                break; // Only need to check the house containing the cell
            }
        }
        candidates
    }
}

impl HouseUnique for RowUnique {
    fn get_houses(&self, sudoku: &Sudoku) -> Vec<House> {
        let rows = sudoku.board.rows();
        let cols = sudoku.board.cols();
        (0..cols)
            .map(|row| (0..rows).map(|col| Cell { row, col }).collect())
            .collect()
    }
}

impl HouseUnique for ColUnique {
    fn get_houses(&self, sudoku: &Sudoku) -> Vec<House> {
        let rows = sudoku.board.rows();
        let cols = sudoku.board.cols();
        (0..rows)
            .map(|col| (0..cols).map(|row| Cell { row, col }).collect())
            .collect()
    }
}
impl HouseUnique for BoxUnique {
    fn get_houses(&self, _sudoku: &Sudoku) -> Vec<House> {
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
}
