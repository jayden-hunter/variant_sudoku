use std::{fs::File, io::Read, path::PathBuf};

use serde::Deserialize;
use variant_sudoku::{Solution, Sudoku};

fn test_game(game: Sudoku, expected_solution: Solution) {}

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
    let sudoku: Sudoku = serde_yaml::from_str(&string_buf.clone()).expect("Failed to parse YAML");
    let expected_solution: YamlSolution =
        serde_yaml::from_str(&string_buf.clone()).expect("Failed to parse YAML");
    let expected_solution = match expected_solution.solution {
        Some(v) => Solution::PreComputed(v.into()),
        None => Solution::NoSolution,
    };
    test_game(sudoku, expected_solution);
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

    sudoku_test!(test_unsolveable_variants, SKIP);
}
