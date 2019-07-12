// HashMap<K, V> stores a mapping of keys of type K to values of type V

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("∑", 1337);
    scores.insert("†", 69);

    let teams = vec!["Team 1", "Team 2", "Team 3", "Team 4"];
    let pts = [69; 4];
    let mut scores: HashMap<_, _> = teams.iter().zip(pts.iter()).collect();
    println!("Team 2: {:?}", scores.get(&"Team 2"));
    println!("{:?}", scores);
    scores.insert(&"Team 3", &1337);
    scores.entry(&"Team 5").or_insert(&420); // Only update if doesn't exist
    for (k, v) in &scores {
        println!("{} = {}", k, v);
    }

    let text = "I'm slim shady yes I'm the real shady";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
