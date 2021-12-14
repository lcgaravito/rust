fn main() {
    // Slices -> Heap
    // Arrays -> Stack
    let message = String::from("Hello world, desde el curso de RUST!");
    // let hello = &message[0..5]; // [start..end]
    let hello = &message[..5];
    let resto_mensaje = &message[5..];
    let mensaje_completo = &message[..];
    println!("El mensaje es: {}", message);
    println!("El slice es: {}", hello);
    println!("El resto del mensaje es: {}", resto_mensaje);
    println!("El mensaje completo es: {}", mensaje_completo);
}
