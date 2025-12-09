fn main() {
	let A:f32 = 10.0;
	let B:f32 = 20.0;

	println!("Value of A {}",A);
	println!("Value of B {}",B);

	let mut res = A>B;
	println!("A greater than B: {}",res);

	res = A<B;
	println!("A less than B: {}",res);
	
	res = A>=B;
	println!("A greater than  or equal to B: {}",res);

	res = A<=B;
	println!("A lesser than  or equal to B: {}",res);

	res = A==B;
	println!("A equal to B: {}",res);

	res = A!=B;
	println!("A not equal B: {}",res);



	
}