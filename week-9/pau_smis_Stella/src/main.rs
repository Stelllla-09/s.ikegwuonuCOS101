use std::io::Write;

fn main() {
    let title = "Student Name, Matric Number, Department, Level\n";

    let students = vec![
        "Oluchi Mordi, ACC10211111, Accounting, 300\n",
        "Adams Aliyu, ECO10110101, Economics, 100\n",
        "Shania Bolade, CSC10328828, Computer, 200\n",
        "Adekunle Gold, EEE11020202, Electrical, 200\n",
        "Blanca Edemoh, MEE10202001, Mechanical, 100\n",
    ];

    print!("{}", title);
    for student in &students {
        print!("{}", student);
    }
    let mut file = std::fs::File::create("STUDENTS.csv").expect("Could not create the file");
    file.write_all(title.as_bytes()).expect("Could not write title");
    for student in &students {
        file.write_all(student.as_bytes()).expect("Could not write student");
    }

    println!("\nStudent details saved to STUDENTS.csv");
}