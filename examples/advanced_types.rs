type Kilometers = i32;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    print_type_of(&y);

    // SIMPLIFICATION
    // ------THIS BLOCK------- //
    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    //
    // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    //     // --snip--
    // }
    //
    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    //     Box::new(|| println!("sigh, bapanada"))
    // }
    // ----------------------- //

    // -----SAME AS ABOVE----- //

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("sigh, bapanada"))
    }
    // ----------------------- //
    // SIMPLIFICATION END

    let normal: u16 = 0;
    let integer: i16 = 0;

    // println!("Unsigned Int 0 -1 : {}", normal - 1);  // compile error : cause overflow
    println!("Signed Int 0 -1 : {}", integer - 1);
}
