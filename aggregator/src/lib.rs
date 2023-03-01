pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

// traits as parameters

// `impl Trait` syntax
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// trait bound syntax
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// Specifying Multiple Trait Bounds with the + Syntax
// pub fn notify<T: Summary + Display>(item: &T) {
//     // some codes
// }
// pub fn notify(item: &(impl Summary + Display)) {
//     //some codes
// }

// Clearer Trait Bounds with where Clauses
// pub fn some_function<T, U>(t: &T, u: &U) -> u32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     // some codes
// }

// Returning Types that Implement Traits
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        String::from(&self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Paper {
    pub title: String,
    pub authors: Vec<String>,
    pub year: u32,
    pub publication: String,
}

impl Summary for Paper {
    fn summarize_author(&self) -> String {
        String::from(self.authors.get(0).unwrap())
    }
}
