use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {   // Define a struct to hold the configuration options
    files: Vec<String>,     // A vector to hold the list of files to be processed
    lines: i32,     // The number of lines to display
    bytes: i32,    // The number of bytes to display
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Hugo")
        .about("Rust head")
        .arg(Arg::with_name("files")
            .value_name("FILE")
            .help("Input files")
            .multiple(true)         // Allow multiple files to be specified
            .default_value("-"),
        )
        .arg(Arg::with_name("lines")
            .short("n")
            .long("lines")
            .value_name("LINES")
            .help("Number of lines to display")
            .takes_value(true)
        )
        .arg(Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .value_name("BYTES")
            .help("Number of bytes to display")
            .takes_value(true)
            .conflicts_with("lines"),
        )
        .get_matches();

    let lines = matches
                .value_of("lines")
                .map(parse_positive_int)   // If the lines argument is provided, attempt to parse it as a positive integer using the parse_positive_int function
                .transpose()   // If parsing is successful, transpose the result to get a MyResult<usize> value. If parsing fails, return the error.
                .map_err(|e| format!("illegal line count: {e}"))?;   // If parsing fails, map the error to a more user-friendly error message indicating that the value for lines is invalid

    let bytes = matches
                .value_of("bytes")
                .map(parse_positive_int)   // If the bytes argument is provided, attempt to parse it as a positive integer using the parse_positive_int function
                .transpose()   // If parsing is successful, transpose the result to get a MyResult<usize> value. If parsing fails, return the error.
                .map_err(|e| format!("illegal byte count: {e}"))?;   // If parsing fails, map the error to a more user-friendly error

    Ok(Config {         // Create a Config struct instance and populate it with the values from the command-line arguments
        files: matches.values_of_lossy("files").unwrap(),  
        lines: lines.unwrap_or(10),   // If the lines argument is not provided, use a default value of 10
        bytes: bytes.unwrap_or(0),   // If the bytes argument is not provided, use a default value of 0
    })
}


pub fn run(config: Config) -> MyResult<()> {
    println!("Execute head function!");
    println!("Config: {:#?}", config); 
    
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate(){
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {} <==",
                            if file_num > 0 {"\n"} else {""},
                            filename
                    );
                }                
                if config.bytes > 0 {
                    let mut handle = file.take(config.bytes as u64);   // If the bytes field in the config struct is set, create a new reader that reads only the specified number of bytes from the file
                    let mut buffer = vec![0; config.bytes as usize];   // Create a buffer to hold the bytes read from the file
                    let bytes_read = handle.read(&mut buffer)?;   // Read bytes from the file
                    
                    print!(
                        "{}",
                        String::from_utf8_lossy(&buffer[..bytes_read])
                    );   // Print the bytes read from the file to the console, converting them to a string using from_utf8_lossy to handle any invalid UTF-8 sequences
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines.max(0) as usize {      // Iterate over the lines in the file, 0 to the number of lines
                        let bytes = file.read_line(&mut line)?;   // Read a line from the file and store it in the line variable. The number of bytes read is stored in the bytes variable
                        if bytes == 0 {    // If the number of bytes read is 0, it means we have reached the end of the file, so break out of the loop
                            break;
                        }
                        
                        print!("{}", line);   // Print the line to the console
                        line.clear();    // Clear the line variable for the next iteration
                    }
                }
            }
        }
    }
    Ok(())
}


fn parse_positive_int(val: &str) -> MyResult<i32> {
    //About: attempts to parse a string value into a positive i32 value
    // unimplemented!()   //  unimplemented! macro 
    match val.parse::<i32>(){
        Ok(num) if num > 0 => Ok(num),   // If parsing is successful and the number is greater than 0, return the parsed number wrapped in an Ok variant
        _ => Err(From::from(val)),   // If parsing fails or the number is not greater than 0, return an error with a message indicating that the input is not a positive integer
    }
}

fn open (filename: &str) -> MyResult<Box<dyn std::io::BufRead>> {
    //About: attempts to open a file and return a buffered reader for it
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),   // If the filename is "-", return a buffered reader for standard input
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),   // Otherwise, attempt to open the specified file and return a buffered reader for it. If opening the file fails, return an error.
    }
}

// #[test]
// fn test_parse_positive_int() {
//     // 3 is an OK integer
//     let res = parse_positive_int("3");
//     assert!(res.is_ok());
//     assert_eq!(res.unwrap(), 3);
//     // Any string is an error
//     let res = parse_positive_int("foo");
//     assert!(res.is_err());
//     assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
//     // A zero is an error
//     let res = parse_positive_int("0");
//     assert!(res.is_err());
//     assert_eq!(res.unwrap_err().to_string(), "0".to_string());
// }   