use std::slice;

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len, "Attempt to slice a vector Error : slice index is greater than vector length.");

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x01234usize;
    let _r = address as *mut i32;

    let _slice: &[i32] = unsafe { slice::from_raw_parts_mut(_r, 10000) };
    // println!("{:?}", _slice);    // this will result in segmentation fault,
                                //because the line above it has no guarantee
                                //that the slice this code creates contains valid i32 values

    unsafe {
        println!("address is: {}", address);
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    const SOMNUM: u32 = 10;
    // const SOMNUM: f32 = 1.0;    // error at compile, see below
    // let mut COUNTER: u32 = 0;    // will error at compile
                                // because COUNTER already used by static var
    println!("{}", SOMNUM);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
