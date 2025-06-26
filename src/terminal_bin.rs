use clap::Parser;
use std::{fs::File, path::PathBuf, time};
use variant_sudoku::Sudoku;

#[derive(Parser)]
struct Args {
    path: PathBuf,
    logfile: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    setup_logging(&args);
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

fn setup_logging(args: &Args) {
    let mut logger = env_logger::builder();
    if let Some(path) = &args.logfile {
        let file = Box::new(File::create(path).unwrap());
        logger.target(env_logger::Target::Pipe(file));
        logger.filter_level(log::LevelFilter::Debug);
    }
    logger.init();
}
