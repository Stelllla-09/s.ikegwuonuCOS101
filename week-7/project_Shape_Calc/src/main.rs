use std::io;

fn trapezium_area() {
    let mut input1 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let height: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter base1:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1: f64 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter base2:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2: f64 = input3.trim().parse().expect("Invalid input");

    let area = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is: {}", area);
}

fn rhombus_area() {
    let mut input1 = String::new();
    println!("Enter diagonal1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let diagonal1: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter diagonal2:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let diagonal2: f64 = input2.trim().parse().expect("Invalid input");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {}", area);
}

fn parallelogram_area() {
    let mut input1 = String::new();
    println!("Enter base:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude: f64 = input2.trim().parse().expect("Invalid input");

    let area = base * altitude;
    println!("The area of the parallelogram is: {}", area);
}

fn cube_surface_area() {
    let mut input1 = String::new();
    println!("Enter side:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let side: f64 = input1.trim().parse().expect("Invalid input");

    let area = 6.0 * side * side;
    println!("The surface area of the cube is: {}", area);
}

fn cylinder_volume() {
    let mut input1 = String::new();
    println!("Enter radius:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height: f64 = input2.trim().parse().expect("Invalid input");

    let pi = 3.14159;
    let volume = pi * radius * radius * height;
    println!("The volume of the cylinder is: {}", volume);
}

fn main() {
    println!("Project: The Shape Calculator");
    println!("1. Trapezium, area");
    println!("2. Rhombus, area");
    println!("3. Parallelogram, area");
    println!("4. Cube, surface area");
    println!("5. Cylinder, volume");
    
    println!("Enter your choice (1-5):");
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read input");
    let choice: i32 = choice_input.trim().parse().expect("Invalid input");

    if choice == 1 {
        trapezium_area();
    } else if choice == 2 {
        rhombus_area();
    } else if choice == 3 {
        parallelogram_area();
    } else if choice == 4 {
        cube_surface_area();
    } else if choice == 5 {
        cylinder_volume();
    } else {
        println!("Invalid choice");
    }
}