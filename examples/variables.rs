fn change_example(mut example: i32) -> i32 {
    example += 1;
    example
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut example: i32 = 32;
    example = change_example(example);
    println!("Example = {}", example);

    let borrow = &mut example;
    println!("Borrowing {}", borrow);
    *borrow += 1;
    println!("Borrow = {}", borrow);

    let mut short = borrow.clone();
    short += 1;
    println!("Shorted = {}", short);

    let steal = example;
    example += 1;
    println!("Stealing {}", steal);
    println!("Example {}", example);
}
