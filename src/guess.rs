use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guess() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // readline returns Result Ok or Err
        io::stdin()
            .read_line(&mut guess) // referencja
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
