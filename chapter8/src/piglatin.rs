use std::io;
use std::io::prelude::*;

fn main() {
    println!("Pig Latin Text Maker\n--------------------");
    print!("Enter text: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    pig_latin(&input);
}

fn pig_latin(input: &str) {
    for word in input.split_whitespace() {
        let first = &word[..1];
        match first {
            "a"|"e"|"i"|"o"|"u" => println!("{}-hay", word),
            _ => println!("{}-{}ay", &word[1..], first)
        }
    }
}
