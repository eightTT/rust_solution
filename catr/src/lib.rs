use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
// Define a struct to hold the configuration options
// This struct will be populated based on the command-line arguments provided by the user
pub struct Config {
    files: Vec<String>,     // A vector to hold the list of files to be processed
    number_lines: bool,     // A boolean flag = whether to number all lines
    number_nonblank_lines: bool,    // A boolean flag = to number only non-blank lines
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Hugo")
        .about("Rust cat")
        .arg(Arg::with_name("files")
            .value_name("FILE")
            .help("Input files")
            .multiple(true)         // Allow multiple files to be specified
            .required(true)         // Make the files argument required
            .default_value("-"),)
        .arg(Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Number lines")
            .takes_value(false)
            .conflicts_with("number_nonblank"),)
        .arg(Arg::with_name("number_nonblank")
            .short("b")
            .long("number-nonblank")
            .help("Number non-blank lines")
            .takes_value(false)
        )
        .get_matches();
    Ok(Config {         // Create a Config struct instance and populate it with the values from the command-line arguments
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {   // function need to accept config argument
    // println!("Hello, world!");
    // dbg!(config);   //debug print the config struct
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => //read_content(file)?,    // If the file is successfully opened, read its content and print it to the console
                {
                    let mut last_num = 0;           // Keep track of the last line number for non-blank lines
                    for (line_num, line) in file.lines().enumerate() {   // Iterate over each line in the file
                        let line = line?;                                // Handle any errors that occur while reading the line
                        if config.number_lines {
                            println!("{:>6}\t{}", line_num + 1, line);  // If the number_lines flag is set, print the line number and the line content
                        } else if config.number_nonblank_lines {
                            if !line.is_empty() {                       // If the number_nonblank_lines flag is set and the line is not empty, print the line number and the line content
                                last_num += 1;
                                println!("{:>6}\t{}", last_num, line); 
                            } else {
                                println!();                             // If the line is empty, just print a blank line without a line number
                            }
                        } else {
                            println!("{}", line);               // If neither flag is set, just print the line content without any line numbers
                        }
                    }
                }
        }
    }
    Ok(())
}   




fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


fn read_content(mut reader: Box<dyn BufRead>) -> MyResult<()> {
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        print!("{}", line);
        line.clear();
    }
    Ok(())
}
