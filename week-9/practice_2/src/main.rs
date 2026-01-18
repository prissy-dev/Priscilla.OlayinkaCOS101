use std::io::Read;

fn main() {
    // Open an existing file
    let mut file = std::fs::File::open("prissy.txt").unwrap();
    let mut contents = String::new();
    
    // Read contents into the 'contents' string
    file.read_to_string(&mut contents).unwrap();
    
    print!("{}", contents);
}