use anyhow::Result;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
pub fn read(path: impl AsRef<Path>) -> Result<Option<String>> {
    if fs::exists(&path)? {
        let file_contents = fs::read_to_string(path)?;
        if file_contents.is_empty() {
            Ok(None)
        } else {
            Ok(Some(file_contents))
        }
    } else {
        Ok(None)
    }
}

pub fn append(path: impl AsRef<Path>, text: String) -> Result<()> {
    let mut logbook = File::options().create(true).append(true).open(path)?;
    writeln!(logbook, "{}", text)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;
    mod read {
        use super::*;
        #[test]
        fn returns_none_if_file_does_not_exist() {
            let text = read("tests/data/dne.txt").unwrap();
            assert_eq!(text, None, "expected None");
        }
        #[test]
        fn returns_none_if_file_exists_and_is_empty() {
            let text = read("tests/data/empty.txt").unwrap();
            assert_eq!(text, None, "file should be empty");
        }
        #[test]
        fn returns_file_contents_as_string() {
            let text = read("tests/data/some.txt").unwrap();
            let expected_output = Some("Data Read Successfully!".to_string());
            assert_eq!(text, expected_output, "File Read Incorrectly");
        }
    }
    mod write {
        use super::*;
        #[test]
        fn creates_file_if_necessary() {
            let dir = tempdir().unwrap();
            let path = dir.path().join("newlog.txt");

            let mut test_text = "hello logbook".to_string();
            append(&path, test_text.clone()).unwrap();

            let file_text = fs::read_to_string(path).unwrap();
            test_text.push('\n');
            assert_eq!(
                file_text, test_text,
                "Text must match input text, plus a newline"
            );
        }
    }
}
