// fn main() {
//     println!("{:?}", std::env::args());
// }

use clap::{App, Arg};   //import the clap::App struct 

fn main() {
    let matches = App::new("echor")    //create new App 
    .version("0.1.0")   //set version
    .author("Thanh")
    .about("Rust echo")   //set about
    // .get_matches()   //get matches
    .arg( 
        Arg::with_name("text")       //crate new Arg with name 'text'
        .value_name("TEXT")
        .help("Input text")
        .required(true)
        .min_values(1),
    )
    .arg(
        Arg::with_name("omit_newline") //crate new Arg with name 'omit_newline'
        .short("n")
        .help("Do not print newline Thanh")
        .takes_value(false),
    )
    .get_matches();

    // println!("{:#?}", matches);     //pretty print 

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    // let mut ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });  //print not add newline to the output
}