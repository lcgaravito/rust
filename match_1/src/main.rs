fn main() {
    let number: i32 = 101;
    let message = match number {
        // value => task
        1 => "The number is one.",
        2 => "The number is two.",
        3 => "The number is three.",
        4 | 5 | 6 => "The number is between four and six.",
        7..=100 => {
            let message = "The number is between seven and one hundred";
            message
        },
        _ => "Number" // default
    };
    println!("The message is: {}", message);
}
