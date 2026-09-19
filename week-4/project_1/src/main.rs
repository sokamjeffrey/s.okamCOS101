// Rust program to calculate the roots of a 
// quadratic equation for given a, b, and c

use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    let d: f32 = (b * b) - (4.0 * a * c);

    println!("\nResults");
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("Discriminant (d) = {}", d);

    if d > 0.0 {
        let root1: f32 = (-b + d.sqrt()) / (2.0 * a);
        let root2: f32 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct real roots found:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if d == 0.0 {
        let root: f32 = -b / (2.0 * a);
        println!("Exactly one real root found:");
        println!("Root = {}", root);
    } else {
        println!("No real roots exist (d is negative).");
    }
}