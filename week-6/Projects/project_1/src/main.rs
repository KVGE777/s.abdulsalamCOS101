use std::io;

fn main () {
    let menu = [
    ("Poundo Yam / Edinkaiko Soup", 3200.0),
    ("Fried Rice & Chicken", 3000.0),
    ("Amala & Ewedu Soup", 2500.0),
    ("Eba & Egusi Soup", 2000.0),
    ("White Rice & Stew", 2500.0)
    ];

    println!("--- MENU ---");
    for (i, (name, price)) in menu.iter().enumerate() {
        println!("{}. {} - {:.2}", i + 1, name, price);
    }

    println!("Enter item number: ");
    let mut buy = String::new();
    io::stdin().read_line(&mut buy).expect("Failed to read input");
    let item: usize = buy.trim().parse().expect("Please enter a number");

    if item < 1 || item > menu.len() {
        println!("Invalid choice.");
        return;
    }

    println!("Enter quantity:");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Failed to read input");
    let quantity: u32 = qty.trim().parse().expect("Please enter a number");

    let price = menu[item - 1].1;
    let total = price * quantity as f64;

    let threshold = 10000.0;
    let mut final_total = total;

    if total > threshold {
        final_total = total * 0.90
    };

    println!("Item: {}", menu[item - 1].0);
    println!("Quantity: {}", quantity);
    println!("Total: {:.2}", total);

    if total > threshold {
        println!("10% discount applied!");
    }

    println!("Final Amount: {:.2}", final_total);
}