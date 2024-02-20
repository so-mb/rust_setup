// The functions in this file are not used in the project

fn average_price() -> f64 {
    println!("average_price");
    let prices = [1, 2, 3, 4, 5];
    let mut total = 0;
    for price in prices {
        total += price;
    }
    total as f64 / prices.len() as f64
}

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}


// Create a function to add two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Create a function to subtract two numbers
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// Create a function to multiply two numbers
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

