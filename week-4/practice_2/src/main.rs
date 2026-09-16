
use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first angle of triangle: ");
    io::stdin()
    .read_line(&mut input1)
    .expect("Not A Valid Number");
    let a:f32 = input1.trim().parse().expect("Not a valid number");
    
    println!("Enter Second angle of triangle: ");
    io::stdin()
    .read_line(&mut input2)
    .expect("Not A Valid Number");
    let b:f32 = input2.trim().parse().expect("Not a valid number");
    
    println!("Enter third angle of triangle: ");
    io::stdin()
    .read_line(&mut input3)
    .expect("Not A Valid Number");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

let s:f32 = (a + b + c) / 2.0;
let area:f32 = s * (s - a) * (s - b) * (s - c);
let _area = area.sqrt();

println!("Area Of A Triangle");
}
