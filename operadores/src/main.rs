fn main() {
    let number_one: i32 = 10;
    let number_two: i32 = 30;
    // + - * / %
    let resultado: i32 = number_one + number_two;
    // > < >= <= == !=
    let _resultado: bool = resultado != 40;
    // && ||
    let resultado: bool = 20 + 10 > 100 || true;

    println!("El resultado es {}", resultado);
}
