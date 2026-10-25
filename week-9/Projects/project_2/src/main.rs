use std::fs::OpenOptions;
use std::io;
use std::io::Write;

struct Student {
    full_name: String,
    matric_no: String,
    department: String,
    level: u32,
}

fn input_text(data: &str) -> String {
    println!("Please enter your {}:", data);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn input_level(data: &str) -> u32 {
    loop {
        let text = input_text(data);
        if let Ok(n) = text.parse::<u32>() {
            if n >= 100 && n <= 500 && n % 100 == 0 {
                return n;
            }
            println!("Please enter proper University level");
        }
        println!("Please type a whole number");
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        let full_name = input_text("Full Name");
        let matric_no = input_text("Matric Number");
        let department = input_text("Department");
        let level = input_level("Level");

        students.push(Student {full_name, matric_no, department, level});

        let rerun = input_text("Add another student? (Yes/No)");
        if rerun.to_lowercase() != "yes" {
            break;
        }
    }

    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("Students.csv")
    .expect("Could not open file");

    for s in students.iter() {
        let row = format!("{},{},{},{}\n", s.full_name, s.matric_no, s.department, s.level);
        file.write_all(row.as_bytes()).expect("Failed to Append");
    }
    println!("Saved all students to student table");
}