use std::io::Write;

fn main() {
    
    let names = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"]; 
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"]; 
    let zones = vec!["South West", "North East", "South South", "South West", "South East"]; 

    let mut file = std::fs::File::create("efcc_merged_data.txt").expect("create failed"); 

    file.write_all("EFCC CONVICTED MINISTERS REPORT\n".as_bytes()).unwrap();
    file.write_all("S/N | NAME | MINISTRY | GEOPOLITICAL ZONE\n".as_bytes()).unwrap();
    file.write_all("----------------------------------------------------------\n".as_bytes()).unwrap();

    // Merging the datasets using a loop
    for i in 0..5 {
        let record = format!("{}. {} | {} | {}\n", i + 1, names[i], ministries[i], zones[i]);
        file.write_all(record.as_bytes()).expect("write failed"); 
    }

    println!("Project III: Datasets successfully merged into file");
}
