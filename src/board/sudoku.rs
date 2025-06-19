use grid::Grid;
use serde::Deserialize;

use crate::board::digit::Digit;

struct Sudoku {
    board: Grid<Digit>,
}

impl<'de> Deserialize<'de> for Sudoku {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SudokuHelper {
            board: Vec<String>,
        }
        let helper = SudokuHelper::deserialize(deserializer)?;
        let board: Vec<Digit> = helper
            .board
            .iter()
            .flat_map(|row| row.chars().map(Digit::from).collect::<Vec<Digit>>())
            .collect();
        let grid = Grid::from_vec(board, 9);
        Ok(Sudoku { board: grid })
    }
}
