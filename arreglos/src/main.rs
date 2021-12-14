fn main() {
               // 0  1  2  3  4
    let numbers = [1, 2, 3, 4, 5]; // size -> 5
    println!("The value of the array is {:?}", numbers);
    let mut numbers: [i32; 5] = [6, 7, 8, 9, 10];
    println!("The value of the array is {:?}", numbers);
    let valores = [1; 10];
    println!("The value of the array valores is {:?}", valores);
    let first_element = numbers[0];
    let last_element = numbers[numbers.len()-1];
    println!("The first element is: {}", first_element);
    println!("Tha last element is:  {}", last_element);
    numbers[2] = 545;
    println!("The value of the array is {:?}", numbers);
}