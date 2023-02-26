use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score of team Bule: {}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // if the key exists, it's value will be overwritten
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // adding a key and value only if a key isn't persent
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(40);
    println!("{:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut words_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = words_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", words_map);
}
