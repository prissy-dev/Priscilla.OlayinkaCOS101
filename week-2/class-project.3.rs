fn main() {
    
    let principal_cost: f64 = 510_000.0; 
    let rate_of_depreciation: f64 = 5.0;  
    let time_years: i32 = 3;             

    
    let rate_decimal: f64 = rate_of_depreciation / 100.0;

 
    let final_value = principal_cost * (1.0 - rate_decimal).powi(time_years);

    
    println!("Initial Cost (P): N{:.2}", principal_cost);
    println!("Depreciation Rate (R): {}%", rate_of_depreciation);
    println!("Time Period (t): {} years", time_years);
    
    println!("Value of the TV after {} years (A): N{:.2}", time_years, final_value);
}