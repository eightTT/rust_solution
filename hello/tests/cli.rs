// #[test]      //attribute to mark this function as a test
// fn works(){
//     assert!(true);  // assert macro check Bool expr is true 
// }


// use std::process::Command;    // standard lib 
// #[test]
// fn run() {
//     let mut cmd = Command::new("hello");    // create new command to run ls , let to bind, mut to mutable 
//     let res = cmd.output();
//     assert!(res.is_ok());   // check if the result is OK variant 
// }


use assert_cmd::Command;    // use assert_cmd crate for testing command line applications
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap(); // create a command to run hello in current crate
    cmd.assert().success().stdout("Hello, world!\n"); // assert that the command runs successfully and outputs "Hello, world!\n"
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap(); 
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap(); 
    cmd.assert().failure();
}