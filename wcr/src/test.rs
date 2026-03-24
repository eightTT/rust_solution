use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
// The count function will accept a mutable file value, and it might return a
// FileInfo struct.
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();       // a mutable buffer to hold new line text

    loop{
        let line_bytes = file.read_line(&mut line)?;    // read line from file handle
        if line_bytes == 0 {
            break;          //break when there is end of line
        }
        num_bytes += line_bytes;
        num_lines += 1
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();  
        line.clear();       // clear line buffer
    }

    Ok(FileInfo {
            num_lines,
            num_words,
            num_bytes,
            num_chars,
        }
    )
}


#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;
    #[test]
        fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
            };
        assert_eq!(info.unwrap(), expected);
    }
}