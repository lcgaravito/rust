fn main() {
    for number in 1..31 {
        match (number % 3, number % 5) {
            (0, 0) => println!("Fizz Buzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", number)
        }
    }
}
