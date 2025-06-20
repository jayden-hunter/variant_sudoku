use std::rc::Rc;

use grid::Grid;
use serde::{Deserialize, Serialize};

use crate::{
    board::{
        constraints::{self, RcConstraint},
        digit::{CellData, Digit},
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
        let helper = SudokuHelper::deserialize(deserializer)?;
        let valid_digits = [
            Digit('1'),
            Digit('2'),
            Digit('3'),
            Digit('4'),
            Digit('5'),
            Digit('6'),
            Digit('7'),
            Digit('8'),
            Digit('9'),
        ];
        let board: Vec<CellData> = helper
            .board
            .lines()
            .flat_map(|row| {
                row.chars()
                    .map(|d| {
                        if d.is_ascii_alphanumeric() {
                            CellData::Digit(Digit(d))
                        } else {
                            CellData::Candidates(valid_digits.to_vec())
                        }
                    })
                    .collect::<Vec<CellData>>()
            })
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

impl Serialize for Sudoku {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            &self
                .board
                .iter()
                .map(|digit| digit.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
