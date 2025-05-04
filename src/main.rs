#![cfg_attr(windows, windows_subsystem = "windows")]

use eframe::egui;
use rfd::FileDialog;
use std::fs;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RustPad",
        options,
        Box::new(|_cc| Box::new(RustPad::default())),
    )
}

#[derive(Default)]
struct RustPad {
    text: String,
}


fn open_file() -> Option<String> {
    if let Some(path) = FileDialog::new().pick_file() {
        fs::read_to_string(path).ok()
    } else {
        None
    }
}

fn save_file(content: &str) {
    if let Some(path) = FileDialog::new().save_file() {
        let _ = fs::write(path, content);
    }
}

impl eframe::App for RustPad {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        if let Some(content) = open_file() {
                            self.text = content;
                        }
                        ui.close_menu();
                    }
                    if ui.button("Save As...").clicked() {
                        save_file(&self.text);
                        ui.close_menu();
                    }
                });
            });
        });
    
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();
            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.text));
        });
    }
}
