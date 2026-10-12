use std::thread;
use std::time::Duration;

fn dummy_function(a: i32, b: i32, sleep: bool) {
    println!("dummy_function({a}, {b})");
    println!("dummy_function output = {}", a + b);

    if sleep {
        println!("dummy_function sleeping...");
        thread::sleep(Duration::from_secs(5));
    }

    println!("dummy_function done.");
}


fn main() {
    let t1 = thread::spawn(|| dummy_function(12, 21, false));
    let t2 = thread::spawn(|| dummy_function(15, 51, true));

    println!("main waiting for threads to be done.");
    t1.join().unwrap();
    t2.join().unwrap();
    println!("main exiting.");
}