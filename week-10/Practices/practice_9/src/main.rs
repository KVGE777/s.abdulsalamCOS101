// Define dimensions of a rectangle

struct Rectangle {
    width:u32,
    height:u32
}

// Logic to Calculate area of a rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        // Use the '.' operator to fetch the value of a field via the self keyword
        self.width * self.height
    }
}

fn main() {
    // Instantiate the Structure
    let small = Rectangle {
        width:10,
        height:20
    };

    // Print the Rectangle's Area
    println!("Width is: {} \nHeight is: {} \nArea of Rectangle is: {}", small.width, small.height, small.area());
}