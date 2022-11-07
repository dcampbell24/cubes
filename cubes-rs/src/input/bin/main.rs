use eframe::egui;
use egui::emath::Vec2;
use serde::{Deserialize, Serialize};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2 { x: 180., y: 300. }),
        ..Default::default()    
    };

    eframe::run_native(
        "Polycubes",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Debug, Deserialize, Serialize)]
struct Pieces {
    data: Vec<Vec<[usize; 3]>>
}

struct MyApp {
    name: String,
    cube: [[[bool; 3]; 3]; 3],
    pieces: Pieces,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            cube: [[[false; 3]; 3]; 3],
            pieces: Pieces { data: Vec::new() },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("polycube piece");
            ui.horizontal(|ui| {
                ui.set_width(100.);
                ui.text_edit_singleline(&mut self.name);
                ui.label(".ron")
            });
            
            for x in 0..3 {
                if x > 0 {
                    ui.horizontal(|ui| {
                        ui.label("");
                    });
                }
                for y in 0..3 {
                    ui.horizontal(|ui| {
                        for z in 0..3 {
                            if ui.toggle_value(&mut self.cube[x][y][z], "    ").clicked() {

                            }
                        }
                    });
                }
            }

            ui.horizontal(|ui| {
                if ui.button("save piece").clicked() {
                    self.pieces.data.push(Vec::new());
                    let len = self.pieces.data.len() - 1;

                    for x in 0..3 {
                        for y in 0..3 {
                            for z in 0..3 {
                                if self.cube[x][y][z] {
                                    println!("{:?} {:?} {:?}", x, y, z);
                                    self.pieces.data[len].push([x, y, z]);
                                }
                            }
                        }
                    }
                    println!();
                }

                if ui.button("save all").clicked() {
                    println!("{}", ron::to_string(&self.pieces).unwrap());
                    println!();    
                }
            });
        });
    }
}