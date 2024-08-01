use directories::ProjectDirs;

use std::fs;
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    if let Some(proj_dirs) = ProjectDirs::from("", "", "Cubes") {
        let dir = proj_dirs.data_dir();
        let path = dir.join("puzzles");
        fs::create_dir_all(&path)?;
        fs::copy(Path::new("puzzles").join("blue.ron"), path.join("blue.ron"))?;
        fs::copy(
            Path::new("puzzles").join("minotaur.ron"),
            path.join("minotaur.ron"),
        )?;
        fs::copy(
            Path::new("puzzles").join("orange.ron"),
            path.join("orange.ron"),
        )?;
        fs::copy(Path::new("puzzles").join("red.ron"), path.join("red.ron"))?;
        fs::copy(
            Path::new("puzzles").join("white.ron"),
            path.join("white.ron"),
        )?;
        fs::copy(
            Path::new("puzzles").join("yellow.ron"),
            path.join("yellow.ron"),
        )?;
        fs::copy(Path::new("puzzles").join("towo.ron"), path.join("towo.ron"))?;
    }

    Ok(())
}
