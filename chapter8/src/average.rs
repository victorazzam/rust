// Help from https://codereview.stackexchange.com/q/173338

use std::collections::HashMap;

fn main() {
    let mut v = vec![12, 47, 93, -66, 115, 142, -5, 189, 93, 0, -21];
    println!("List: {:?}", v);
    println!("Mean: {:?}", mean(&v));
    println!("Median: {:?}", median(&mut v));
    println!("Mode: {:?}", mode(&v));
    println!("List (sorted): {:?}", v);
}

fn mean(vec: &[i32]) -> f32 {
    vec.iter().sum::<i32>() as f32 / vec.len() as f32
}

fn median(vec: &mut [i32]) -> i32 {
    vec.sort_unstable();
    let size = vec.len();
    vec[size / 2]
}

fn mode(vec: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for &i in vec {
        *map.entry(i).or_insert(0) += 1;
    }
    match map.into_iter().max_by_key(|&(_, count)| count) {
        Some(value) => value.0,
        None => 0
    }
}
