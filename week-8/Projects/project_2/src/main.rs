use std::io;

fn find_top(candidates: &Vec<(String, u32)>) -> (String, u32) {
    let mut top_name = String::new();
    let mut top_years = 0;

    for candidate in candidates.iter() {
        if candidate.1 > top_years {
            top_name = candidate.0.clone();
            top_years = candidate.1;
        }
    }

    (top_name, top_years)
}

fn main() {
    let mut candidates: Vec<(String, u32)> = Vec::new();

    let mut count_input = String::new();
    println!("How many candidates?");
    io::stdin().read_line(&mut count_input).expect("Failed to read line");
    let count: u32 = count_input.trim().parse().expect("Please type a whole number");

    for i in 1..=count {
        let mut name_input = String::new();
        println!("Candidate {} name:", i);
        io::stdin().read_line(&mut name_input).expect("Failed to read line");
        let name = name_input.trim().to_string();

        let mut years_input = String::new();
        println!("Candidate {} years of programming experience:", i);
        io::stdin().read_line(&mut years_input).expect("Failed to read line");
        let years: u32 = years_input.trim().parse().expect("Please type a whole number");

        candidates.push((name, years));
    }

    let (name, years) = find_top(&candidates);
    println!("Highest experience: {} with {} years", name, years);
}