fn main() {
    let value: i32 = 10; // Inmutable
    println!("The value is: {}", value);
    let value = 20; // Shadowing
    println!("The value is: {}", value);
    let value = false;
    println!("The value is: {}", value);
}
