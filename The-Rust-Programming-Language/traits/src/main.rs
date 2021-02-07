#![allow(dead_code, unused_variables)]

// Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior. The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior.

pub mod text;
use text::{ Tweet, NewsArticle};
pub mod aggregator;
// use aggregator::Summary1;
// use aggregator::Summary2;
use aggregator::Summary3;
use aggregator::{
    notify0,
    notify1,
    notify_all,
    notify_display,
    notify_display0,
    notify_display_where,
    notify_all_display,
};

mod pair;
use pair::Pair;

fn use_pair() {
    let p1 = Pair::new(0, 0);
    let p2 = Pair::new(5, 0);
    p2.cmp_display();
}


fn main() {
    use_pair();
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    //  Blanket implementation
    // impl<T: Display> ToString for T {
    // --snip--

    println!("{}", tweet.to_string());

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify0(&tweet);
    notify1(&tweet);
    notify_all(&tweet, &tweet, &tweet);
    notify_display(&tweet);
    notify_display0(&tweet);
    notify_display_where(&tweet);
    notify_all_display(&tweet, &tweet, &article);
    let summary3 = returns_summarizable();
    println!("{}", summary3.summarize());
}

fn returns_summarizable() -> impl Summary3 {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}