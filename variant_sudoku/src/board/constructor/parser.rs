pub mod killer;
pub mod yaml;

use log::debug;

use crate::SudokuBuilder;
use crate::{
    board::{
        constraints::RcConstraint,
        constructor::parser::{
            killer::YamlKillerCage,
            yaml::{YamlConstraint, YamlSudoku},
        },
        digit::Symbol,
        sudoku::Cell,
    },
    Sudoku, SudokuError,
};

impl<'de> serde::de::Deserialize<'de> for Sudoku {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
    D: serde::Deserializer<'de>,
    {
        let helper = YamlSudoku::deserialize(deserializer)?;
        let givens = helper.generate_given_board();
        let mut builder = SudokuBuilder::new();
        builder.set_board_size(helper.rows(), helper.cols());
        match &helper.valid_digits {
            Some(v) => builder.set_valid_symbols(v.trim().chars().map(Symbol).collect()),
            None => {
                let partial_symbols = givens.iter().filter_map(|a| *a).collect();
                builder.infer_valid_symbols_from_partial(partial_symbols)
            }
        };
        for c in parse_constraints(helper.constraints, &mut builder).map_err(serde::de::Error::custom)? {
            builder.add_constraint(c);
        }
        debug!("Building Sudoku");
        let mut sudoku = builder.build().map_err(serde::de::Error::custom)?;
        for ((row, col), symbol) in givens.indexed_iter() {
            let symbol = match symbol {
                Some(s) => s,
                None => continue,
            };
            let cell = Cell { row, col };
            sudoku.place_digit(&cell, symbol).map_err(serde::de::Error::custom)?;
        }
        Ok(sudoku)
    }
}

impl<'de> serde::de::Deserialize<'de> for Cell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (row, col) = <(usize, usize)>::deserialize(deserializer)?;
        Ok(Cell { row, col })
    }
}

fn parse_constraints(
    yaml_constraints: Option<Vec<YamlConstraint>>, builder: &mut SudokuBuilder
) -> Result<Vec<RcConstraint>, SudokuError> {
    let mut constraints = vec![];
    for constraint in yaml_constraints.unwrap_or(vec![YamlConstraint::Standard]) {
        constraints.extend(yaml_to_constraint(constraint, builder)?);
    }
    Ok(constraints)
}

fn yaml_to_constraint(
    constraint: YamlConstraint,
    builder: &mut SudokuBuilder,
) -> Result<Vec<RcConstraint>, SudokuError> {
    let ok = match constraint {
        YamlConstraint::Standard => {builder.add_standard_constraints(); return Ok(vec![])},
        YamlConstraint::Killer { cages } => YamlKillerCage::to_real(cages)?,
        e => return Err(SudokuError::UnsupportedConstraint(format!("{e:?}"))),
    };
    Ok(ok)
}
