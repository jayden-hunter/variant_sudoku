#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{fs::File, io::Read};

use eframe::egui;
use rfd::FileDialog;
use itertools::join;
use variant_sudoku::{
    board::{
        digit::{Digit, Symbol},
        sudoku::Cell,
    },
    Sudoku,
};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Variant Sudoku",
        options,
        Box::new(|_| {
            Ok(Box::<SudokuApp>::default())
        }),
    )
}

struct SudokuApp {
    sudoku: Sudoku,
}

impl Default for SudokuApp {
    fn default() -> Self {
        Self {
            sudoku: Sudoku::empty(),
        }
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Variant Sudoku in Rust");
            let (rows, cols) = self.sudoku.size();
            let cell_size = 80.0;
            egui::Grid::new("sudoku_grid")
                .spacing([4.0, 4.0])
                .show(ui, |ui| {
                    for row in 0..rows {
                        for col in 0..cols {
                            let cell = Cell { row, col };
                            let cell: &mut Digit = self.sudoku.get_cell_mut(&cell).unwrap();
                            let mut buf = join(cell.0.iter().map(|s| format!("{:#}", s)), " ");
                            let cell_ui = ui.add(
                                egui::TextEdit::singleline(&mut buf)
                                    .horizontal_align(egui::Align::Center)
                                    .vertical_align(egui::Align::Center)
                                    .desired_width(cell_size)
                                    .font(egui::TextStyle::Monospace)
                                    .clip_text(true)
                                    .min_size(egui::vec2(cell_size, cell_size)),
                            );
                            if cell_ui.changed() {
                                let symbols = buf.chars().map(|c| Symbol(c)).collect::<Vec<_>>();
                                *cell = Digit(symbols);
                            }
                        }
                        ui.end_row();
                    }
                });
            if ui.button("Load from File").clicked() {
                    if let Some(path) = FileDialog::new().add_filter("Sudoku Files", &["yaml"]).pick_file() {
                        let mut string = String::new();
                        File::open(path).unwrap().read_to_string(&mut string).unwrap();
                        self.sudoku = serde_yaml::from_str(&string).unwrap_or_else(|_| {
                            eprintln!("Failed to parse the Sudoku file.");
                            Sudoku::empty()
                        });
                    }
                }

                if ui.button("Solve").clicked() {
                    self.sudoku.solve().unwrap();
                }
        });
    }
}
