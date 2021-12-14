fn main() {
    // One line comment.
    /*
        Comment with line break.
    */
    const VALOR: i32 = 10; // Constant declaration
    let mut number_one = 100; // mut: indicates that the variable is mutable
    let number_two: i32 = 200; // i32: data type
    number_one = 120;
    let result = number_one + number_two + VALOR;
    println!("The result is({} + {} + {}):{}.", number_one, number_two, VALOR, result);
}
