fn main() {
    let rect1 = Rectangle {
        width: 4,
        height: 5
    };

    println!("The area of rect1: {}", area(&rect1));
    println!("Rectangle[w={}, h={}]", rect1.width, rect1.height);
    println!("{:?}", rect1);
    println!("{:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 20
    };

    dbg!(&rect2);

    println!("The area of rect2: {}", rect2.area());
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::square(5);

    println!("The area of rect3: {}", rect3.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    // The parameter `rectangle` is an immutable borrow of a struct `Rectangle` instance
    // because we donot want to take ownership of it.
    rectangle.width * rectangle.height
}

// associated functions
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}
