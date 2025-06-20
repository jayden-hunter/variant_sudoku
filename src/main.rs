use clap::Parser;
use std::{fs::File, path::PathBuf, time};
use variant_sudoku::Sudoku;

#[derive(Parser)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let path = File::open(args.path).unwrap();
    let mut sudoku: Sudoku = serde_yaml::from_reader(path).unwrap();
    println!("Loaded:\n{}", sudoku);
    let start_time = time::Instant::now();
    let solved = sudoku.solve();
    let end_time = time::Instant::now();
    println!(
        "Solved Sudoku:\n{}\nTook: {:.?}",
        solved,
        end_time - start_time
    );
    println!("{:#}", solved)
}
