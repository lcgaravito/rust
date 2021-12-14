fn main() {
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];

    for numero in numeros.iter() {
        println!("El número es: {}", numero);
    }
    // Fizz Buzz
    for number in 1..101 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("Fizz Buzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }
}
