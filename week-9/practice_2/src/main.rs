use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("welcome_message.txt").expect("You can't open a file you haven't created girlll");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File not found");
    print!("{}", contents);
}
