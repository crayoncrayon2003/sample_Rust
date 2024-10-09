fn main() {
    let arr = [10, 20, 30];
    println!("{} {} {}", arr[0], arr[1], arr[2]);

    for v in &arr {
        println!("{}", v);
    }
}