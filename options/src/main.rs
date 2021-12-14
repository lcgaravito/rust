/*
enum Option<T>{
    Some(T), -> El valor
    None -> La ausencia de algún valor
}
*/
fn obtener_valor(bandera: bool) -> Option<String> {
    if bandera {
        Some(String::from("Soy un mensaje para la tupla Some!"))
    } else {
        None
    }
}
fn main() {
    // Option -> Si existe o no algún valor
    // Result -> Errores -> panic!
    let resultado = obtener_valor(true); // Option
    // match resultado {
    //     Some(valor) => println!("El valor es: {}", valor),
    //     None => println!("No existe ningún valor")
    // }
    // let valor = resultado.unwrap_or("La tupla no almacena ningún valor.".to_string()); // unwrap() -> Retorna el valor del Some // unwrap_or("Mensaje que retorna por defecto")
    let valor = resultado.expect("Se esperaba un String. La tupla no almacena ningún valor.");
    println!("El valor es: {}", valor);
}
