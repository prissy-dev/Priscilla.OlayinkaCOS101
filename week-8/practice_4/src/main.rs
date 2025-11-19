fn main(){

	//Name vector
	let name = vec!["Mary","Luke","Sally","Grey","Ade","Anthony","June","Ife"];

	//Age vector
	let age = vec![16,17,19,22,20,21,18,23];

	print!("\nAge allocation:\n");

	//loop to iternate element in vector
	for i in 0..age.len(){
		//iternating through i on vector
		print!("{} is {} years old\n",name[i],age[i]);
	}
}