// String is a wrapper over Vec<u8>

fn main() {
    let mut s1 = String::new();
    let mut s2 = String::from("Hello, ");
    let mut s3 = "world!".to_string();     // Implements the Display trait
    s2.push_str(s3.as_str());              // Append a string slice (str)
    println!("{}", s2);
    s1.push('∆');                          // Append a character
    println!("{}", s1);
    let s4 = s1 + &s2 + &s3;               // Plus concatenation
    println!("{}", s4);
    let (a, b, c) = ("tic", "tac", "toe");
    let s5 = format!("{}-{}-{}", a, b, c); // Using format! macro
    println!("{} of length {}", s5, s5.len());

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    for i in hello.chars() {
        println!("{}", i);
    }
    for i in "नमस्ते".bytes() {
        println!("{}", i);
    }
}
