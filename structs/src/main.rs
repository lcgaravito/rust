struct User {
    username: String,
    password: String
}
fn create_user(username: String, password: String) -> User {
    User {
        username,
        password
    }
}
fn main() {
    let username = String::from("Usuario1");
    let password = String::from("password1");
    let usuario1 = create_user(username, password);
    let mut usuario2 = create_user(String::from("Usuario2"), String::from("password2"));
    usuario2.password = String::from("Cambio de valor");

    println!("El usuario1 es: {}", usuario1.username);
    println!("El password1 es: {}", usuario1.password);
    println!("El usuario2 es: {}", usuario2.username);
    println!("El password2 es: {}", usuario2.password);
}
