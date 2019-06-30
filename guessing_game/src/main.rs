use std::io;
use std::cmp::Ordering;
use std::io::prelude::*;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        //println!("Secret number: {}", secret);

        print!("Input: ");

        // Just... -_-
        // https://github.com/rust-lang/rust/issues/23818
        io::stdout().flush().expect("Could not flush stdout!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            //.expect("Please type a number!");
            //.unwrap("Please type a number!");
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Bradley Martyn"),
            Ordering::Greater => println!("Dom Mazzetti"),
            Ordering::Equal => { println!("You win!"); break; }
        }
    }
}
