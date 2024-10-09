fn main() {
    let mut vect = vec![10, 20, 30];
    vect.push(40);
    println!("{} {} {} {}", vect[0], vect[1], vect[2], vect[3]);

    for v in &vect {
        println!("{}", v);
    }
}