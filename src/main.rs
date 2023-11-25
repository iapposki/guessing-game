use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number between 1 to 100 !");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is : {secret_num}");

    println!("Please input your guess.");

    let mut guess = String::new();
    
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number !");
    
    println!("You guessed: {guess}");

    match guess.cmp(&secret_num) {
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
    }
}
