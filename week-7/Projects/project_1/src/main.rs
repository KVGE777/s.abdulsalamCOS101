use std::io;

fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse().expect("Please enter a number")
}

fn trapezium_area() -> f64 {
    let height = read_number("Enter height:");
    let base1 = read_number("Enter base 1:");
    let base2 = read_number("Enter base 2:");
    height / 2.0 * (base1 + base2)
}

fn rhombus_area() -> f64 {
    let d1 = read_number("Enter diagonal 1:");
    let d2 = read_number("Enter diagonal 2:");
    0.5 * d1 * d2
}

fn parallelogram_area() -> f64 {
    let base = read_number("Enter base:");
    let altitude = read_number("Enter altitude:");
    base * altitude
}

fn cube_surface_area() -> f64 {
    let side = read_number("Enter side:");
    6.0 * side * side
}

fn cylinder_volume() -> f64 {
    let radius = read_number("Enter radius:");
    let height = read_number("Enter height:");
    3.14159 * radius * radius * height
}

fn main() {
    println!("Shape Calculator");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");

    let choice = read_number("Choose a shape (1-5):") as i32;

    if choice == 1 {
        println!("Area = {}", trapezium_area());
    } else if choice == 2 {
        println!("Area = {}", rhombus_area());
    } else if choice == 3 {
        println!("Area = {}", parallelogram_area());
    } else if choice == 4 {
        println!("Surface Area = {}", cube_surface_area());
    } else if choice == 5 {
        println!("Volume = {}", cylinder_volume());
    } else {
        println!("Invalid choice");
    }
}