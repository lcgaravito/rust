fn main() {
    let tupla = (20, true, 2.5);
    println!("The value of the tupla is {:?}", tupla);
    let mut tupla: (i32, bool, f64, i32) = (10, false, 5.9, 19);
    println!("The value of the tupla is {:?}", tupla);
    let first_element = tupla.0;
    let last_element = tupla.3;
    println!("The fisrt element is: {}", first_element);
    println!("The fisrt element is: {}", last_element);
    tupla.1 = true;
    println!("The value of the tupla is {:?}", tupla);
}
