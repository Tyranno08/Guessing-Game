extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess number GAME");

    let mut x=0;

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Enter any number between 1 to 100");

        let mut guess = String::new();
    
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read user input");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Your guessed number is {}", guess);
        x +=1; 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small number: "),
            Ordering::Greater =>println!("Too Big numbber: "),
            Ordering::Equal => {
                println!("Great, You Won after {} attempt",x);
                break;
            }
        }
    }

}