use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Open file with append option set to true
    let mut file = OpenOptions::new().append(true).open("prissy.txt")
        .expect("cannot open file");
    
    // Add new lines to the end of the file
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("write failed");
    
    println!("file append success");
}