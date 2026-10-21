use std::io;

struct Level {
    level: String,
    years_min: u32,
    years_max: u32,
    office_admin: String,
    academic: String,
    lawyer: String,
    teacher: String,
}

fn check(table: &Vec<Level>, role: &str, years: u32) {
    let mut found = false;
    
    for row in table.iter() {
        let role_match = row.office_admin == role
            || row.academic == role
            || row.lawyer == role
            || row.teacher == role;

        if role_match {
            found = true;
            if years >= row.years_min && years <= row.years_max {
                println!("{} with {} years is {}", role, years, row.level);
            } else {
                println!("Invalid: {} needs {}-{} years", role, row.years_min, row.years_max);
            }
        }
    }

    if found == false {
        println!("Role not found: {}", role)
    }
}

fn main() {
    let table: Vec<Level> = vec![
        Level { level: "APS 1-2".to_string(), years_min: 0, years_max: 2,
                office_admin: "Intern".to_string(), academic: "-".to_string(),
                lawyer: "Paralegal".to_string(), teacher: "Placement".to_string()},
        Level { level: "APS 3-5".to_string(), years_min: 3, years_max: 4,
                office_admin: "Administrator".to_string(), academic: "Research Assistant".to_string(),
                lawyer: "Junior Associate".to_string(), teacher: "Classroom Teacher".to_string().to_string() },
        Level { level: "APS 5-8".to_string(), years_min: 5, years_max: 7,
                office_admin: "Senior Administrator".to_string(), academic: "PhD Candidate".to_string(),
                lawyer: "Associate".to_string(), teacher: "Senior Teacher".to_string() },
        Level { level: "EL1 8-10".to_string(), years_min: 8, years_max: 9,
                office_admin: "Office Manager".to_string(), academic: "Post-Doc Researcher".to_string(),
                lawyer: "Senior Associate 1-2".to_string(), teacher: "Leading Teacher".to_string() },
        Level { level: "EL2 10-13".to_string(), years_min: 10, years_max: 13,
                office_admin: "Director".to_string(), academic: "Senior Lecturer".to_string(),
                lawyer: "Senior Associate 3-4".to_string(), teacher: "Deputy Principal".to_string() },
        Level { level: "SES".to_string(), years_min: 13, years_max: 50,
                office_admin: "CEO".to_string(), academic: "Dean".to_string(),
                lawyer: "Partner".to_string(), teacher: "Principal".to_string() },
        ];

        println!("Enter your Staff Level:");
        let mut role_input = String::new();
        io::stdin().read_line(&mut role_input).expect("Failed to read input");
        let role = role_input.trim();
        println!("You typed: {}", role);

        let mut years_input = String::new();
        println!("Enter years of experience:");
        io::stdin().read_line(&mut years_input).expect("Please type of a whole number");
        let years:u32 = years_input.trim().parse().expect("Please type a whole number");

        check(&table, role, years);
}