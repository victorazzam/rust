fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let num = if number > 0 {
        1
    } else {
        2
    };
    println!("num = {}", num);
}
