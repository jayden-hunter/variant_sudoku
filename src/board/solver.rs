use crate::{
    board::{digit::Digit, sudoku::Cell},
    Sudoku,
};
mod naked_single;
type SolverStrategy = fn(sudoku: &Sudoku) -> Option<(Cell, Digit)>;

type StrategyData = (SolverStrategy, &'static str);

pub(super) const ALL_STRATEGIES: &[StrategyData] = &[(naked_single::naked_single, "1.1")];
