
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
    println!("Shoe Test Passed!");
}

struct Counter {
    value: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {value: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * 2) + 1;
        if self.value < 1000000 {
            Some(self.value)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    filters_by_size();

    let mut counter = Counter::new();
    for _ in 1..25 {
        println!("{:?}", counter.next());
    }

    let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a + b)
            .filter(|x| x % 2 == 0)
            .sum();
    println!("{}", sum);
}
