use std::io;
use std::io::Read;
use std::fs::File;
use bcrypt::verify;

fn main() {
    
    let my_pass = "cos101";
    
    println!("Enter System Password:");
    let mut pass_input = String::new();
    io::stdin().read_line(&mut pass_input).expect("Error reading");
    let pass_input = pass_input.trim();


    if pass_input == my_pass {
        println!("Correct! Now enter your role:");
        
        let mut user_role = String::new();
        io::stdin().read_line(&mut user_role).expect("Error reading");
        let user_role = user_role.trim().to_lowercase();

    
        if user_role == "administrator" {
            read_my_file("globacom_dbase.sql"); 
        } 
        else if user_role == "project manager" {
            read_my_file("project_tb.sql"); 
        } 
        else if user_role == "employee" {
            read_my_file("staff_tb.sql"); 
        } 
        else if user_role == "customer" {
            read_my_file("customer_tb.sql"); 
        } 
        else if user_role == "vendor" {
            read_my_file("dataplan_tb.sql"); 
        } 
        else {
            println!("That role does not exist.");
        }
    } else {
        println!("Wrong password. Goodbye.");
    }
}


fn read_my_file(name: &str) {
    let mut file = File::open(name).unwrap(); 
    let mut contents = String::new(); 
    file.read_to_string(&mut contents).unwrap(); 
    print!("{}", contents); //
}