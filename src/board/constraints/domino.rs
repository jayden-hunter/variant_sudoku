use std::ops::Sub;

use crate::{
    board::{
        digit::{Candidates, Symbol},
        sudoku::{self, Cell},
    },
    errors::SudokuError,
    Constraint, Sudoku,
};

enum DominoDot {
    Black((Cell, Cell)),
    White((Cell, Cell)),
    Quad(([Cell; 4], Vec<Symbol>)),
}

impl Constraint for DominoDot {
    fn notify_update(
        &self,
        sudoku: &mut crate::Sudoku,
        cell: &Cell,
    ) -> Result<(), crate::errors::SudokuError> {
        // First, check if this is actually a cell we care about
        if !self.is_relevant(cell) {
            return Ok(());
        }
        // Now, let's ensure that this digit meets the requirements of this single-cell.
        let valid_cell1_digits = self.valid_candidates(sudoku, cell)?;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn use_strategies(&self, sudoku: &mut crate::Sudoku) -> Result<(), crate::errors::SudokuError> {
        todo!()
    }
}

impl DominoDot {
    fn is_relevant(&self, cell: &Cell) -> bool {
        match self {
            DominoDot::Black((c1, c2)) => cell == c1 || cell == c2,
            DominoDot::White((c1, c2)) => cell == c1 || cell == c2,
            DominoDot::Quad((c, _)) => c.iter().any(|f| f == cell),
        }
    }

    fn valid_candidates(&self, sudoku: &Sudoku, cell: &Cell) -> Result<&Candidates, SudokuError> {
        match self {
            DominoDot::Black(_) => todo!(),
            DominoDot::White(_) => todo!(),
            DominoDot::Quad((_, v)) => Ok(v),
        }
    }
}
