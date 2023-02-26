fn main() {
    let mut s = String::new();  // creating a new, empty String
    s.push('🎶');
    s.push_str("🕺🎤🏀");
    println!("{s}");

    let data = "Sing, dance, rap, 🏀";
    let s = data.to_string();
    println!("{s}");

    let num = 250;
    let s = num.to_string();
    println!("{s}");

    let s = String::from(data);
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3: {s3}");
    // println!("s1: {s1}");  // value borrowed here after move
    println!("s2: {s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let greeting = String::from("你好世界");
    println!("length of greeting: {} byte(s)", greeting.len());  // 12 bytes
    // let hello = &greeting[0..5]; //  panicked at 'byte index 5 is not a char boundary
    let hello = &greeting[0..6];
    println!("{}", hello);

    // iterating over a String
    for ch in greeting.chars() {
        print!("'{}' ", ch);
    }
    println!();
    for b in greeting.bytes() {
        print!("{} ", b);
    }
    println!();

    let hello = String::from("नमस्ते");
    for ch in hello.chars() {
        print!("'{}' ", ch);
    }
    println!();
}
