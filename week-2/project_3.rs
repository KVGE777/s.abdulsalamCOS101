fn main() {
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	let a =  p * (1.0 - (r / 100.0)).powf(t);
	println!("Amount after Depreciation is {:.2}", a);
}