fn main() {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let n = 100;
    for _i in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    println!("Fibonacci index {} = {}", n, b);
}

/* Recursive function
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}
*/
