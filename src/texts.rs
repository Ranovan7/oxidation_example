use std::fmt::Formatter;
use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        String::from("(This is the default `summarize` implementation...)")
    }
}

pub struct RandomText {
    pub text: String,
}

impl Summary for RandomText {
    fn summarize_author(&self) -> String {
        format!("@anonymous")
    }
}

impl Display for RandomText {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.text)
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
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn tweet_example() -> impl Summary {
    Tweet {
        username: "ranovan7".to_string(),
        content: String::from("Hewwo, Pawper Pwease!"),
        reply: false,
        retweet: false,
    }
}

pub fn notify<T>(item: &T) -> ()
    where T: Summary + Display
{
    println!("Breaking news! {}", item.summarize());
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::from("Hello!")   // uncomment to show failed test with custom message
}
