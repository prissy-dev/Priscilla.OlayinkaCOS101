use std::io;

fn main() {
    println!("Quadratic Equation Solver:ax^2 + bx + c = 0");

    println!("Enter coefficient a:");
    let a: f64  = read_input();

    println!("Enter coefficient b:");
    let b: f64  = read_input();

    println!("Enter coefficient c:");
    let c: f64  = read_input();

    let discriminant = b * b - 4.0 * a * c;

    println!("\nEquation: {}x^2 + {}x + {} = 0", a, b, c);
    println!("discriminant: {}",discriminant);

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b + discriminant.sqrt()) / (2.0 * a);
        println!("Two diatinct real roots:");
        println! ("Root 1: {:.2}", root1);
        println! ("Root 2: {:.2}", root2);

    
} else if discriminant == 0.0 {

    let root = -b / (2.0 * a);
    println!("One real root (repeated):");
    println!("Root: {:.2}", root);
    }else {
    let real_part = -b / (2.0 * a);
    let imaginary_part = (- discriminant).sqrt() / (2.0 * a);
    println!("No real roots (complex roots):");

    println!("Root 1: {:.2} + {:.2}i", real_part, imaginary_part);
    println!("Root 2: {:.2} + {:.2}i", real_part, imaginary_part);


    

}
fn read_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}
}