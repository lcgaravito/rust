fn main() {
    // str -> Stack
    // String -> Heap

    // str
    let variable_str = "Hola, soy un tipo str";
    let mut variable_string = String::from("Hola, soy un tipo String"); // String::new() -> ""

    variable_string.push('.');
    variable_string.push(' ');
    variable_string.push('H');
    variable_string.push('E');
    variable_string.push('L');
    variable_string.push('L');
    variable_string.push('O');
    variable_string.push(' ');
    variable_string.push_str("WORLD.");

    println!("El str es: {}", variable_str);
    println!("El String es: {}", variable_string);

    let new_string = "Hola, soy una nueva cadena".to_string(); // str

    let diferentes = new_string != "Hola, soy una nueva cadena".to_string();

    println!("El nuevo String es: {}", new_string);
    println!("Los Strings son diferentes?: {}", diferentes);
}