use std::collections::HashMap;

use crate::{
    board::{
        constraints::standard::{House, HouseSet},
        digit::Symbol,
        sudoku::Cell,
    },
    errors::SudokuError,
    Sudoku,
};

type HouseStrategy = fn(sudoku: &mut Sudoku, houses: &HouseSet) -> Result<(), SudokuError>;

pub(crate) const HOUSE_STRATEGIES: &[(HouseStrategy, f32)] = &[(hidden_single, 1.1)];

pub(crate) fn hidden_single(sudoku: &mut Sudoku, houses: &HouseSet) -> Result<(), SudokuError> {
    for house in houses {
        hidden_single_house(sudoku, house)?;
    }
    Ok(())
}

fn hidden_single_house(sudoku: &mut Sudoku, house: &House) -> Result<(), SudokuError> {
    let mut digit_count: HashMap<Symbol, Vec<Cell>> = HashMap::new();
    for cell in house {
        let candidates = match sudoku.get_cell(cell)?.try_get_candidates().cloned() {
            Some(c) => c,
            None => continue,
        };
        for candidate in candidates {
            digit_count.entry(candidate).or_default().push(*cell);
        }
    }
    for (symbol, cell) in
        digit_count
            .iter()
            .filter_map(|(s, v)| if v.len() == 1 { Some((s, v[0])) } else { None })
    {
        sudoku.place_digit(&cell, symbol)?;
    }
    Ok(())
}
