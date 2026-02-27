use std::io;
use rand::RngExt;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::rng().random_range(1..=10);
    println!(" --- Guess the Number! --- ");

    loop{
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Please, check your input and try again.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please, check your input and try again.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("Game over! The secret number was: {secret_number}");

}
