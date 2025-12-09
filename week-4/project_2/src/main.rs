use std::io;

fn main() {

    
    println!("Employee Annual Incentive Calculator");


   
    println!("Is the employee experienced? (yes/no):");
    let mut experience_input = String::new();
    
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let is_experienced = experience_input.trim().to_lowercase() == "yes";

    println!("Enter your employee age");
    let age: u32 = read_input();

    let incentive = if is_experienced {
        if age >= 40 {
            1_560_000
        }else if age >=30{
            1_480_000
        }else if age >= 28 {
            1_300_000
        } else {
            println!("Note: No specific criteria for experienced employees aged 28-29");
            1_300_000
            }

    }  else {
        100_000
      };

    println!("\nEmployee Details");
    println!("experienced: {}", if is_experienced { "Yes" } else{ "No" });
    println!("Age: {}",age);
    println!("Annual Incentive: N{:}",incentive);

    fn read_input() -> u32 {
        let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")

    }


    

}