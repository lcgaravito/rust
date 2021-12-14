/*
enum Result<T, E> {
    Ok(T),
    Err(E)
}
*/
#[derive(Debug)]
enum ErrorDivision {
    DivisionPorCero,
    DivisionNegativos
}
fn division(number1: i32, number2: i32) -> Result<i32, ErrorDivision> {
    if number2 == 0 {
        return Err(ErrorDivision::DivisionPorCero);
    }
    if number1 < 0 || number2 < 0 {
        return Err(ErrorDivision::DivisionNegativos);
    }
    Ok(number1 / number2)
}
fn main() {
    // Result
    let resultado = division(10, -2); // Result

    // match resultado {
    //     Ok(valor) => println!("El resultado es: {}", valor),
    //     Err(ErrorDivision::DivisionPorCero) => {
    //         println!("El error es por dividir entre 0.");
    //     },
    //     Err(ErrorDivision::DivisionNegativos) => {
    //         panic!("El error es por dividir con números negativos");
    //     }
    // }

    // unwrap, unwrap_or y expect

    let valor = resultado.expect("No es posible dividir por cero ni usar números negativos.");
    println!("El resultado es: {}", valor);
    
}
