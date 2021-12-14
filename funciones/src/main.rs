fn saludar_usuarios() {
    println!("Hola, desde una función.");
}

fn suma(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn factorial(number: u32) -> u32 {
    if number == 1 {
        number
    } else {
        factorial(number -1) * number
    }
}

fn main() {
    saludar_usuarios();
    let result = suma(10, 20);
    let number = 10;
    let fact = factorial(number);
    println!("El resultado es: {}", result);
    println!("El factorial de {} es: {}", number, fact);
}
