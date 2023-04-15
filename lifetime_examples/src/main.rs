use std::fmt::Display;

fn main() {
    let string1 = String::from("I am longer.");
    {
        let string2 = String::from("abc");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    }

    // this code does not compile
    // let string1 = String::from("I am longer.");
    // let result;
    // {
    //     let string2 = String::from("abc");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is: {}", result);

    let novel = String::from("Harry Potter and the Sorcerer's Stone. The boy who lived.");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime
    let _s: &'static str = "I have a static lifetime";
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// this code will compile
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// this code won't compile
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime annotations in method definitions
#[allow(unused)]
impl<'a> ImportantExcerpt<'a> {
    // we’re not required to annotate the lifetime of the reference to self
    // because of the first elision rule.
    fn level(&self) -> i32 {
        3
    }

    // There are two input lifetimes, so Rust applies the first lifetime elision rule
    // and gives both &self and announcement their own lifetimes. Then, because one
    // of the parameters is &self, the return type gets the lifetime of &self, and
    // all lifetimes have been accounted for. 
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// use the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function
#[allow(unused)]
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    print!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}