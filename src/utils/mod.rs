use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};

/// Reads the entire contents of a file and returns it as a [`String`].
///
/// # Errors
///
/// Returns an [`io::Error`] if the file does not exist or cannot be read.
pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Reads a file line by line and returns a vector of [`String`]s.
///
/// # Errors
///
/// Returns an [`io::Error`] if the file cannot be opened or if any line
/// cannot be read as UTF-8.
pub fn read_file_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io, path::PathBuf};

    fn temp_file_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("plug_and_hack_{name}"))
    }

    #[test]
    fn reads_entire_file() -> io::Result<()> {
        let path = temp_file_path("read_file_to_string.txt");
        fs::write(&path, "hello\nworld")?;

        let contents = read_file_to_string(&path)?;

        fs::remove_file(&path).ok();
        assert_eq!(contents, "hello\nworld");
        Ok(())
    }

    #[test]
    fn reads_file_lines() -> io::Result<()> {
        let path = temp_file_path("read_file_lines.txt");
        fs::write(&path, "line1\nline2\nline3\n")?;

        let lines = read_file_lines(&path)?;

        fs::remove_file(&path).ok();
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
        Ok(())
    }
}
