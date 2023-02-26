fn main() {
    another_function(5, 'h');

    // let y = (let x = 6) // compile error!

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    println!("The value of y is: {}", five());

    println!("The value of y is: {}", plus_one(y));
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 'x + 1;' complie error!
}