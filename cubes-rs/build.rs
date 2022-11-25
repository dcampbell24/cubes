use directories::ProjectDirs;

use std::fs;
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "Cubes") {
        let dir = proj_dirs.data_dir();
        let path = dir.join("puzzles");
        fs::create_dir_all(&path)?;
        fs::copy(Path::new("puzzles").join("blue"), path.join("blue"))?;
        fs::copy(Path::new("puzzles").join("minotaur"), path.join("minotaur"))?;
        fs::copy(Path::new("puzzles").join("orange"), path.join("orange"))?;
        fs::copy(Path::new("puzzles").join("red"), path.join("red"))?;
        fs::copy(Path::new("puzzles").join("white"), path.join("white"))?;
        fs::copy(Path::new("puzzles").join("yellow"), path.join("yellow"))?;
    }

    Ok(())
}
