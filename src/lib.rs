use anyhow::{Result, anyhow};
use std::{
    fs::{self, File},
    io::Write,
};
pub fn read(path: &str) -> Result<Option<String>> {
    if fs::exists(path)? {
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

pub fn append(path: &str, text: String) -> Result<()> {
    let mut logbook = File::options().create(true).append(true).open(path)?;
    writeln!(logbook, "{}", text)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
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
            let path = "tests/data/newlog.txt";
            assert!(!fs::exists(path).unwrap(), "Path must not already exist");

            let mut test_text = "hello logbook".to_string();
            append(&path, test_text.clone()).unwrap();

            let file_text = fs::read_to_string(path).unwrap();
            test_text.push('\n');
            assert_eq!(
                file_text, test_text,
                "Text must match input text, plus a newline"
            );
            //cleanup
            fs::remove_file(path).unwrap();
        }
    }
}
