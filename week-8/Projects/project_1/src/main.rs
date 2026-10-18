use std::io;

struct Level {
    level: &'static str,
    years_min: u32,
    years_max: u32,
    office_admin: &'static str,
    academic: &'static str,
    lawyer: &'static str,
    teacher: &'static str,
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
        Level { level: "APS 1-2", years_min: 0, years_max: 2,
                office_admin: "Intern", academic: "-",
                lawyer: "Paralegal", teacher: "Placement" },
        Level { level: "APS 3-5", years_min: 3, years_max: 4,
                office_admin: "Administrator", academic: "Research Assistant",
                lawyer: "Junior Associate", teacher: "Classroom Teacher" },
        Level { level: "APS 5-8", years_min: 5, years_max: 7,
                office_admin: "Senior Administrator", academic: "PhD Candidate",
                lawyer: "Associate", teacher: "Senior Teacher" },
        Level { level: "EL1 8-10", years_min: 8, years_max: 9,
                office_admin: "Office Manager", academic: "Post-Doc Researcher",
                lawyer: "Senior Associate 1-2", teacher: "Leading Teacher" },
        Level { level: "EL2 10-13", years_min: 10, years_max: 13,
                office_admin: "Director", academic: "Senior Lecturer",
                lawyer: "Senior Associate 3-4", teacher: "Deputy Principal" },
        Level { level: "SES", years_min: 13, years_max: 50,
                office_admin: "CEO", academic: "Dean",
                lawyer: "Partner", teacher: "Principal" },
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