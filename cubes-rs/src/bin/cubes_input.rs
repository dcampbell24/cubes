use iced::widget::{button, column, radio, row, text_input};
use iced::widget::{Column, Row};
use iced::{window, Alignment, Element};

use std::fs::{self, File};
use std::io::Write;

use cubes::{project_dir_cubes, Puzzle};

pub fn main() -> iced::Result {
    iced::application(
        "Polycube Pieces",
        PolycubePieces::update,
        PolycubePieces::view,
    )
    .window(window::Settings {
        size: iced::Size::new(210.0, 450.0),
        ..Default::default()
    })
    .run()
}

struct PolycubePieces {
    name: String,
    cube: [[[bool; 3]; 3]; 3],
    pieces: Puzzle,
}

impl PolycubePieces {
    fn sum_cubes(&self) -> i32 {
        let mut sum = 0;
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if self.cube[x][y][z] {
                        sum += 1;
                    }
                }
            }
        }
        sum
    }

    fn sum_pieces(&self) -> i32 {
        let mut sum = 0;
        for piece in &self.pieces.data {
            for _cube in piece {
                sum += 1;
            }
        }
        sum
    }
}

impl Default for PolycubePieces {
    fn default() -> Self {
        Self {
            name: String::new(),
            cube: [[[false; 3]; 3]; 3],
            pieces: Puzzle { data: Vec::new() },
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    NameChanged(String),
    SelectedPiece(usize),
    SavePiecePressed,
    SaveAllPressed,
}

impl PolycubePieces {
    fn update(&mut self, message: Message) {
        match message {
            Message::NameChanged(name) => {
                self.name = name;
            }
            Message::SelectedPiece(i) => {
                let x = i / 9;
                let y = (i / 3) % 3;
                let z = i % 3;
                self.cube[x][y][z] = !self.cube[x][y][z];
            }
            Message::SavePiecePressed => {
                self.pieces.data.push(Vec::new());
                let len = self.pieces.data.len() - 1;

                for x in 0..3 {
                    for y in 0..3 {
                        for z in 0..3 {
                            if self.cube[x][y][z] {
                                println!("{x} {y} {z}");
                                self.pieces.data[len].push([
                                    i32::try_from(x).unwrap(),
                                    i32::try_from(y).unwrap(),
                                    i32::try_from(z).unwrap(),
                                ]);
                            }
                        }
                    }
                }
                println!();
            }
            Message::SaveAllPressed => {
                let proj_dirs = project_dir_cubes().expect("expected a cubes directory");
                let dir = proj_dirs.data_dir();
                let mut path = dir.join("puzzles");
                fs::create_dir_all(&path).unwrap();

                path = path.join(&self.name);
                path.set_extension("ron");
                let mut buffer = File::create(&path).unwrap();
                let encoded =
                    ron::ser::to_string_pretty(&self.pieces, ron::ser::PrettyConfig::default())
                        .unwrap();
                buffer.write_all(encoded.as_bytes()).unwrap();
                println!("saved {path:?}");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let text_input = text_input("", &self.name).on_input(Message::NameChanged);
        let name = row!["Name: ", text_input]
            .padding(10)
            .align_y(Alignment::Start);

        let mut pieces_matrix = Vec::new();
        for x in 0..3 {
            if x > 0 {
                pieces_matrix.push(Row::new().padding(10).into());
            }
            for y in 0..3 {
                let mut row = Vec::new();
                for z in 0..3 {
                    let i = 9 * x + 3 * y + z;
                    let mut cube_selected = None;
                    if self.cube[x][y][z] {
                        cube_selected = Some(i);
                    }
                    row.push(radio("", i, cube_selected, Message::SelectedPiece).into());
                }
                pieces_matrix.push(Row::with_children(row).into());
            }
        }
        let pieces_col = Column::with_children(pieces_matrix).padding(10).spacing(10);

        let save_piece = if self.sum_pieces() >= 27
            || self.sum_cubes() < 1
            || self.sum_cubes() + self.sum_pieces() > 27
        {
            button("Save Piece")
        } else {
            button("Save Piece").on_press(Message::SavePiecePressed)
        };
        let save_all = if self.sum_pieces() != 27 || self.name.is_empty() {
            button("Save All")
        } else {
            button("Save All").on_press(Message::SaveAllPressed)
        };
        let buttons = row![save_piece, save_all]
            .padding(10)
            .spacing(10)
            .align_y(Alignment::Start);

        column![name, pieces_col, buttons]
            .align_x(Alignment::Start)
            .into()
    }
}
