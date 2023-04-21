#[allow(unused)]
fn main() {
    // creating new `Vector`s
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    // updating a `Vector`
    v1.push(5);
    v2.push(4);

    let mut v3 = Vec::new();
    v3.push('🎈');

    let third: &i32 = &v2[2];
    println!("The third element of v2 is {third}");
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element of v2 is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v1[100];  // panic
    let does_not_exist = v1.get(100);
    match does_not_exist {
        Some(n) => println!("The 101st element of v2 is {n}"),
        None => println!("There is no 101st element."),
    }

    let first = &v3[0];  // immutable borrow occurs here
    // v3.push('🪂');          // mutable borrow occurs here
    println!("The first element of v3 is {first}"); // immutable borrow later used here

    for i in &mut v2 {
        *i += 50;
    }
    let last = v2.pop();
    for i in &v2 {
        print!("{i} ");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.24),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
