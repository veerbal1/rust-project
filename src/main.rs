use std::{thread, time::Duration};
fn main() {
    let v = vec![1, 2, 3, 4];

    let handler = thread::spawn(move || {
        println!("{:?}", v);
    });
}
