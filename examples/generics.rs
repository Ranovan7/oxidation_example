mod lib;
use crate::lib::*;


#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// below are struct that both x and y are the same type
// struct Point<T> {
//     x: T,
//     y: T,
// }

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct Square {
    height: i32,
    width: i32
}

impl Square {
    fn perimeter(&self) -> i32 {
        &self.height + &self.height + &self.width + &self.width
    }

    fn volume(&self) -> i32 {
        &self.height * &self.width
    }
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest: &char = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn largest_v2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    println!("\n--- GENERIC DATA TYPES ---");
    let number_list = vec![12,5,23,35,62,91];

    let result = largest_i32(&number_list);
    println!("The Largest number is {}", result);
    println!("From List {:?}", number_list);

    let somestr = vec!['s','u','i','c','o','p','a','t','h'];

    let result = largest_char(&somestr);
    println!("The Largest char is {}", result);
    println!("From String {:?}", somestr);

    println!("\n# struct templating");
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = Point { x: 10.5, y: 40 };
    println!("Integer Point {:?}", integer);
    println!("Float Point {:?}", float);
    println!("Mixed Point {:?}", mixed);

    let chaos = mixed.mixup(float);
    println!("Chaos Point {:?}", chaos);

    println!("\n--- TRAITS ---");

    println!("\n# direct implementation");
    let somebox = Square {height: 10, width: 6};
    println!("Box Perimeter {:?}", somebox.perimeter());
    println!("Box Volume {:?}", somebox.volume());

    println!("\n# template implementation");
    let tweet = tweet_example();
    println!("{} - {}", tweet.summarize(), tweet.summarize_author());

    let news = NewsArticle {
        headline: "Oxidation".to_string(),
        location: String::from("Iron"),
        author: "Man".to_string(),
        content: String::from("Rustification are observed"),
    };
    println!("{}", news.summarize());

    let defimpl = RandomText { text: "random".to_string() };
    println!("{}", defimpl.summarize());
    println!("{}", defimpl.summarize_author());
    notify(&defimpl);

    println!("\n# template functions");
    let result = largest(&number_list);
    let result_v2 = largest_v2(&number_list);
    println!("The Largest number is {}", result);
    println!("The Largest number v2 is {}", result_v2);
    println!("From List {:?}", number_list);

    println!("\n# others");
    let pair = Pair {x: 1129, y: -5321};
    pair.cmp_display();
}
