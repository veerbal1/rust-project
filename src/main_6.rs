use std::{thread, time::Duration};
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=100 {
            println!("Hello from the spawned thread! {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    handle.join().unwrap();

    for i in 1..=10 {
        println!("Hello from the main thread! {}", i);
        thread::sleep(Duration::from_millis(2));
    }
}
