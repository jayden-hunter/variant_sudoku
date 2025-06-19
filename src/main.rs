use clap::Parser;
use std::{fs::File, path::PathBuf};
use variant_sudoku::Sudoku;

#[derive(Parser)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let path = File::open(args.path).unwrap();
    let sudoku: Sudoku = serde_yaml::from_reader(path).unwrap();
    println!("Loaded:\n{}", sudoku);
    let solved = sudoku.solve();
    println!("Solved Sudoku:\n{}", solved);
}
