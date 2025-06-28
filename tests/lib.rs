use std::{fs::File, io::Read, path::PathBuf};

use serde::Deserialize;
use variant_sudoku::{Solution, Sudoku};

fn test_game(game: &mut Sudoku, expected_solution: Solution) {
    let actual = game.solve().expect("Sudoku should not error");
    match (actual, expected_solution) {
        (Solution::UniqueSolution(actual_board), Solution::PreComputed(expected_board)) => {
            assert_eq!(
                actual_board.to_string_line(),
                expected_board,
                "Unique solutions differ"
            );
        }

        (Solution::MultipleSolutions(actual_vec), Solution::PreComputed(expected_board)) => {
            assert!(
                actual_vec
                    .iter()
                    .any(|sudoku| sudoku.to_string_line() == expected_board),
                "None of the multiple solutions match the expected solution"
            );
        }

        (Solution::NoSolution, Solution::NoSolution) => {
            // all good
        }

        (actual, expected) => {
            panic!("Solution mismatch:\n  Expected: {expected:?}\n  Actual:   {actual:?}");
        }
    }
}

fn test_file(path: PathBuf) {
    #[derive(Deserialize)]
    struct YamlSolution {
        solution: Option<String>,
    }
    let mut string_buf = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut string_buf)
        .expect("Failed to read file");
    for _ in 0..100 {
        let mut sudoku: Sudoku =
            serde_yaml::from_str(&string_buf.clone()).expect("Failed to parse YAML");
        let expected_solution: YamlSolution =
            serde_yaml::from_str(&string_buf.clone()).expect("Failed to parse YAML");
        let expected_solution = match expected_solution.solution {
            Some(v) => Solution::PreComputed(v.into()),
            None => Solution::NoSolution,
        };
        test_game(&mut sudoku, expected_solution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! sudoku_test {
        ($name:ident) => {
            #[test]
            fn $name() {
                let name_str = stringify!($name);
                // Strip "test_" prefix at runtime
                let stem = name_str.strip_prefix("test_").unwrap();
                let filename = format!("games/{}.yaml", stem);
                let path = std::path::PathBuf::from(filename);
                test_file(path);
            }
        };
        ($name:ident, SKIP) => {
            #[test]
            #[ignore = "not yet implemented"]
            fn $name() {
                let name_str = stringify!($name);
                // Strip "test_" prefix at runtime
                let stem = name_str.strip_prefix("test_").unwrap();
                let filename = format!("games/{}.yaml", stem);
                let path = std::path::PathBuf::from(filename);
                test_file(path);
            }
        };
    }

    sudoku_test!(test_easy_standard);
    sudoku_test!(test_trivial_standard);
    sudoku_test!(test_locked_candidate_standard);
    sudoku_test!(test_hidden_subset_standard);
    sudoku_test!(test_medium_standard);
    sudoku_test!(test_hard_standard, SKIP);
    sudoku_test!(test_4x4_standard);
    sudoku_test!(test_6x6_standard);
    sudoku_test!(test_unsolveable_standard);
    sudoku_test!(test_unsolveable_variants, SKIP);
}
