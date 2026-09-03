fn main () {
	let toshiba_amount:f64 = 450_000.0;
	let mac_amount:f64 = 1_500_000.0;
	let hp_amount:f64 = 750_000.0;
	let dell_amount:f64 = 2_850_000.0;
	let acer_amount:f64 = 250_000.0;

 let toshiba_qty:f64 = 2.0;
	let mac_qty:f64 = 1.0;
	let hp_qty:f64 = 3.0;
	let dell_qty:f64 = 3.0;
	let acer_qty:f64 =1.0;

 let sum = toshiba_amount * toshiba_qty + mac_amount * mac_qty + hp_amount * hp_qty + dell_amount * dell_qty + acer_amount * acer_qty;
 let no = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty;
let average = sum / no;

println!("Sum of Amounts is {}", sum);
	println!("Average is {}", average);
}