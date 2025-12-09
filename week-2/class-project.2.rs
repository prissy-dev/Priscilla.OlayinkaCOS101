fn main() {
    
    let sales_amounts: [f64; 5] = [
        450_000.00,
        1_500_000.00,
        750_000.00,
        2_850_000.00,
        250_000.00,
    ];

    
    let total_sum: f64 = sales_amounts.iter().sum();

    
    let count: usize = sales_amounts.len(); 
    let average: f64 = total_sum / (count as f64); 

    
    println!("Sales Record (Amounts): {:?}", sales_amounts);
    
    println!("Total Number of Items: {}", count);
    println!("Total Sales Sum: N{:.2}", total_sum);
    println!("Average Sales Amount: N{:.2}", average);
    
}
