fn main() {
    let mut s = String::from("Hello");
    for i in 1..10 {
        stuff(&mut s, i);
        stuff(&mut s, i);
    }
}

fn stuff(s: &mut String, i: usize) {
    s.push_str(format!("{}", i).as_str());
    println!("{}", s);
}
