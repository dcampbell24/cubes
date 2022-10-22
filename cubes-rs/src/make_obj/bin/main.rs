use std::fmt::Write;
use std::fs::File;
use std::io::Write as _;


fn main()-> std::io::Result<()> {
    let puzzle = cubes_rs::minotaur();

    for (i, piece) in puzzle.iter().enumerate() {
        let mut buffer = File::create(format!("target/piece{}.obj", i))?;
        let mut string = String::new();
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

    Ok(())
}