fn main() {
    //initiating a mutable tuple
    let mut mountain_height = ("Everest", 9876, "Fishtail", 8579);
}   
println!("Original tuple = {:?}",mountain_heights);

//change 3rd and 4th element of a mutable tuple
mountain_heights.2 = "Lhotse";
mountain_heights.3 = 6789;

println!("Changed tuple = {:?}", mountain_heights);


