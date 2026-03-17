fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);         //print the error message
        std::process::exit(1);      //exit program with non-zero value to indicate an error
    }
}
