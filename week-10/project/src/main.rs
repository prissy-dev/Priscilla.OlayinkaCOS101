struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    // Method to calculate total for 3 units
    fn total_cost(&self) -> u32 {
        self.price * 3
    }
}

fn main() {
    // Inventory based on notes [cite: 520]
    let hp = Laptop { brand: String::from("HP"), price: 650_000 };
    let ibm = Laptop { brand: String::from("IBM"), price: 755_000 };
    let toshiba = Laptop { brand: String::from("Toshiba"), price: 550_000 };
    let dell = Laptop { brand: String::from("Dell"), price: 850_000 };

    // Calculate sum of 3 units from each brand
    let grand_total = hp.total_cost() + ibm.total_cost() + 
                      toshiba.total_cost() + dell.total_cost();

    println!("Total cost for 3 units of each brand: NGN {}", grand_total);
}
