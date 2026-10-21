use std::io::Write;

fn main() {
  
    let mut file = std::fs::File::create("nigerian_breweries.txt").expect("create failed");

    // Write the categories and their respective drinks to the file
    file.write_all("NIGERIAN BREWERIES PLC - DRINK CATEGORIES\n".as_bytes()).expect("write failed");

    
    file.write_all("Lager:\n".as_bytes()).expect("write failed");
    file.write_all("33 Export, Desperados, Goldberg, Gulder, Heineken, Star".as_bytes()).expect("write failed");
    
    file.write_all("Stout:\n".as_bytes()).expect("write failed");
    file.write_all("Legend, Turbo King, Williams,".as_bytes()).expect("write failed");
    
    file.write_all("Non-Alcoholic:\n".as_bytes()).expect("write failed");
    file.write_all("Maltina, Amstel Malta, Malta Gold, Fayrouz".as_bytes()).expect("write failed");

    println!("Data written to file successfully.");
}