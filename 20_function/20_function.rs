fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn aaa() {
    println!("aaa");
    return ;
}

fn main() {
    aaa();
    let ret = add(1, 2);
    println!("{}", ret);
}