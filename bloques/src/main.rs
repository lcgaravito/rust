fn main() {
    let message = "Hi. I am the variable message in main";
    {
        let message = "Hi. I am the variable message in the nested block";
        println!("Hello. From a second block.");
        println!("{}", message);
        let _message_two = "Message in a nested block.";
    }

    println!("{}", message);

    // ERROR!!!
    // println!("{}", _message_two);

    let result = {
        println!("Hi. We are in a nested block");
        let variable: i32 = 200;
        println!("{}", variable);
        variable// Return the value. Do not put ";"
    };
    println!("The value of result is {}", result);

    // --------------------------------------------
    let score: i8 = 10;

    let message = if score >= 6 {
        String::from("Congratulations! 🍻")
    } else {
        String::from("You need to study more. 😥")
    };

    println!("{}", message);
}
