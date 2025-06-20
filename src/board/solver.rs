use crate::{
    board::{digit::Symbol, sudoku::Cell},
    Sudoku,
};
mod brute_force;
mod easy;
type SolverStrategy = fn(sudoku: &Sudoku) -> Option<(Cell, Symbol)>;

type StrategyData = (SolverStrategy, &'static str);

pub(super) const ALL_STRATEGIES: &[StrategyData] = &[
    (easy::naked_single, "1.1"),
    (brute_force::brute_force, "9.0"),
];
