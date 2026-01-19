use std::io::Write;

fn main() {
    
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", "300"), 
        ("Adams Aliyu", "ECO10110101", "Economics", "100"), 
        ("Shania Bolade", "CSC10328828", "Computer", "200"), 
        ("Adekunle Gold", "EEE11020202", "Electrical", "200"), 
        ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"), 
    ];

    let mut file = std::fs::File::create("pau_smis.txt").expect("create failed"); 

    
    let header = "Student Name | Matric Number | Department | Level\n"; 
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all("----------------------------------------------------\n".as_bytes()).unwrap();

    
    for student in students {
        let row = format!("{} | {} | {} | {}\n", student.0, student.1, student.2, student.3);
        file.write_all(row.as_bytes()).expect("write failed"); 
        
        
        println!("{}", row); 
    }

    println!("\nProject II: Student details saved to file");
}