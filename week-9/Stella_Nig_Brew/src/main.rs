use std::io::Write;

fn main() {
    let title = "Lager, Stout, Non-Alcoholic\n";
    let data_1 = "33 Export, Legend, Maltina\n";
    let data_2 = "Desperados, Turbo King, Amstel Malta\n";
    let data_3 = "Goldberg, Williams, Malta Gold\n";
    let data_4 = "Gulder, , Fayrouz\n";
    let data_5 = "Heineken, , ,\n";
    let data_6 = "Star, , \n";
     let mut file = std::fs::File::create("STELLA.csv").expect("CREATE THAT FILE GIRLL");

    file.write_all(title.as_bytes()).expect("error");
    file.write_all(data_1.as_bytes()).expect("EHH");
    file.write_all(data_2.as_bytes()).expect("Try again");
    file.write_all(data_3.as_bytes()).expect("It's not working");
    file.write_all(data_4.as_bytes()).expect("AGain");
    file.write_all(data_5.as_bytes()).expect("AGAIN");
    file.write_all(data_6.as_bytes()).expect("That's it, stop");

    println!("You're good to go girll");
}