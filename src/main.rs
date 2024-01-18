use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guess the Number game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    let mut min_num: u32 = 1;
    let mut max_num: u32 = 100;

    loop {
        println!(
            "Guess the secret number between {} & {}. 🤔",
            min_num, max_num
        );

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Does not compute");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please use the number keys.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("\nToo low, guess higher! ⬆️");
                min_num = guess;
            }
            Ordering::Greater => {
                println!("\nToo high, guess lower! ⬇️");
                max_num = guess;
            }
            Ordering::Equal => {
                println!("\nYou win! 🎆🔥🪄🔥🎇\n");
                break;
            }
        }
    }
}
