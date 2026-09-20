use std::io;

fn main() {

    let a = 1_560_000.00;
    let b = 1_480_000.00;
    let c = 1_300_000.00;
    let d = 100_000.00;
    println!("Are you experienced? (Experienced / Not Experienced)");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input, please specify");
    let experience = experience.trim().to_lowercase();

    println!("Specify your age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i8 = age.trim().parse().expect("Failed to Input");

    if experience == "experienced" {
        if age >= 40 {
            println!("Annual Incentive: ₦{}", a);
        } else if age >= 30 && age <= 39 {
            println!("Annual Incentive: ₦{}", b);
        } else if age < 28 {
            println!("Annual Incentive: ₦{}", c);
        } else {
            println!("Age range is not accounted for. Please contact helpline for further details.");
        };
    } else if experience == "not experienced" {
        println!("Annual Incentive: ₦{}", d)
    } else {
        println!("Wrong Input.")
    }
}