fn main() {
    let t: f64 = 450000.0;
    let m: f64 = 1500000.0;
    let h: f64 = 750000.0;
    let d: f64 = 2850000.0;
    let a: f64 = 250000.0;

    //sum
    let s = m + a + (t*2.0) + (h*3.0) + (d*3.0);
    println!("Sum is {}", s);
    let av = s / 10.0;
    println!("Average is {}", av);
}