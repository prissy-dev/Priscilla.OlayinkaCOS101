
fn main() {
    
    let candidates = vec![
        ("John", 20),
        ("Ene", 11),
        ("Peter", 28),
        ("Hamza", 8),
        ("Blessing", 13),
    ];

    
    println!("Candidates list = {:?}", candidates);

    let mut max_candidate = candidates[0];
    for i in 0..candidates.len() {
        if candidates[i].1 > max_candidate.1 {
            max_candidate = candidates[i];
        }
    }

    
    let (name, years) = max_candidate;

    // Print result
    println!("The candidate with the highest experience is {} with {} years.", name, years);

    
    println!("Accessing tuple elements directly: Name = {}, Years = {}", max_candidate.0, max_candidate.1);
}
