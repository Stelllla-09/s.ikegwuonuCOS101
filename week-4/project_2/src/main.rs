//The Incentive Calculator

use std::io;
fn main() {
let mut employee_age = String::new();
let mut employee_experience = String::new();
println!("Input Employee's age: ");
io::stdin().read_line(&mut employee_age).expect("Please enter your correct age");

println!("Input Employee's experience(experienced or not)");
io::stdin().read_line(&mut employee_experience).expect("Please enter your experience");

let age:f64= employee_age.trim().parse().expect("Please enter a numeric age");
let experience= employee_experience.trim();


if experience.contains("not"){
    println!("Then annual incentive is: #100_000.00");
}

else if age>=40.0{
    println!("Then annual incentive is: #1_560_000.00");
}
   
else if age>=30.0 && age<=39.0{
    println!("Then annual incentive is: #1_480_000.00");
}

 
 else if age<28.0{
    println!("Then annual incentive is: #1_300_000.00");
}

else {
    println!("Age does not fall into any incentive range");
    }
}