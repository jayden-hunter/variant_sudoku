use std::fmt::Display;

use crate::Sudoku;

#[derive(PartialEq, Clone, Debug)]
pub struct SolutionString(pub(crate) String);

#[derive(PartialEq, Clone, Debug)]
pub enum Solution {
    PreComputed(SolutionString),
    UniqueSolution(Sudoku),
    MultipleSolutions(Vec<Sudoku>),
    NoSolution,
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::UniqueSolution(sudoku) => sudoku.fmt(f),
            Solution::PreComputed(v) => v.fmt(f),
            Solution::NoSolution => write!(f, "No solution found"),
            Solution::MultipleSolutions(solutions) => {
                writeln!(f, "{} solutions found:", solutions.len())?;
                solutions.first().unwrap().fmt(f)
            }
        }
    }
}

impl Display for SolutionString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for SolutionString {
    fn from(s: String) -> Self {
        SolutionString(s)
    }
}
