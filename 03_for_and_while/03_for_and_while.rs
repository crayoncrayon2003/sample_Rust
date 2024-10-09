
fn main() {
    println!("-------forループ------");
    for i in 0..10 {
        println!("{}", i);
    }

    println!("-------loopループ------");
    let mut n = 0;
    loop {
        n += 1;
        if n == 2 {
            continue;
        }
        if n == 8 {
            break;
        }
        println!("{}", n);
    }

    println!("-------whileループ------");
    let mut n = 0;
    while n < 10 {
        n += 1;
        println!("{}", n);
    }

}