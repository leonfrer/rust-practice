use crate::traitz::{NewsArticle, Summary, Twitter};

mod traitz;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let result = traitz::_largest(&number_list);
    let result = traitz::largest_alt(&number_list);
    println!("{}", result);

    let tweet = Twitter {
        username: String::from("some_user"),
        content: String::from("some text for tweet!"),
        reply: false,
        retweet: true,
    };
    println!("new tweet: {}", tweet.summarize());
    println!();

    println!("traits as parameters");
    let news1 = NewsArticle {
        headline: String::from("text1"),
        location: String::from("text1"),
        author: String::from("text1"),
        content: String::from("text1"),
    };
    let news2 = NewsArticle {
        headline: String::from("text2"),
        location: String::from("text2"),
        author: String::from("text2"),
        content: String::from("text2"),
    };
    traitz::two_same_type_notify(&news1, &news2);
    traitz::two_diff_type_notify(&news1, &tweet);
    println!();

    println!("returning types that implement traits...");
    _ = traitz::return_summarizable();
    println!();

    println!("using trait bounds to conditionally implement methods");
    let p1 = traitz::Pair::new(1, 2);
    p1.cmp_display();
}
