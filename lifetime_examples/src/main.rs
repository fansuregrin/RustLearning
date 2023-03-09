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
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
