struct SalesRecord {
	item: String,
	quantity: i32,
	amount: f64,
}

fn main() {
	let sales = vec![
		SalesRecord { item: String::from("Toshiba"), quantity: 2, amount: 450_000.00 },
		SalesRecord { item: String::from("Mac"), quantity: 1, amount: 1_500_000.00 },
		SalesRecord { item: String::from("HP"), quantity: 3, amount: 750_000.00 },
		SalesRecord { item: String::from("Dell"), quantity: 3, amount: 2_850_000.00 },
		SalesRecord { item: String::from("Acer"), quantity: 1, amount: 250_000.00 },
	];

	let mut total_amount = 0.0;

	for record in &sales {
		println!("{} - Qty: {} - Amount: {:.2}", record.item, record.quantity, record.amount);
		total_amount += record.amount;
	}

	let average_amount = total_amount / sales.len() as f64;

	println!("Total: {:.2}", total_amount);
	println!("Average: {:.2}", average_amount);
}