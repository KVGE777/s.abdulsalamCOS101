use std::io::Write;

fn main() {

    let announce = "Week 9 - Rust File Input and Output\n";
    let dept = "Department of Software Engineering";

    let mut file = std::fs::File::create("data.txt").expect("Create Failed.");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("Write Failed");
    file.write_all(announce.as_bytes()).expect("Write Failed");
    file.write_all(dept.as_bytes()).expect("Write Failed");
    println!("\nData Written to File.");
}