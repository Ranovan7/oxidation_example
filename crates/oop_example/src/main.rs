use oop_example::{ AveragedCollection, Draw, Button, Screen, Post, Post2 };

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing Select Box {:?}", self.options);
    }
}

fn main() {
    println!("Hello, world!");

    let mut somelist = AveragedCollection::new(vec![1,2,3,4,5,6,7]);
    println!("Avg : {}", somelist.average());

    somelist.add(8);

    println!("Avg : {}", somelist.average());

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            // Box::new(String::from("Hi")),  // error at compile, because it's not implementing `Draw` trait
        ],
    };

    screen.run();

    // Blog Post Example
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // Alternative
    let mut post = Post2::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
