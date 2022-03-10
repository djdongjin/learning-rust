use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// a function accepting a trait param
// if we want more trait bound, use `+`
// e.g.: (item: &(impl Summary + Display))
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// complete `trait bound syntax`
// <T: Summary + Display>
pub fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// use `where` when trait bound is too long.
fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    5
}

fn some_func_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Debug,
    U: Clone + Debug,
{
    5
}

fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    };

    notify(&article);
    notify_bound(&tweet);

    // let mut news: Vec<dyn Summary> = Vec::new();
    // for item in &news {
    //     println!("{}", item.summarize());
    // }
}
