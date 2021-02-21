mod coin;
use coin::Coin;

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("'six' value is : {:?}", six.unwrap());
    // println!("'none' value is : {:?}", none.unwrap());

    let a_penny = Coin::Penny;
    if let Coin::Penny = a_penny {
        println!("Penny is a Penny");
    }
}