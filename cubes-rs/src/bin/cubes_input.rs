use iced::widget::{Column, Row};
use iced::widget::{button, checkbox, column, row, text_input};
use iced::{window, Alignment, Element, Sandbox, Settings};

use std::fs::{self, File};
use std::io::Write;

use cubes::{project_dir_cubes, Puzzle};

pub fn main() -> iced::Result {
    PolycubePieces::run(Settings {
        window: window::Settings {
            size: (200, 400),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct PolycubePieces {
    name: String,
    cube: [[[bool; 3]; 3]; 3],
    pieces: Puzzle,
}

impl Default for PolycubePieces {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            cube: [[[false; 3]; 3]; 3],
            pieces: Puzzle { data: Vec::new() },
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    NameChanged(String),
    SelectedPiece(bool),
    SavePiecePressed,
    SaveAllPressed,
}

impl Sandbox for PolycubePieces {
    type Message = Message;

    fn new() -> Self {
        PolycubePieces::default()
    }

    fn title(&self) -> String {
        String::from("Polycube Pieces")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NameChanged(name) => {
                self.name = name;
            },
            Message::SelectedPiece(bool) => {
                // fixme
            }
            Message::SavePiecePressed => {
                self.pieces.data.push(Vec::new());
                let len = self.pieces.data.len() - 1;

                for x in 0..3 {
                    for y in 0..3 {
                        for z in 0..3 {
                            if self.cube[x][y][z] {
                                println!("{:?} {:?} {:?}", x, y, z);
                                self.pieces.data[len].push([x as i32, y as i32, z as i32]);
                            }
                        }
                    }
                }
                println!();
            },
            Message::SaveAllPressed => {
                let proj_dirs = project_dir_cubes().expect("expected a cubes directory");
                let dir = proj_dirs.data_dir();
                let path = dir.join("puzzles");
                fs::create_dir_all(&path).unwrap();

                let mut buffer = File::create(path.join(&self.name)).unwrap();
                let encoded: Vec<u8> = bincode::serialize(&self.pieces).unwrap();
                buffer.write_all(&encoded).unwrap();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let name = row![
            "Name: ",
            text_input("", &self.name, Message::NameChanged),
        ]
        .padding(10)
        .align_items(Alignment::Start);

        let mut pieces_matrix = Vec::new();
        for x in 0..3 {
            if x > 0 {
                pieces_matrix.push(Row::new().into());
            }
            for y in 0..3 {
                let mut row = Vec::new();
                for z in 0..3 {
                    row.push(checkbox("", self.cube[x][y][z], Message::SelectedPiece).into());
                }
                pieces_matrix.push(Row::with_children(row).into());
            }
        }
        let pieces_col = Column::with_children(pieces_matrix).padding(10).spacing(10);

        let save_piece = button("Save Piece").on_press(Message::SavePiecePressed);
        let save_all = button("Save All").on_press(Message::SaveAllPressed);
        let buttons = row![save_piece, save_all]
            .padding(10)
            .spacing(10)
            .align_items(Alignment::Start);

        column![name, pieces_col, buttons]
            .align_items(Alignment::Start)
            .into()
    }
}
