use std::io;
fn main() {
    println!("Enter the username:");
    let mut username = String::new(); // String -> ""
    // Result -> Exito o Error
    io::stdin().read_line(&mut username); // Referencia
    let username = username.trim();
    println!("Enter the age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age);
    let age = age.trim();
    // Result
    let age: i32 = age.parse().unwrap();
    println!("The name is {} and the age is {}", username, age);
}