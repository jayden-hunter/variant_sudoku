use grid::Grid;
use serde::Deserialize;

use crate::{board::digit::Digit, Sudoku};

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
        cells: Vec<[(usize, usize); 2]>,
    },
    #[serde(rename = "white_kropki")]
    WhiteKropki {
        #[serde(default)]
        variant: Option<String>,
        cells: Vec<[(usize, usize); 2]>,
    },
}

#[derive(Debug, Deserialize)]
struct KillerCage {
    cells: Vec<(usize, usize)>,
    sum: u32,
}

impl<'a, 'de> serde::de::Deserialize<'de> for Sudoku {
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
        let constraints = helper.constraints;
        Ok(Sudoku {
            board: grid,
            constraints: vec![], // Replace with actual conversion from YamlConstraint to your constraints type
        })
    }
}
