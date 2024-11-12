use std::io::stdin; // io library come from std library

fn guess_num() { // fn syntax declares a new function
    println!("Guess the number!");
    println!("Input your guess.");

    let mut guess = String::new(); // create a variable to store the user input
    // Here is another example:
    // let apples = 5; // in default, variables are immutable.
    // let mut bananas = 5 // variable named bananas is mutable.
    // String is a string type provided by the standard library that growable UTF-8 encoded
    stdin().read_line(&mut guess).expect("Failed to readline");
    println!("You guessed: {}", guess)
}

fn main() {
   guess_num();
}

