//Quadratic roots

use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

println!("Enter the value for a: ");
io::stdin().read_line(&mut input_a).expect("Input a number ma");


println!("Enter the value for b: ");
io::stdin().read_line(&mut input_b).expect("Input a number ma");


println!("Enter the value for c: ");
io::stdin().read_line(&mut input_c).expect("Input a number ma");
    
    let a:f64 = input_a.trim().parse().expect("Please input a valid number ma");
let b:f64 = input_b.trim().parse().expect("Please input a valid number ma");
let c:f64 = input_c.trim().parse().expect("Please input a valid number ma");

    let d = b*b - 4.0*a*c;
    if d>0.0{
        println!("two distinct roots");
    }

    if d==0.0{
        println!("exactly one real root");
    }

    if d<0.0{
        println!("no real roots");
    }
}
