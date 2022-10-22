use std::fmt::Write;
use std::fs::{self, File};
use std::io::ErrorKind::AlreadyExists;
use std::io::Write as _;


fn main()-> std::io::Result<()> {
    let puzzles = vec![cubes_rs::minotaur(), cubes_rs::blue()];
    let puzzles_string = vec![
        String::from("minotaur"),
        String::from("blue")
    ];

    for (j, (puzzle, puzzle_string)) in puzzles.iter().zip(puzzles_string).enumerate() {
        match fs::create_dir(format!("target/{}", puzzle_string)) {
            Err(e) if e.kind() == AlreadyExists => { },
            e @ Err(_) => return e,
            Ok(_) => { },
        }
    
        write_mtl_file(&puzzle_string)?;

        for (i, piece) in puzzle.iter().enumerate() {
            let mut buffer = File::create(format!("target/{}/piece_{}.obj", puzzle_string, i))?;
            let mut string = String::new();
            
            writeln!(string, "# Rust generated OBJ file.").unwrap();        
            writeln!(string, "mtllib piece.mtl").unwrap();
            writeln!(string, "usemtl {}", j).unwrap();

            for [x, y, z] in piece {
                writeln!(string, "v {} {} {}", x - 1, y - 1, z - 1).unwrap();
                writeln!(string, "v {} {} {}", x - 1, y, z - 1).unwrap();
                writeln!(string, "v {} {} {}", x, y - 1, z - 1).unwrap();
                writeln!(string, "v {} {} {}", x, y, z - 1).unwrap();
                writeln!(string, "v {} {} {}", x - 1, y - 1, z).unwrap();
                writeln!(string, "v {} {} {}", x - 1, y, z).unwrap();
                writeln!(string, "v {} {} {}", x, y - 1, z).unwrap();
                writeln!(string, "v {} {} {}", x, y, z).unwrap();
            }

            for (i, _) in piece.iter().enumerate() {
                writeln!(string, "f {} {} {} {}", i * 8 + 1, i * 8 + 2, i * 8 + 4, i * 8 + 3).unwrap();
                writeln!(string, "f {} {} {} {}", i * 8 + 5, i * 8 + 6, i * 8 + 8, i * 8 + 7).unwrap();
                writeln!(string, "f {} {} {} {}", i * 8 + 1, i * 8 + 2, i * 8 + 6, i * 8 + 5).unwrap();
                writeln!(string, "f {} {} {} {}", i * 8 + 4, i * 8 + 3, i * 8 + 7, i * 8 + 8).unwrap();
                writeln!(string, "f {} {} {} {}", i * 8 + 1, i * 8 + 5, i * 8 + 7, i * 8 + 3).unwrap();
                writeln!(string, "f {} {} {} {}", i * 8 + 2, i * 8 + 6, i * 8 + 8, i * 8 + 4).unwrap();
            }

            buffer.write_all(&string.into_bytes())?;
        }
    }
    Ok(())
}

fn write_mtl_file(path: &str) -> std::io::Result<()> {
    let mut buffer = File::create(format!("target/{}/piece.mtl", path))?;
    let mut string = String::new();
    writeln!(string, "# Rust generated MTL file").unwrap();
    writeln!(string, "newmtl 0").unwrap();
    writeln!(string, "Kd 0.3 0.3 0.3").unwrap();
    writeln!(string, "newmtl 1").unwrap();
    writeln!(string, "Kd 0.0 0.0 1.0").unwrap();
    buffer.write_all(&string.into_bytes())
}