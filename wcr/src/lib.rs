use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


// Define a type alias MyResult 
// represents a Result type where: the Ok variant can hold any type T 
// the Err variant can hold any type that implements the Error trait.
// functions can return MyResult<T> to indicate that they may return an error of any type that implements the Error trait.
type MyResult<T> = Result<T, Box<dyn Error>>; 

#[derive(Debug)]
// Define a struct to hold the configuration options
pub struct Config {
    files: Vec<String>,     // A vector to hold the list of files to be processed
    line: bool,             // indicate whether or not to print the 'line' count.
    word: bool,             // indicate whether or not to print the 'word' count.
    byte: bool,             // indicate whether or not to print the 'byte' count.
    chars: bool,            // indicate whether or not to print the 'character' count.
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}


pub fn get_args() -> MyResult<Config> {
    //function to parse command-line arguments and return a Config struct 
    // instance populated with the values from the command-line arguments
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Hugo")
        .about("Rust words count from input files from cmdline")
        .arg(Arg::with_name("files")
            .value_name("FILE")
            .help("Input files")
            .multiple(true)         // Allow multiple files to be specified
            .default_value("-"),)
        .arg(Arg::with_name("line")
            .short("l")
            .long("lines")
            .help("Number of lines")
            .takes_value(false),)
        .arg(Arg::with_name("word")
            .short("w")
            .long("words")
            .help("Number of words")
            .takes_value(false),)
        .arg(Arg::with_name("byte")
            .short("c")
            .long("bytes")
            .help("Number of bytes")
            .takes_value(false),)
        .arg(Arg::with_name("chars")
            .short("m")
            .long("chars")
            .help("Number of characters")
            .takes_value(false),)
        .get_matches();

    let mut line = matches.is_present("line");
    let mut word = matches.is_present("word");
    let mut byte = matches.is_present("byte");
    let mut chars = matches.is_present("chars");

    if [line, word, byte, chars].iter().all(|v| v == &false) {
        // if all flags are false, set them all to true
        line = true;
        word = true;
        byte = true;
        // chars = true;
    }

    Ok(Config {         
        // Create a Config struct instance and populate it with the values from the command-line arguments
        files: matches.values_of_lossy("files").unwrap(),     // at least 1 value required, so unwrap is safe
        line,
        word,
        byte,
        chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename{
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:#?}", config);

    for filename in &config.files{
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => 
                {
                    println!("Open {}", filename);
                    if let Ok(info) = count(file) {
                        println!("FileInfo: {:#?}", info);
                    }
                }
        }
    }

    Ok(())
}



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
        num_lines += 1;
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