fn main() {
    
    let principal: f64 = 520_000_000.0; 
    let rate: f64 = 10.0;               
    let time: i32 = 5;                  

   
    let rate_decimal: f64 = rate / 100.0;

    
    let amount = principal * (1.0 + rate_decimal).powi(time);

    
    let compound_interest = amount - principal;

    
    println!("Principal Amount (P): N{:2}", principal);
    println!("Annual Rate (R): {}%", rate);
    println!("Time Period (t): {} years", time);
    
    println!("Total Amount after 5 years (A): N{:2}", amount);
    println!("Compound Interest (CI): N{:2}", compound_interest);
    
}
