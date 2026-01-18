
use std::fs;

fn main() {

    // Remove the specified file
    fs::remove_file("prissy.txt").expect("could not remove file");
    println!("file is removed");
}