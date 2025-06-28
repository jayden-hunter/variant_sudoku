#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui_extras::{Column, TableBuilder};
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
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Variant Sudoku",
        options,
        Box::new(|cc| {
            // This gives us image support:

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
            let text_height = egui::TextStyle::Body
                .resolve(ui.style())
                .size
                .max(ui.spacing().interact_size.y);

            egui::Grid::new("sudoku_grid")
                .spacing([10.0, 4.0])
                .show(ui, |ui| {
                    for row in 0..9 {
                        for col in 0..9 {
                            let cell = Cell { row, col };
                            let cell: &mut Digit = self.sudoku.get_cell_mut(&cell).unwrap();
                            let mut buf = join(cell.0.iter().map(|s| format!("{:#}", s)), " ");

                            if ui.text_edit_singleline(&mut buf).changed() {
                                if let Ok(n) = buf.parse::<u8>() {
                                    if n >= 1 && n <= 9 {
                                        *cell = Digit(vec![Symbol::from_num(n)]);
                                    } else {
                                        *cell = Digit(vec![]);
                                    }
                                } else {
                                    *cell = Digit(vec![]);
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }
}
