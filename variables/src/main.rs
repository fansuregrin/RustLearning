fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 9;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    let a = 0.26;
    let b: f32 = 0.27;
    println!("a: {a}, b: {b}");

    // addition
    let sum = 5 + 15;
    // subtraction
    let difference = 12.8 - 4.3;
    // multiplication
    let product = 4 * 32;
    // division
    let quotient = 16.8 / 2.4;
    let truncated = -5 / 3;
    // remainder
    let remainder = 45 % 13;
    println!("sum: {}, difference: {}, product: {}, quotient: {}, truncated: {}, remainder: {}",
        sum, difference, product, quotient, truncated, remainder
    );

    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");

    let w = 'w';
    let m: char = 'M';
    let heart_eye_cat = '😻';
    println!("w: {w}, m: {m}, {heart_eye_cat}");

    let tup = ('🚀', 100, 6.6, false);
    let (a, b, c, d) = tup;
    println!("{a}, {b}, {c}, {d}");
    println!("a = {}", tup.0);

    let arr_a: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_b = [3; 5];
    println!("arr_a[1] = {}, arr_b[3] = {}", arr_a[1], arr_b[3])
}
