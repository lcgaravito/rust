#[derive(Debug)]
struct User {
    username: String,
    password: String
}
impl User {
    // without mut
    fn saludar(& self) {
        println!("Hola, soy el usuario: {}", self.username);
    }
    fn set_password(&mut self, new_password: String) {
        self.password = new_password;
    }
}
fn main() {
    let mut usuario1 = User {
        username: String::from("usuario1"),
        password: String::from("Password")
    };
    usuario1.saludar();
    usuario1.set_password("New Password".to_string());
    println!("El usuario es: {:?}", usuario1);
}
