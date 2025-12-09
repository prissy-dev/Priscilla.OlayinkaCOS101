use std::io;

fn main() {
    let mut results: [f64; 5] = [0.0; 5]; // Array to store results

    println!("Select a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: usize = choice.trim().parse().expect("Invalid input");

    if choice == 1 {
        let h = read_input("Enter height: ");
        let b1 = read_input("Enter base1: ");
        let b2 = read_input("Enter base2: ");
        results[0] = area_trapezium(h, b1, b2);
        println!("Area of Trapezium = {}", results[0]);
    } else if choice == 2 {
        let d1 = read_input("Enter diagonal1: ");
        let d2 = read_input("Enter diagonal2: ");
        results[1] = area_rhombus(d1, d2);
        println!("Area of Rhombus = {}", results[1]);
    } else if choice == 3 {
        let base = read_input("Enter base: ");
        let alt = read_input("Enter altitude: ");
        results[2] = area_parallelogram(base, alt);
        println!("Area of Parallelogram = {}", results[2]);
    } else if choice == 4 {
        let side = read_input("Enter side length: ");
        results[3] = area_cube(side);
        println!("Area of Cube = {}", results[3]);
    } else if choice == 5 {
        let r = read_input("Enter radius: ");
        let h = read_input("Enter height: ");
        results[4] = volume_cylinder(r, h);
        println!("Volume of Cylinder = {}", results[4]);
    } else {
        println!("Invalid choice!");
    }

   
}

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid number")
}

fn area_trapezium(h: f64, b1: f64, b2: f64) -> f64 {
    (h / 2.0) * (b1 + b2)
}

fn area_rhombus(d1: f64, d2: f64) -> f64 {
    0.5 * d1 * d2
}

fn area_parallelogram(base: f64, alt: f64) -> f64 {
    base * alt
}

fn area_cube(side: f64) -> f64 {
    6.0 * side * side
}

fn volume_cylinder(r: f64, h: f64) -> f64 {
    std::f64::consts::PI * r * r * h
}