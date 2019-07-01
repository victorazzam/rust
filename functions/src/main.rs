fn main() {
    another(6, 9);

    let x = 5;     // Statement, the 5 is the expression part
    let y = {      // Statement with a multi-line expression
        let x = 3;
        x + 1      // No semi-colon, implicit return, expression
    };
    println!("\nx = {}\ny = {}", x, y);

    let mut z = five();
    println!("\nz = {}", z);
    z = plus_one(z);
    println!("z = {}", z);
}

fn another(x: i32, y: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
