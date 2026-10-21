use std::io::Write;

fn main() {

    let drinks = "Lager, Stout, Non-Alcoholic\n33 Export, Legend, Maltina\nDesperados, Turbo King, Amstel Malta\nGoldberg, Williams, Malta Gold\nGulder,,Fayrous\nHeineken,,\nStar,,\n";

    let mut file = std::fs::File::create("drinks.csv").expect("Create Failed.");
    file.write_all(drinks.as_bytes()).expect("Write failed")
    println!("File Created.")
}