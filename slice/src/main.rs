fn main() {
    let s1 = String::from("Hello world!");
    let word = first_word(&s1);
    println!("first word of s1: {}", word);

    let s2 = "Harry Potter is dead!";
    let word = first_word(s2);
    println!("first word of s2: {}", word);

    let s3 = &s1[2..];
    let word = first_word(s3);
    println!("first word of s3: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (ix, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..ix]
        }
    }
    &s[..]
}
