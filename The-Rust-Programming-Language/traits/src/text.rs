use std::fmt;
use crate::aggregator;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl aggregator::Summary1 for NewsArticle {
    fn summarize(&self) -> String {
        format!("(Summary1) {}, by {} ({})", self.headline, self.author, self.location)
    }
}
impl aggregator::Summary2 for NewsArticle {}
impl aggregator::Summary3 for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl aggregator::Summary1 for Tweet {
    fn summarize(&self) -> String {
        format!("(Summary1) {}: {}", self.username, self.content)
    }
}
impl aggregator::Summary2 for Tweet {}
impl aggregator::Summary3 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}