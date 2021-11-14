use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game. 
    I'm thinking of a number, your job is to guess it using as few guesses as possible");
    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess!");
    
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    // Följande rad betyder essentiellt att om det inte fungerar; 
    // Prefixa felmeddelande i "Err" med argumentet (strängen) som är given. 
    // Känns som att formuleringen blir lite galen va?
    .expect("Failed to read line!"); 

    println!("You guessed: {}", guess);
}