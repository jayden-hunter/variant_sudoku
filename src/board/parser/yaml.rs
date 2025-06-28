use std::rc::Rc;

use grid::Grid;
use serde::Deserialize;

use crate::board::{
    constraints::{standard::HouseUnique, RcConstraint},
    digit::Symbol,
    parser::killer::YamlKillerCage,
    sudoku::Cell,
};

#[derive(Deserialize)]
pub(super) struct YamlSudoku {
    pub(super) board: String,
    pub(super) valid_digits: Option<String>,
    pub(super) constraints: Option<Vec<YamlConstraint>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "name")]
#[allow(dead_code)]
pub(super) enum YamlConstraint {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "killer")]
    Killer { cages: Vec<YamlKillerCage> },
    #[serde(rename = "diagonal")]
    Diagonal { variants: Vec<String> },
    #[serde(rename = "black_kropki")]
    BlackKropki {
        #[serde(default)]
        variant: Option<String>,
        cells: Vec<[Cell; 2]>,
    },
    #[serde(rename = "white_kropki")]
    WhiteKropki {
        #[serde(default)]
        variant: Option<String>,
        cells: Vec<[Cell; 2]>,
    },
}

impl YamlSudoku {
    pub(super) fn generate_given_board(&self) -> Grid<Option<Symbol>> {
        let cells: Vec<Option<Symbol>> = self
            .board
            .lines()
            .flat_map(|row| {
                row.chars().map(|d| {
                    if d.is_ascii_alphanumeric() {
                        Some(Symbol(d))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<Option<Symbol>>>();
        let cols = cells.len().isqrt();
        Grid::from_vec(cells, cols)
    }
}

pub(super) fn new_standard_constraints() -> Vec<RcConstraint> {
    vec![
        Rc::new(HouseUnique::Row),
        Rc::new(HouseUnique::Col),
        Rc::new(HouseUnique::Box),
    ]
}
