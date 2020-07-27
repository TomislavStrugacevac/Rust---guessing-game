use std::io;

fn main() {

    println!("Please enter the number");

    let mut guess = String::new();  // create mutable string guess to store user's input

    io::stdin()
        .read_line(&mut guess)              // store the input into reference of mutable guess
        .expect("Failed to read the line"); // if fails print out this message

    print! ("You guessed {}", guess); // print user's input
}
