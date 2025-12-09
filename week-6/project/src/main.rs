use std::io;

fn main() {
    

    println!("Menu:");
    println!("P - Poundo Yam/Edinkaiko Soup: ₦3200");
    println!("F - Fried Rice & Chicken: ₦3000");
    println!("A - Amala & Ewedu Soup: ₦2500");
    println!("E - Eba & Egusi Soup: ₦2000");
    println!("W - White Rice & Stew: ₦2500");
    println!("Enter orders like: P 3 (food code and quantity), X to finish.");

    let mut orders: Vec<(&str, i32)> = Vec::new();
    let mut total = 0;

    loop {
        let mut input = String::new();
        print!("> ");
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();
        if trimmed.to_uppercase() == "X" {
            break;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Please enter code and quantity like: P 2");
            continue;
        }

        let code = parts[0].to_uppercase();
        let qty: i32 = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid quantity!");
                continue;
            }
        };

        if code == "P" {
            orders.push(("Poundo Yam/Edinkaiko Soup", 3200));
            total += 3200 * qty;
        } else if code == "F" {
            orders.push(("Fried Rice & Chicken", 3000));
            total += 3000 * qty;
        } else if code == "A" {
            orders.push(("Amala & Ewedu Soup", 2500));
            total += 2500 * qty;
        } else if code == "E" {
            orders.push(("Eba & Egusi Soup", 2000));
            total += 2000 * qty;
        } else if code == "W" {
            orders.push(("White Rice & Stew", 2500));
            total += 2500 * qty;
        } else {
            println!("Invalid food code!");
        }
    }

    println!("\nOrder Summary:");
    for (name, price) in &orders {
        println!("{} = ₦{}", name, price);
    }

    if total > 10000 {
        println!("Discount applied (5%)");
        total = (total as f64 * 0.95) as i32;
    }

    println!("Total: ₦{}", total);
}