fn main () {
	println!("Hello Mr Chudi! After a lot of struggling, I finally got it to work. I'm really looking forward to doing more of this!");
let p:f64 = 210_000.0;
let r:f64 = 5.0;
let n:f64 = 3.0;

let a = p * (1.0 - (r/100.0)).powf(n);
println!("Amount is = {}", a);
}