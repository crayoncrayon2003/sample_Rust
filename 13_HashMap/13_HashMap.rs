fn main() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("x", 10);
    map.insert("y", 20);
    map.insert("z", 30);
    println!("{} {} {}", map["x"], map["y"], map["z"]);

    println!("----------");

    for (k, v) in &map {
        println!("{} {}", k, v);
    }
}