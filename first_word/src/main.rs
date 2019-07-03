use std::io;
use std::io::prelude::*;

fn main() {
    let mut s = String::new();

    print!("Enter your sentence: ");
    io::stdout().flush().expect("Could not flush stdout!");
    io::stdin().read_line(&mut s).expect("Failed to read line!");
    s = s.trim().to_string();

    println!("First word: {}", fw(&s));
    println!("End offset: {}", first_word(&s));

    println!("Slice first 5: {}", &s[..5]);
    println!("Slice [6..9]: {}", &s[6..9]);
    println!("Slice last 5: {}", &s[s.len()-5..]);
    println!("Slice whole: {}", &s[0..s.len()]);

    println!("Slice function: {}", slice(&s[..])); // Pass slice instead
}

fn fw(line: &String) -> String {
    let mut word = String::new();
    for i in line.chars() {
        if i == ' ' {
            break
        }
        word.push_str(&i.to_string());
    }
    word
}

fn first_word(s: &String) -> usize {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.trim().len()
}

// Accepts &str (slice) instead of &String for malleability
fn slice(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
