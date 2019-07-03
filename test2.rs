fn main() {
    // Can be copied to a function
    //let x = 5;
    //let y = add1(x);
    //println!("({}, {})", x, y);

    // Does not contain Copy trait
    let x = String::from("abc");
    let y = add2(&x);
    println!("({}, {})", x, y);
}

fn add1(x: u32) -> u32 {
    x + 1
}

fn add2(x: &String) -> String {
    format!("{}def", x)
}
