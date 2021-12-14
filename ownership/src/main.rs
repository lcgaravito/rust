struct Rectangulo {
    ancho: u32,
    alto: u32
}
fn area(rectangulo: &Rectangulo) -> u32 {
    rectangulo.ancho * rectangulo.alto
}
fn main() {
    // Ownership
    /*
    * Cada valor en Rust tiene su propio ownership.
    * Solo puede existir un ownership a la vez.
    * Si un ownership sale del alcance, el valor se descartará.
    */
    let rectangulo = Rectangulo { ancho: 40, alto: 50};

    // Argumentos pasados mediante préstamos -> default
    // Argumentos pasados por referencias -> &
    let area_rectangulo = area(&rectangulo);

    // Los objetos que se almacenan en el HEAP
    let new_rectangulo = rectangulo; // Movimiento de ownership

    let x = 10; // Stack
    let y = x;

    println!("{}", x);
    println!("{}", y);

    println!("El area del rectangulo es: {}", area_rectangulo);
    println!("El ancho del rectangulo es {}.\nEl alto del rectangulo es: {}.", new_rectangulo.ancho, new_rectangulo.alto);
}
