fn main() {
    // 123 -> 3
    // 12345 -> 5
    // 123456789 -> 9
    let mut number: i64 = 123456342378;
    let mut counter = 0;
    while number > 0 {
        number /= 10;
        counter += 1;
    }
    println!("The number of digits is {}", counter);
}
