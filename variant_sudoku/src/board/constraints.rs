pub(crate) mod killer;
pub(crate) mod standard;

use std::{any::Any, rc::Rc};

use crate::{
    board::sudoku::{Cell, DidUpdateGrid},
    errors::SudokuError,
    Sudoku,
};

pub trait Constraint: Any {
    /// For each constraint, this notify update should be called to indicate it should check for any propogations.
    ///
    /// For instance, this might be called on a cell in a RowUnique constraint for that constraint
    /// to handle removing the digit as candidates in that row.
    ///
    /// For advanced logic, you can use a solver. This is mostly used for naked singles
    /// and removing obvious constraint violations (like thermo or kropki constraints)
    fn notify_update(&self, sudoku: &mut Sudoku, cell: &Cell)
        -> Result<DidUpdateGrid, SudokuError>;

    fn as_any(&self) -> &dyn Any;

    fn use_strategies(&self, sudoku: &mut Sudoku) -> Result<DidUpdateGrid, SudokuError>;
}

pub type RcConstraint = Rc<dyn Constraint>;
