use rand::Rng;
use std::{cmp::Ordering, io, mem::ManuallyDrop};
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut number_of_guesses = 0;
    println!("Please input your guess");

    loop {
        //println!("the secert number is {secret_number}");
        let mut guess = String::new();
        let _ = number_of_guesses += 1;
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // .expect("Failed to read line");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("please use enter a number not {guess}");

                continue;
            }
        };
        println!("You guessed!: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("you guessed to low"),
            Ordering::Equal => {
                println!("you Win");
                println!("you guessed {number_of_guesses} times");
                break;
            }
            Ordering::Greater => println!("ypu guessed to high"),
        }
    }
}
