#[allow(unused)]
fn main() {
    let b = Point {x: 5, y: 6.4};
    let c = Point {x: "hello", y: '🤗'};
    println!("b.x = {}, b.y = {}", b.x(), b.y());
    let d = b.mixup(c);
    println!("d.x = {}, d.y = {}", d.x(), d.y());
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<A, B>(self, other: Point<A, B>) -> Point<T, B> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
