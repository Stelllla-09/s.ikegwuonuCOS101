use std::io::Write;

fn main() {
    let title = "Commissioner, Ministry, Zone\n";

    let commissioners = vec!["Algbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye",];
    let ministries = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum",];
    let zones = vec!["South West","North East","South South","South West","South East",];

    let mut file = std::fs::File::create("EFCC_MERGED.csv").expect("Could not create the file");
    file.write_all(title.as_bytes()).expect("Could not write title");

    for i in 0..commissioners.len() {
        let line = format!("{}, {}, {}\n", commissioners[i], ministries[i], zones[i]);
        file.write_all(line.as_bytes()).expect("Could not write line");
    }

    println!("I don save am EFCC_MERGED.csv");
}