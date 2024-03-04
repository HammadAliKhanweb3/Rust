// Hash Map in Rust
// A hash map is a collection that associates a key with a value

use std::collections::HashMap;

fn main() {
    // hashmap
    let mut scores = HashMap::new();

    // inserting key value pairs in hashmap
    scores.insert(String::from("Ali"), 100);
    scores.insert(String::from("Rana"), 10);

    // retreiving element from hashmap
    let ali_tokens = scores.get(&String::from("Ali"));
    println!("{:?}", ali_tokens);

    let mut names = HashMap::new();

    names.insert(15, String::from("Rana"));
    names.insert(20, String::from("osama"));
    println!("{:?}", names);

    names.remove(&15);
    println!("{:?}", names);

    // printing key value pairs of hashmap through loop
    for (key, value) in &scores {
        println!("{} {}", key, value);
    }
}
