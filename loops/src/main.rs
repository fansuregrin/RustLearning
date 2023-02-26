fn main() {
    // returnings value from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // conditional loops with while
    let mut n = 3;
    while n != 0 {
        println!("{n}");
        n -= 1;
    }

    // looping through a collection with for
    let arr = [1, 2, 3, 4 , 5];
    for elem in arr {
        println!("The value is: {elem}");
    }

    // not compile
    // let a = ('🎈', "🧨", 6, '🏀', 3.5);
    // for elem in a {
    //     println!("{elem}");
    // }

    for number in (1..4).rev() {
        println!("{number}");
    }
}
