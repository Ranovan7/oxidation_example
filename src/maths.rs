#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 300 {  // uncomment to see unpanicked test
        if value < 1 || value > 100 {
            // panic!("Guess value must be between 1 and 100, got {}.", value);  // uncomment to see failed test that checked panic message
            panic!("Guess value must be less than or equal to 100");
        }

        Guess { value }
    }
}
