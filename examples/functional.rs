use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn generate_workout(intensity: u32, random_number: u32) {
    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));
    // let n = example_closure(5);  // this line will fail to compile

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };
    let fast_closure = |num| num;

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", fast_closure(intensity));
        }
    }
}

struct Catcher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Catcher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Catcher<T> {
        Catcher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if self.value.contains_key(&arg) {
            self.value[&arg]
        } else {
            let v = (self.calculation)(arg);
            self.value.insert(v, v);
            v
        }
    }
 }

fn generate_workout_v2(intensity: u32, random_number: u32) {
    let mut expensive_result = Catcher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn call_with_different_values() {
    let mut c = Catcher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
    println!("Catcher Test passed!");
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout_v2(simulated_user_specified_value, simulated_random_number);

    call_with_different_values();

    let x = 4;
    let equal_to_x = |z| z == x;    // this cannot work on normal function,
                                    // hint: use of x variable beside not being a param
    let y = 4;
    assert!(equal_to_x(y));
}
