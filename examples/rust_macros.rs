use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let looper = vec![1..3, 1..5, 1..10];
    for l in looper {
        for i in l {
            println!("{:?}", i);
        }
    }

    Pancakes::hello_macro();
}
