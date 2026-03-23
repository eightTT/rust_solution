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


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Hugo")
        .about("Rust wc")
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
    Ok(Config {         
        // Create a Config struct instance and populate it with the values from the command-line arguments
        files: matches.values_of_lossy("files").unwrap(),     // at least 1 value required, so unwrap is safe
        line: matches.is_present("line"),      //either present or not
        word: matches.is_present("word"),
        byte: matches.is_present("byte"),
        chars: matches.is_present("chars"),
    })
}