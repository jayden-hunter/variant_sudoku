use std::rc::Rc;

use grid::Grid;
use serde::{Deserialize, Serialize};

use crate::{
    board::{
        constraints::{self, RcConstraint},
        digit::{Digit, Symbol},
        sudoku::Cell,
    },
    Sudoku, SudokuError,
};

#[derive(Deserialize)]
struct YamlSudoku {
    board: String,
    constraints: Option<Vec<YamlConstraint>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "name")]
#[allow(dead_code)]
enum YamlConstraint {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "killer")]
    Killer { cages: Vec<KillerCage> },
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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct KillerCage {
    cells: Vec<Cell>,
    sum: u32,
}

impl<'de> serde::de::Deserialize<'de> for Sudoku {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = YamlSudoku::deserialize(deserializer)?;
        let givens = helper.generate_given_board();
        let constraints =
            parse_constraints(helper.constraints).map_err(serde::de::Error::custom)?;
        Ok(Sudoku::new(givens, constraints))
    }
}

impl YamlSudoku {
    fn generate_given_board(&self) -> Grid<Option<Symbol>> {
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
        Grid::from_vec(cells, 9)
    }
}

impl<'de> serde::de::Deserialize<'de> for Cell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (row, col) = <(usize, usize)>::deserialize(deserializer)?;
        Ok(Cell { row, col })
    }
}

fn parse_constraints(
    yaml_constraints: Option<Vec<YamlConstraint>>,
) -> Result<Vec<RcConstraint>, SudokuError> {
    yaml_constraints
        .unwrap_or(vec![YamlConstraint::Standard])
        .into_iter()
        .map(|constraint| match constraint {
            YamlConstraint::Standard => {
                Ok(Rc::new(constraints::standard::Standard::default()) as RcConstraint)
            }
            e => Err(SudokuError::UnsupportedConstraint(format!("{:?}", e))),
        })
        .collect()
}
