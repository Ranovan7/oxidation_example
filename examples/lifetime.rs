use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn changesome(x: &mut i32) -> () {
    *x += 1;
}

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    match string1.len() >= string2.len() {
        true => string1,
        _ => string2,
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn highest<'a, 'b>(x: &'a i32, y: &'b i32) -> i32 {
    match x >= y {
        true => x.clone(),
        false => y.clone(),
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("\n--- LIFETIME ---");

    println!("\n# simple example");
    let r;
    let mut s = 10;

    {
        let x = 5;
        r = &x;
        println!("r borrowing: {}", r);
    }

    changesome(&mut s);
    // println!("r: {}", r);   // uncommment to show reference error
    println!("s: {}", s);

    println!("\n# function with lifetime");
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let sentence = "Suisei Hoshimachi";
    let first = first_word(sentence);
    println!("The first word is {}", first);

    println!("\n# function with multiple lifetimes");
    {
        let x = 5;
        println!("highest: {}", highest(&s, &x));
    }

    println!("\n# struct with lifetime");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
    println!("{}", i.level());
    i.announce_and_return_part("You sent for me?");

    longest_with_an_announcement(string1.as_str(), string2, i.part);

    println!("\n# static lifetime");
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}
