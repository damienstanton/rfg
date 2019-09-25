//! A look at `traits` compared to `interfaces`

/// Anything that has a way to be summarized
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

/// A tweet.
#[derive(Debug)]
pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("damienstanton"),
        content: String::from("Hi, everyone."),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let tweet = Tweet {
            username: String::from("damienstanton"),
            content: String::from("Hi, everyone."),
            reply: false,
            retweet: false,
        };
        assert_eq!(
            String::from("(Read more from @damienstanton...)"),
            tweet.summarize()
        );
    }

}
