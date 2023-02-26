fn main() {
    let s = String::from("hello!😉");  // s comes into scope

    takes_ownership(s);  // s's value moves into the function...
                                     // ... and so is no longer valid here
    // println!("{}", s); // compile error

    let n = 5;  // n comes into scope
    makes_copy(n);  // n is copyed
                                 // it's ok to use n afterward
    println!("n is {n}");

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1: {}", s1);
    // println!("{}", s2);  // compile error
    println!("s3: {}", s3);

    println!("The length of s3 is {}", calculate_length(&s3));
    println!("s3: {}", s3);  // you can still use s3 here

    let mut s4 = String::from("hi");
    change(&mut s4);
    println!("s4: {}", s4);

    // Mutable references have one big restriction: 
    // if you have a mutable reference to a value, 
    // you can have no other references to that value.
    //
    // let r1 = &mut s4;
    // let r2 = &mut s4;
    // println!("{}, {}", r1, r2);

    {
        let r1 = &mut s4;
        println!("r1: {}", r1);
    }
    let r2 = &mut s4;
    println!("r2: {}", r2);

    // compile error!
    // let r1 = &s4;
    // let r2 = &s4;
    // let r3 = &mut s4;
    // println!("r1: {}, r2: {}", r1, r2);

    let r1 = &s4;
    let r2 = &s4;
    println!("r1: {}, r2: {}", r1, r2);
    let r3 = &mut s4;
    println!("r3: {}", r3);

    let s5 = no_dangle();
    print!("s5: {s5}");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours");  // some_string comes into scope
    some_string  // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {  // s is a reference of a String
   s.len() 
}  // Here, s goes out of scope. But because it does not have ownership of what
   // it refers to, it is not dropped. 

// compile error!
//
// fn change(s: &String) {
//     s.push_str("...");
// }

fn change(s: &mut String) {
    s.push_str("...");
}

// missing lifetime specifier
// this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
