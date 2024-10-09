use std::{thread,time};

fn mycount

fn main() {
    let v = vec![1, 2, 3];

    // スレッド外部の変数をスレッド内部で触る時は、moveで所有権を与える
    // let handle = thread::spawn(|| {
    let handle = thread::spawn(move || {
        // thread code
        for i in v {
            println!("{}", i);
            // thread sleep
            thread::sleep(time::Duration::from_millis(100));
        }
    });
    // wait thread fin
    handle.join().unwrap();

    return;

}