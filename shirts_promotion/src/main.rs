fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };

    let user_pref_1 = Some(ShirtColor::Red);
    let give_away_1 = store.give_away(user_pref_1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref_1, give_away_1
    );

    let user_pref_2 = None;
    let give_away_2 = store.give_away(user_pref_2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref_2, give_away_2
    )
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // The `unwrap_or_else` method on `Option<T>` is defined by the standard library.
        // It takes one argument: a *closure* without any arguments that returns a value `T`.
        // Rust’s closures are *anonymous functions* you can save in a variable or pass as arguments to other functions.
        // `|| self.most_stocked()` is the argument to `unwrap_or_else`; if the *closure* had
        // parameters, they would appear **between the two vertical bar**.
        // 
        // The closure captures an immutable reference to the `self` `Inventory` instance and
        // passes it with the code we specify to the unwrap_or_else method.
        // Functions, on the other hand, are not able to capture their environment in this way. 
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}