fn main() {
    // Mutability
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    // Shadowing variables
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("y = {}", y);

    // Shadowing with different types
    let spaces = "    ";
    println!("\nSpaces: '{}'", spaces);
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("\n({}, {}, {})", tup.0, tup.1, tup.2);

    // Array
    let a = [1, 2, 3];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let mut c = [3; 5];
    println!("\na: {:?}\nb: {:?}\nc: {:?}", a, b, c);
    c[2] = 99;
    println!("c: {:?}", c);
}
