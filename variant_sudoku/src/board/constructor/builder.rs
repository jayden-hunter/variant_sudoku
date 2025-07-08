use std::collections::HashSet;
use std::rc::Rc;

use log::debug;

use crate::board::constraints::standard::{Division, HouseSet, HouseUnique};
use crate::board::constraints::RcConstraint;
use crate::board::digit::{Digit, Symbol};
use crate::board::sudoku::{Board, Constraints};
use crate::errors::SudokuError;
use crate::Sudoku;

pub struct SudokuBuilder {
    pub board: Board,
    pub houses: HouseSet,
    pub valid_symbols: HashSet<Symbol>,
    pub constraints: Constraints,
}

impl SudokuBuilder {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            houses: HashSet::new(),
            valid_symbols: HashSet::new(),
            constraints: Constraints::new(),
        }
    }

    pub fn set_board_size(&mut self, rows: usize, cols: usize) {
        self.board = Board::init(rows, cols, Digit(self.valid_symbols.clone().into_iter().collect()));
    }

    pub fn set_valid_symbols(&mut self, symbols: HashSet<Symbol>) {
        self.valid_symbols = symbols;
        debug!("Valid symbols are {:?}", self.valid_symbols);
        self.board.fill(Digit(self.valid_symbols.clone().into_iter().collect()))
    }

    /// Grid size should be established with `SudokuBuilder::set_board_size`.
    pub fn infer_valid_symbols_from_partial(&mut self, partial_symbols: HashSet<Symbol>) {
        let distinct_symbols = self.board.rows().max(self.board.cols());
        debug!("We need {distinct_symbols} distinct symbols");
        let remaining_options = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
            .chars()
            .map(Symbol);
        let mut valid_symbols: HashSet<Symbol> = partial_symbols;
        for i in remaining_options {
            if valid_symbols.len() == distinct_symbols {
                self.set_valid_symbols(valid_symbols);
                return
            }
            valid_symbols.insert(i);
        }
    }

    pub fn add_constraint(&mut self, constraint: RcConstraint) {
        self.constraints.push(constraint);
    }

    pub fn add_standard_constraints(&mut self) {
        const STANDARD_DIVISIONS: [Division;3] = [Division::Row, Division::Col, Division::Box];
        // First, get the size of the grid.
        let (rows, cols) = self.board.size();
        for div in STANDARD_DIVISIONS {
            self.houses.extend(div.get_houses(rows, cols));
        }
        self.constraints.push(Rc::new(HouseUnique));
    }   

    pub fn build(self) -> Result<Sudoku, SudokuError> {
        // Validate that the board has valid dimensions and symbols
        if self.board.is_empty() {
            return Err(SudokuError::InvalidBoard);
        }
        Ok(Sudoku {
            board: self.board,
            houses: self.houses,
            valid_symbols: self.valid_symbols,
            constraints: self.constraints,
        })
    }
}