use std::io;

fn main() {
    println!("STELLA'S REST. MENU");
    println!("(P) Poundo Yam / Edinkaiko Soup -N3,200");
    println!("F - Fried Rice & Chicken        -N3,000");
    println!("A - Amala & Ewedu Soup          -N2,500");
    println!("E - Eba & Egusi Soup            -N2,000");
    println!("W - White Rice & Stew           -N2,500\n");
    
    println!("Enter food type girlll (P, F, A, E, W):");
    let mut food_input = String::new();
    io::stdin().read_line(&mut food_input).expect("Please choose from the given letters, thanksiess");
    let food_type:char = food_input.trim().parse().expect("Please enter a single letter");

    let mut price = 0.0;
    if food_type == 'P' || food_type == 'p' {
        price = 3200.0;
    } else if food_type == 'F' || food_type == 'f' {
        price = 3000.0;
    } else if food_type == 'A' || food_type == 'a' {
        price = 2500.0;
    } else if food_type == 'E' || food_type == 'e' {
        price = 2000.0;
    } else if food_type == 'W' || food_type == 'w' {
        price = 2500.0;
    } else {
        println!("Invalid food choice");
        return; // Stops the program here if they type a wrong letter
    }

    println!("Enter quantity:");
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str).expect("Girllll");
    
    // Convert the quantity string into a usable float number
    let quantity: f64 = quantity_str.trim().parse().expect("Please enter a number for quantity");

    let mut total = price * quantity;
    println!("\nSubtotal: N{:.2}", total);

    if total > 10000.0 {
        let discount = total * 0.05;
        total -= discount;
        println!("You've been granted a 5% discount love, Have a nice dayy!");
    }

    println!("Total Charge: N{:.2}", total);
}