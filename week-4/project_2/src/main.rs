// Rust program to calculate the annual incentive
// based on an employee's experience and age

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    // Get experience status from user
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let is_experienced = input1.trim().to_lowercase();

    // Get age from user
    println!("Enter age: ");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let age: u32 = input2.trim().parse().expect("Please enter a valid number");

    // Determine the incentive based on criteria
    if is_experienced == "yes" {
        if age >= 40 {
            println!("Incentive: N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Incentive: N1,480,000");
        } else if age < 28 {
            println!("Incentive: N1,300,000");
        } else {
            println!("No specific incentive criteria matched for this age group.");
        }
    } else {
        println!("Incentive: N100,000");
    }
}