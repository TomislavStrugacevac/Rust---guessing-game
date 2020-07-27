use std::io;  // use standard library to track input and output
use rand::Rng; // use library for generating random number
use std::cmp::Ordering; // use for comparing with match

fn main() {

    println! ("-------------------------");
    println! ("-                       -");
    println! ("-    GUESS THE NUMBER   -");
    println! ("-                       -");
    println! ("-------------------------");


    let mut guess = String::new();  // create mutable string guess to store user's input
    let secret_number = rand::thread_rng().gen_range(1,101); // generate random secret_number
    
    println! ("Secret number is {}", secret_number);

    println!("Please enter the number");

    io::stdin()
        .read_line(&mut guess)              // store the input into reference of mutable guess
        .expect("Failed to read the line"); // if fails print out this message

    let guess: u32 = guess.trim().parse().expect("Please type a number!");  //convert user's string input into number

    println! ("You guessed {}", guess); // print user's input

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
