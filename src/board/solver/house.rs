use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use log::{debug, trace};

use crate::{
    board::{
        constraints::standard::{get_cells_in_house, get_house_candidates, House, HouseSet},
        digit::Symbol,
        sudoku::{Cell, DidUpdateGrid},
    },
    errors::SudokuError,
    Sudoku,
};

type HouseStrategy =
    fn(sudoku: &mut Sudoku, houses: &HouseSet) -> Result<DidUpdateGrid, SudokuError>;

pub(crate) const HOUSE_STRATEGIES: &[(HouseStrategy, f32)] = &[
    (hidden_single, 1.5),
    (locked_candidate, 2.5),
    (hidden_subset, 3.0),
];

pub(crate) fn hidden_single(
    sudoku: &mut Sudoku,
    houses: &HouseSet,
) -> Result<DidUpdateGrid, SudokuError> {
    debug!(
        "Running Hidden Single, Entropy is {:.2} ({:?}",
        sudoku.get_entropy(),
        sudoku.to_string_line()
    );
    for house in houses {
        let did_update = hidden_single_house(sudoku, house)?;
        if did_update {
            return Ok(true)
        }
    }
    Ok(false)
}

fn hidden_single_house(sudoku: &mut Sudoku, house: &House) -> Result<DidUpdateGrid, SudokuError> {
    let mut did_update = false;
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
        did_update = true;
    }
    Ok(did_update)
}

/// A Locked Candidate occurs when a given digit might be placed in a house only at the intersection with another house.
/// In this situation, you can eliminate this digit from the remaining cells of the second house.
pub(crate) fn locked_candidate(
    sudoku: &mut Sudoku,
    houses: &HouseSet,
) -> Result<DidUpdateGrid, SudokuError> {
    debug!(
        "Running Locked Candidate (Starting Entropy is {:.2}) ({:?}",
        sudoku.get_entropy(),
        sudoku.to_string_line()
    );
    let combinations = houses.iter().combinations(2);
    for t in combinations {
        let (house1, house2) = match t.as_slice() {
            [h1, h2] => (h1, h2),
            _ => continue,
        };
        let did_update = locked_candidate_houses(sudoku, house1, house2)?;
        if did_update {
            return Ok(true)
        }
    }
    Ok(false)
}

/// Checks if a given digit is placed in h1 only at an intersection of h2.
/// If so, remove this digit from the remaining cells of h2.
fn locked_candidate_houses(
    sudoku: &mut Sudoku,
    house1: &House,
    house2: &House,
) -> Result<DidUpdateGrid, SudokuError> {
    let mut did_update = false;
    let candidates1 = get_house_candidates(sudoku, house1)?;
    for symbol in candidates1 {
        let h1_candidate_cells = get_cells_in_house(sudoku, house1, &symbol)?;
        // If all of the cells in the intersection are contained within house2
        let is_all_within_house2 = h1_candidate_cells.iter().all(|c| house2.contains(c));
        if !is_all_within_house2 {
            continue;
        }
        let h2_candidate_cells = get_cells_in_house(sudoku, house2, &symbol)?;
        // Remove the digit from remaining cells in house2
        for cell in h2_candidate_cells
            .iter()
            .filter(|c| !h1_candidate_cells.contains(c))
        {
            did_update |= sudoku.remove_candidate(cell, &symbol)?;
        }
    }
    Ok(did_update)
}

pub(crate) fn hidden_subset(
    sudoku: &mut Sudoku,
    houses: &HouseSet,
) -> Result<DidUpdateGrid, SudokuError> {
    debug!(
        "Running Hidden Subset, Entropy is {:.2} ({:?})",
        sudoku.get_entropy(),
        sudoku.to_string_line()
    );
    let max_house_size = houses.iter().filter_map(|h| get_house_candidates(sudoku, h).ok().map(|f| f.len())).max().ok_or_else(|| {
        SudokuError::UnsupportedConstraint("House must contain at least 1 Cell".to_owned())
    })?;
    let max_subset_size = max_house_size / 2;
    debug!("Max House Size: {max_house_size}, Max Subset Size: {max_subset_size}");
    for subset_size in 2..=max_subset_size {
        for house in houses {
            let did_update = hidden_subset_house(sudoku, house, subset_size)?;
            if did_update {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// If you can find `n` cells within a house such as that two candidates appear nowhere outside those cells in that house,
/// those `n` candidates must be placed in the `n` cells.
fn hidden_subset_house(
    sudoku: &mut Sudoku,
    house: &House,
    num: usize,
) -> Result<DidUpdateGrid, SudokuError> {
    let mut did_update = false;
    let candidates = get_house_candidates(sudoku, house)?;
    let combinations = candidates.iter().combinations(num);
    for combo in combinations {
        debug!("Attempting Combo {combo:?}");
        // Check that the combo exist in the same `num` cells
        let found_cells_vec = combo
            .iter()
            .map(|s| get_cells_in_house(sudoku, house, s))
            .collect::<Result<Vec<_>, _>>()?;
        let found_cells: HashSet<_> = found_cells_vec.into_iter().flatten().collect();
        if found_cells.len() != num {
            // If the number of cells found is not equal to the number of candidates, skip
            continue;
        }
        // Found a match - we can remove all other candidates from these cells.
        debug!("Found a Subset!");
        for cell in found_cells.into_iter() {
            let combo_symbols: Vec<Symbol> = combo.iter().copied().cloned().collect();
            did_update |= sudoku.keep_candidates(&cell, &combo_symbols)?;
        }
        debug!("Subset did_update {did_update}");
        if did_update {
            return Ok(true)
        }
    }
    Ok(did_update)
}
