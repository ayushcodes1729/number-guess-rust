use std::{cmp::Ordering, io};
use rand::Rng;
fn main(){
    println!("Number Guessing Game!!");
    let secret_number = rand::rng().random_range(1..=100);
    println!("Secret number is {secret_number}");
    loop {
        print!("Guess a number: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to  read line");
    
        let guess: i32= guess.trim().parse().expect("Please type a number");
        // In the above line rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess.
        println!("You guessed {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You guessed right");
                break;
            }
        }
    }
}