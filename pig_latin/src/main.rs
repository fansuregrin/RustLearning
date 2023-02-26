fn main() {
    let s1 = "Hello world";
    println!("{}", pig_latin(s1));
    let s2 = "apple tree";
    println!("{}", pig_latin(s2));
}

fn pig_latin(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let chars = word.chars().collect::<Vec<char>>();
            let split_index = if chars.len() > 1
                && (chars[0] == 'x' && chars[1] == 'r' || chars[0] == 'y' && chars[1] == 't') {
                    0
                } else {
                    let mut index = 0;
                    for (i, &c) in chars.iter().enumerate() {
                        if is_vowel(c) || (i> 0 && c == 'y') {
                            index = i;
                            break;
                        }
                    }
                    if index > 0 && chars[index-1] == 'q' && chars[index] == 'u' {
                        index += 1;
                    }
                    index
                };
            let (left, right) = chars.split_at(split_index);
            let left = left.iter().collect::<String>();
            let right = right.iter().collect::<String>();
            right + &left + "ay"
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
