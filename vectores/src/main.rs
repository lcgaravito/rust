fn main() {
    // let mut vector = vec![1, 2, 3];
    let mut vector: Vec<i32> = Vec::new(); // String::new()
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);
    vector.insert(0, -1);
    vector.insert(1, 0);
    println!("The vector value is {:?}", vector);
    vector.remove(vector.len()-1);
    println!("The vector value is {:?}", vector);
    let first_element = vector[0];
    let last_element = vector[vector.len()-1];
    println!("The first element is  {}", first_element);
    println!("The last element is   {}", last_element);
    vector[0] = -10;
    println!("The vector value is {:?}", vector);
    let last_element = vector.pop().unwrap(); // Option
    println!("The last element with pop() is {}", last_element);
    println!("The vector value is {:?}", vector);
}