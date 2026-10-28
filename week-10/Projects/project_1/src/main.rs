#[allow(dead_code)]
struct Laptops {
    brand:String,
    quantity:u32,
    price:u32
}

impl Laptops {
    fn total_price(&self) -> u32 {
        let cost = self.price * 3;
        println!("3 {} laptops cost ₦{}", self.brand, cost);
        cost
    }
}

fn main() {
    let hp = Laptops {
        brand:String::from("HP"),
        quantity:10,
        price:650_000
    };

    let ibm = Laptops {
        brand:String::from("IBM"),
        quantity:6,
        price:755_000
    };

    let tosh = Laptops {
        brand:String::from("Toshiba"),
        quantity:10,
        price:550_000
    };

    let dell = Laptops {
        brand:String::from("Dell"),
        quantity:4,
        price:850_000
    };

    hp.total_price();
    ibm.total_price();
    tosh.total_price();
    dell.total_price();

}