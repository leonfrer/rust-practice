use std::fmt::Display;

pub fn _largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn largest_alt<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    largest
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct Twitter {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Twitter {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
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
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{} {} {}", self.headline, self.location, self.author)
    }
}

pub fn two_same_type_notify<T: Summary>(item1: &T, item2: &T) {
    println!("New Notify: \n{}\n{}", item1.summarize(), item2.summarize())
}

pub fn two_diff_type_notify(item1: &impl Summary, item2: &impl Summary) {
    println!("New Notify: \n{}\n{}", item1.summarize(), item2.summarize())
}

pub fn return_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("text2"),
        location: String::from("text2"),
        author: String::from("text2"),
        content: String::from("text2"),
    }
}

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Pair<T> {
        Self { x, y }
    }
}

impl<T> Pair<T>
where
    T: Display + PartialOrd,
{
    pub fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
