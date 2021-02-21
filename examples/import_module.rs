mod coin;
use coin::*;

fn main() {
    let mut wallet = Wallet{
        coins: vec![Coin::Dime, Coin::Nickel]
    };

    for _ in 1..100 {
        wallet.coins.push(rand::random())
    }

    let mut value: u32 = 0;
    for coin in wallet.coins {
        value += value_in_cents(coin);
    }

    println!("Your wallet value is : {}", value);
}
