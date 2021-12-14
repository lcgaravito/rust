/*
enum Option<T>{
    Some(T), -> El valor
    None -> La ausencia de algún valor
}
*/
#[derive(Debug)]
struct User {
    username: String,
    password: String,
    email: String,
    edad: Option<u32>
}
fn main() {
    let usuario = User {
        username: String::from("Luis"),
        password: String::from("password"),
        email: String::from("luis@email.com"),
        edad: None // Some(15)
    };
    println!("El usuario es: {:?}", usuario);
    // let edad = usuario.edad.unwrap(); // Si no estoy seguro -> usar match
    match usuario.edad {
        Some(edad) => println!("Su edad es: {}", edad),
        None => {}
    }
}