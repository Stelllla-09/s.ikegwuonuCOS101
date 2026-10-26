struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop { brand: String::from("HP"), price: 650000 };
    let ibm = Laptop { brand: String::from("IBM"), price: 755000 };
    let toshiba = Laptop { brand: String::from("Toshiba"), price: 550000 };
    let dell = Laptop { brand: String::from("Dell"), price: 850000 };

    let quantity = 3;

    let hp_cost = hp.cost(quantity);
    let ibm_cost = ibm.cost(quantity);
    let toshiba_cost = toshiba.cost(quantity);
    let dell_cost = dell.cost(quantity);

    println!("{} x {} = N{}", hp.brand, quantity, hp_cost);
    println!("{} x {} = N{}", ibm.brand, quantity, ibm_cost);
    println!("{} x {} = N{}", toshiba.brand, quantity, toshiba_cost);
    println!("{} x {} = N{}", dell.brand, quantity, dell_cost);

    let total = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("Total cost = N{}", total);
}