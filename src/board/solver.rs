use std::rc::Rc;

use crate::{
    board::{
        constraints::standard::HouseUnique, digit::Symbol, solver::house::naked_single,
        sudoku::Cell,
    },
    Sudoku,
};
mod brute_force;
mod house;
type SolverStrategy<ConstraintType> =
    fn(sudoku: &Sudoku, applicable_constraints: Vec<Rc<ConstraintType>>) -> Option<(Cell, Symbol)>;

pub(super) const ALL_STRATEGIES: &[(SolverStrategy, f64)] = &[(naked_single, 1.1)];
