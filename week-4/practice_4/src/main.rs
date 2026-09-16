use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name jo: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Abeg Enter your age fast: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 18 {
        println!("You fit go for the beach party {}!", input1);
    } else {
        println!("You are too Young to go for this party {}", input1);
    }
}
