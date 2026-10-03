use std::io;

fn main() {
    // Print the menu
    println!("Menu:");
    println!("P - Poundo Yam / Edinkaiko Soup : N3200");
    println!("F - Fried Rice & Chicken       : N3000");
    println!("A - Amala & Ewedu Soup         : N2500");
    println!("E - Eba & Egusi Soup           : N2000");
    println!("W - White Rice & Stew          : N2500");

    // Read food code
    println!("\nEnter Food Code (P, F, A, E, W):");
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    let code = code.trim();

    // Determine price using match
    let price = match code {
        "P" | "p" => 3200.0,
        "F" | "f" => 3000.0,
        "A" | "a" => 2500.0,
        "E" | "e" => 2000.0,
        "W" | "w" => 2500.0,
        _ => {
            println!("Invalid code!");
            return;
        }
    };

    // Read quantity
    println!("Enter Quantity:");
    let mut input_qty = String::new();
    io::stdin().read_line(&mut input_qty).unwrap();
    let qty: f64 = input_qty.trim().parse().unwrap();

    // Calculate total
    let mut total = price * qty;
    println!("Total: N{}", total);

    // Apply 5% discount if over 10,000
    if total > 10000.0 {
        let discount = total * 0.05;
        total = total - discount;
        println!("5% Discount Applied!");
        println!("Final Total to pay: N{}", total);
    }
}