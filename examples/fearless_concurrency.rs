use std::thread;
use std::sync::{Mutex, Arc, mpsc};
use std::time::Duration;
use num_cpus;

fn main() {
    // thread::sleep(Duration::from_secs(2));
    println!("Number of logical cores is {}", num_cpus::get());

    let v = vec![1..7];
    let w = vec![1..10];

    let handle = thread::spawn(move || {
        println!("Here's a moved vector: {:?}", v);
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // println!("Here's a vector: {:?}", v);  // error at compile, cause variable is moved to spawned thread
    println!("Here's a vector: {:?}", w);

    // drop(v);

    // handle.join().unwrap();  // will cause main thread wait the spawned thread to finish before the next code executed

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // Channel, mpsc (multi producer, single consumer)
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("huuft"),
            String::from("bapanada"),
            String::from("shaw"),
            String::from("gashaah"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // ownership changes
            thread::sleep(Duration::from_millis(300));
            // println!("Value : {}", val);  // error at compile, cause ownership already changed
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // ownership changes
            thread::sleep(Duration::from_millis(250));
            // println!("Value : {}", val);  // error at compile, cause ownership already changed
        }
    });

    for i in 1..15 {
        let received = rx.try_recv();
        let received = match received {
            Ok(val) => val,
            Err(error) => String::from("error"),
        };

        match received.as_str() {
            "error" => {
                println!("Waiting for transmission...");
                thread::sleep(Duration::from_millis(200));
                // thread::sleep(Duration::from_secs(1));
            },
            _ => {
                println!("Got: {}", received);
            }
        }
    }

    // // Other way to do it
    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // Mutex (mutual exclusion)
    println!("\nMutex Examples...");
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("m = {:?}", m);
    }

    println!("m = {:?}", m);

    let mut num = m.lock().unwrap();
    *num = 7;
    drop(num);

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
