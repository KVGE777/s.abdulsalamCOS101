use std::io;

fn main() {

    println!("Enter Coeffiecient a");
    let mut coefa = String::new();
    io::stdin().read_line(&mut coefa).expect("Failed to read input");
    let a:f64 = coefa.trim().parse().expect("Failed to Input");

    println!("Enter Coeffiecient b");
    let mut coefb = String::new();
    io::stdin().read_line(&mut coefb).expect("Failed to read input");
    let b:f64 = coefb.trim().parse().expect("Failed to Input");

    println!("Enter Constant c");
    let mut constc = String::new();
    io::stdin().read_line(&mut constc).expect("Failed to read input");
    let c:f64 = constc.trim().parse().expect("Failed to Input");

    let disc = (b * b) - (4.0 * a * c);

    if disc > 0.0 {
        let x = (-b + disc.sqrt()) / (2.0 * a);
        let y = (-b - disc.sqrt()) / (2.0 * a);

        println!("Root 1 is: {}", x);
        println!("Root 2 is: {}", y);
    } else if disc == 0.0 {
        let z = -b / (2.0 * a);

        println!("Equation only has one root: {}", z);
    } else {
        println!("Equation has no real roots.")
    }
}