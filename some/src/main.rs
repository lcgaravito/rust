fn main() {
    let message = Some("Hello world!");
    match message {
        Some("Hello world!") => println!("The message is: Hello world!"),
        Some("Bye!") => println!("The message is: Bye!"),
        Some(_) => println!("It is another message."),
        None => println!("There is no value")
    }
}
