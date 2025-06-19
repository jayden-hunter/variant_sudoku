use std::rc::Rc;

use grid::Grid;
use serde::Deserialize;

use crate::{
    board::{
        constraints::{self, base::RcConstraint},
        digit::Digit,
        sudoku::Cell,
    },
    Sudoku, SudokuError,
};

#[derive(Deserialize)]
struct SudokuHelper {
    board: String,
    constraints: Option<Vec<YamlConstraint>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "name")]
enum YamlConstraint {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "killer")]
    Killer { cages: Vec<KillerCage> },
    #[serde(rename = "diagonal")]
    Diagonal { variant: String },
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
struct KillerCage {
    cells: Vec<Cell>,
    sum: u32,
}

impl<'de> serde::de::Deserialize<'de> for Sudoku {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = SudokuHelper::deserialize(deserializer)?;
        let board: Vec<Digit> = helper
            .board
            .lines()
            .flat_map(|row| row.chars().map(Digit::from).collect::<Vec<Digit>>())
            .collect();
        let grid = Grid::from_vec(board, 9);
        let constraints =
            parse_constraints(helper.constraints).map_err(serde::de::Error::custom)?;
        Ok(Sudoku {
            board: grid,
            constraints,
        })
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
