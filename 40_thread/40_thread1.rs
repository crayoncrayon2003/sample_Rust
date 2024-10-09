use std::{thread,time};

fn main() {
    let handle = thread::spawn(|| {
        // thread code
        for i in 0..10 {
            println!("{}", i);
            // thread sleep
            thread::sleep(time::Duration::from_millis(100));
        }
    });
    // wait thread fin
    handle.join().unwrap();

    return;

}