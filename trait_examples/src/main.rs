use aggregator::{Summary, Tweet, NewsArticle, Paper};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    let news = NewsArticle {
        headline: String::from("Giant pandas enjoy themselves at breeding base in Sichuan"),
        location: String::from("China"),
        author: String::from("huaxia"),
        content: String::from("A giant panda feeds on bamboo at the Dujiangyan base of the China Conservation and Research Center for Giant Panda in Dujiangyan, southwest China's Sichuan Province, Feb. 15, 2023. (Xinhua/Wang Xi)"),
    };
    let paper = Paper {
        title: String::from("Attention Is All You Need"),
        authors: vec!["Ashish Vaswani", "Noam Shazeer", "Niki Parmar", "Jakob Uszkoreit",
        "Llion Jones", "Aidan N. Gomez", "Łukasz Kaiser", "Illia Polosukhin"].iter().map(|&s| String::from(s)).collect(),
        year: 2017,
        publication: String::from("MIT Press"),
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new news article: {}", news.summarize());
    println!("1 new paper: {}", paper.summarize());
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
