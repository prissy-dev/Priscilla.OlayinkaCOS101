use std::io::Write;

fn main() {
    
    let lager = "LAGER: 33 Export, Desperados, Goldberg, Gulder, Heineken, Star\n"; 
    let stout = "STOUT: Legend, Turbo King, Williams,Lion,Eagle,Guinness\n"; 
    let non_alcoholic = "NON-ALCOHOLIC: Maltina, Amstel Malta, Malta Gold, Fayrouz,Maltex,Dubic\n"; 

    
    let mut file = std::fs::File::create("brewery.txt").expect("create failed"); 


    file.write_all("NIGERIAN BREWERIES PLC - DRINK CATEGORIES\n\n".as_bytes()).expect("write failed"); 
    file.write_all(lager.as_bytes()).expect("write failed"); 
    file.write_all(stout.as_bytes()).expect("write failed"); 
    file.write_all(non_alcoholic.as_bytes()).expect("write failed"); 

    println!("Project I: Drink categories saved to file");
}