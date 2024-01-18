use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guess the Number game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret_number is: {secret_number}");

    let min_num = "1";
    let max_num = "100";
    println!("Guess the secret number between {min_num} & {max_num}.");

    loop {
        let low_guess: u32 = 1;
        let high_guess: u32 = 100;

        // input starts as a string
        let mut guess = String::new();

        // access user input
        io::stdin()
            // specify which string stores input, need mut to change the content
            .read_line(&mut guess)
            // handle 'Result' enum returned by `read_line`: `Ok` or `Err`
            // 'Ok' holds successfully generated variable
            // 'Err' contains info of how/why operation failed
            .expect("Does not compute");

        println!("You guessed: {guess}");

        // use match & the returned Result from parse
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please use the number keys.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                let low_guess = guess;
                println!("Guess between {low_guess} & {high_guess}.");
            }
            Ordering::Greater => {
                let high_guess = guess;
                println!("Guess between {low_guess} & {high_guess}.");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
