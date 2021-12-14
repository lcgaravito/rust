fn main() { // Bloque 1
    // El bloque limita el scope de una variable
    let mensaje = "Hola, soy una variable del bloque main.";
    println!("Bloque 1: {}", mensaje);
    { // Bloque 2
        let mensaje = "Hola, soy una variable del bloque 2.";
        println!("Bloque 2: {}", mensaje);
        { // Bloque 3
            println!("Bloque 3: {}", mensaje);
        }
    }

    println!("---------------------------------------------------------");

    let mut mensaje = String::from("Hola, soy una variable para préstamo.");
    {
        let prestamo = &mensaje; // Prestamos -> Se mueve
        // mensaje = String::from("Mensaje cambiado."); // Error
        println!("Préstamo: {}", prestamo); // Freezing -> mensaje
    }
    println!("Mensaje: {}", mensaje);
}
