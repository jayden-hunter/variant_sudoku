use std::rc::Rc;

use log::debug;
use serde::Deserialize;

use crate::{
    board::{
        constraints::{
            killer::{Cage, Killer, KillerMarking},
            RcConstraint,
        },
        sudoku::Cell,
    },
    errors::SudokuError,
};

#[derive(Debug, Deserialize)]
pub(super) struct YamlKillerCage {
    cells: Vec<Cell>,
    value: Option<u32>,
    operation: Option<YamlKillerOperation>,
}

#[derive(Debug, Deserialize)]
enum YamlKillerOperation {
    #[serde(rename = "sum")]
    Sum,
}

fn killer_value_err() -> SudokuError {
    SudokuError::ConstraintPredicateInvalid(
        "Killer cage with operation must have a value".to_string(),
    )
}

impl YamlKillerCage {
    pub(super) fn to_real(cages: Vec<YamlKillerCage>) -> Result<Vec<RcConstraint>, SudokuError> {
        debug!("Converting YamlKillerCage to real cages: {cages:?}");
        let mut killer_cages: Vec<Cage> = vec![];
        for cage in cages {
            if cage.cells.len() < 2 {
                return Err(SudokuError::ConstraintPredicateInvalid(
                    "Killer cage must have at least two cells".to_string(),
                ));
            }
            let marking = match cage.operation {
                Some(YamlKillerOperation::Sum) => {
                    KillerMarking::Sum(cage.value.ok_or_else(killer_value_err)?)
                }
                None => {
                    // If there is a value, assume it is a sum
                    if let Some(value) = cage.value {
                        KillerMarking::Sum(value)
                    } else {
                        KillerMarking::None
                    }
                }
            };
            killer_cages.push(Cage::new(cage.cells, marking));
        }
        Ok(vec![Rc::new(Killer::new(killer_cages))])
    }
}
