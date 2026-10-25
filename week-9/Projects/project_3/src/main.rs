use std::fs::OpenOptions;
use std::io;
use std::io::Write;

fn input_details(data: &str) -> String {
    println!("Enter {}", data);
    let mut details = String::new();
    io::stdin().read_line(&mut details).expect("Failed to read input");
    details.trim().to_string()
}

fn main() {
    let mut commissioner = Vec::new();
    let mut ministry = Vec::new();
    let mut zone = Vec::new();

    loop {
        commissioner.push(input_details("Full Name:"));
        ministry.push(input_details("Ministry:"));
        zone.push(input_details("Geo-Political Zone:"));

        let rerun = input_details("another student? (Yes/No):");
        if rerun.to_lowercase() != "yes" {
            break;
        }
    }

    let mut file = OpenOptions::new().create(true).append(true).open("Convicts.csv").expect("Could not open file");

    for no in 0..commissioner.len() {
        let row = format!("{},{},{}\n", commissioner[no], ministry[no], zone[no]);
        file.write_all(row.as_bytes()).expect("Failed to add row");
    }

    println!("Saved Conivicts to Table");
}