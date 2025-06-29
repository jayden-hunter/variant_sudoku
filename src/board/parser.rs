pub(super) mod killer;
pub(super) mod yaml;

use crate::{
    board::{
        constraints::RcConstraint,
        digit::Symbol,
        parser::{
            killer::YamlKillerCage,
            yaml::{new_standard_constraints, YamlConstraint, YamlSudoku},
        },
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
        let constraints =
            parse_constraints(helper.constraints).map_err(serde::de::Error::custom)?;
        let sudoku = match helper.valid_digits {
            Some(v) => {
                let valid_symbols = v.trim().chars().map(Symbol).collect();
                Sudoku::new_with_valid_digits(givens, constraints, valid_symbols)
            }
            None => Sudoku::new(givens, constraints),
        };
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
    yaml_constraints: Option<Vec<YamlConstraint>>,
) -> Result<Vec<RcConstraint>, SudokuError> {
    let nested_constraints = yaml_constraints
        .unwrap_or(vec![YamlConstraint::Standard])
        .into_iter()
        .map(yaml_to_constraint)
        .collect::<Result<Vec<_>, _>>()?;

    let flat_constraints = nested_constraints.into_iter().flatten().collect::<Vec<_>>();
    Ok(flat_constraints)
}

fn yaml_to_constraint(constraint: YamlConstraint) -> Result<Vec<RcConstraint>, SudokuError> {
    let ok = match constraint {
        YamlConstraint::Standard => new_standard_constraints(),
        YamlConstraint::Killer { cages } => YamlKillerCage::to_real(cages)?,
        e => return Err(SudokuError::UnsupportedConstraint(format!("{e:?}"))),
    };
    Ok(ok)
}
