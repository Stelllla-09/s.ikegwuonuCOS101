use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Girl, I can't open this file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nI'm appending this document boo".as_bytes()).expect("Failed mehn, try again");
    println!("file has been appended boo");
}
