use std::io;  // use standard library to track input and output
use rand::Rng; // use library for generating random number
use std::cmp::Ordering; // use for comparing with match

fn main() {

    println! ("-------------------------");
    println! ("-                       -");
    println! ("-    GUESS THE NUMBER   -");
    println! ("-                       -");
    println! ("-------------------------");


    
    let secret_number = rand::thread_rng().gen_range(1,101); // generate random secret_number
    
    let mut guess = String::new();  // create mutable string guess to store user's input
    
    loop {                                  // loop until user guesses the number

    guess.clear();                          // clear the variable or on loop or it will append the new entries and cause panic      
    println!("Please enter the number");

    io::stdin()
        .read_line(&mut guess)              // store the input into reference of mutable guess
        .expect("Failed to read the line"); // if fails print out this message

    let guess: u32 = match guess.trim().parse() {           // convert user's string input into number
        Ok(num) => num,                                     // on number return the number
        Err(_) => {                                         // on other entries ignore the entry
            println! ("This is not a number. Try again."); 
            continue; 
        }                                                
    };
       
        println! ("You guessed {}", guess); // print user's input

        match guess.cmp(&secret_number) {   // compare user input with secret_number reference
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {            // if guess is correct break out of the loop
                println!("You win!");
                println!("  ******** ");
                println!(" *  O  O  *");
                println!("*          *");
                println!("*  \\____/  *");
                println!(" *        *");
                println!("  ********  ");

                break;
            }
        }
    }
}
