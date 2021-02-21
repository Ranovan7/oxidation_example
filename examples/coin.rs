use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub struct Wallet {
    pub coins: Vec<Coin>
}

#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Distribution<Coin> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Coin {
        match rng.gen_range(0..4) {
            0 => Coin::Penny,
            1 => Coin::Nickel,
            2 => Coin::Dime,
            _ => Coin::Quarter
        }
    }
}

pub fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
