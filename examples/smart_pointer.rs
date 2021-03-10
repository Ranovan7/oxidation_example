use std::ops::Deref;
use std::rc::Rc;
use crate::List::{Cons, Nil};

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // let _list = Cons(1, Cons(2, Cons(3, Nil)));   // error
    // let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // using custom Box/Smart Pointer
    let x = 5;
    let y = &x;
    let z = &y;
    let aa = Box::new(x); // behave the same as the y variable
    let ab = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, **z);
    assert_eq!(5, *aa);
    assert_eq!(5, *ab);

    // Using Rc (Reference Counted)
    let ac = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&ac));
    let ad = Cons(3, Rc::clone(&ac));
    println!("count after creating b = {}", Rc::strong_count(&ac));
    {
        let ae = Cons(4, Rc::clone(&ac));
        println!("count after creating c = {}", Rc::strong_count(&ac));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&ac));

    // Deref Coercion Example
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Drop trait implementation (automatic)
    let _c = CustomSmartPointer {
        data: String::from("first stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("second stuff"),
    };

    // Drop trait implementation (manual)
    let e = CustomSmartPointer {
        data: String::from("third stuff"),
    };
    println!("CustomSmartPointer created.");
    drop(e);

    println!("\nEnd of Program...\n");

    // Interior Mutability
    println!("For Interior Mutability, check mock_object.rs on the library");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
