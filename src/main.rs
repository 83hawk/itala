#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::fs;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    // 1. Cross-platform Home Directory
    // Checks "HOME" (Linux) first, then "USERPROFILE" (Windows)
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Could not find home directory");
    
    let mut folder_path = PathBuf::from(home);
    folder_path.push("Documents");
    folder_path.push("Notes");
    let _ = fs::create_dir_all(&folder_path);

    let mut file_path = folder_path.clone();
    file_path.push("note.txt");

    // Rest of your code follows...
    let mut my_note = fs::read_to_string(&file_path).unwrap_or_default();
    let mut show_save_dialog = false;
    let mut new_filename = String::from("new_note.txt");

    eframe::run_simple_native("itala", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📝 itala");

            if ui.button("Save...").clicked() {
                show_save_dialog = true;
            }

            ui.separator();

            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut my_note));
        });

        if show_save_dialog {
            egui::Window::new("Save Options").show(ctx, |ui| {
                if ui.button("Save as 'note.txt'").clicked() {
                    let _ = fs::write(&file_path, &my_note);
                    show_save_dialog = false;
                }
                ui.text_edit_singleline(&mut new_filename);
                if ui.button("Save as New File").clicked() {
                    let mut new_p = folder_path.clone();
                    new_p.push(&new_filename);
                    let _ = fs::write(new_p, &my_note);
                    show_save_dialog = false;
                }
                if ui.button("Cancel").clicked() { show_save_dialog = false; }
            });
        }
    })
}

