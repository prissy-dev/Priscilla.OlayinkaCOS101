fn main() {

	//Using Vec::new()
	let v : Vec<i64> = Vec::new();

	//printing the size of vector
	println!("\nThe length of Vec::newis: {}",v.len());

	//Using macro
	let v = vec!["Grace","Effiong","Basil","Kareem","SUsan"];

	//printing the size of the vector
	println!("\nThe lengthof vec macro is: {}",v.len());
}