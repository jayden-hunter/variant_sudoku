use crate::{
    board::{constraints::combine_candidates, digit::Digit, sudoku::Cell},
    Sudoku,
};

pub(crate) fn naked_single(sudoku: &Sudoku) -> Option<(Cell, Digit)> {
    for (cell, _) in sudoku.indexed_iter() {
        // Get candidates for the cell
        let candidates = combine_candidates(&sudoku.constraints, sudoku, &cell).unwrap();

        // If there is only one candidate, set it
        if candidates.len() == 1 {
            let digit = candidates[0];
            println!("Naked Single found at cell {:?}, digit {}", cell, digit);
            return Some((cell, digit));
        }
    }
    println!("Naked Single Strategy Failed.");
    None
}
