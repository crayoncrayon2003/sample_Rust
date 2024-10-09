fn main() {
    let filename = "hello.txt";

    let mut file = match File::create(filename) {
        Err(why) => panic!("Couldn't create {}: {}", filename, why),
        Ok(file) => file,
    };

    let contents = "こんにちは、Rust!";
    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("Couldn't write \"{}\" to {}: {}", contents, filename, why),
        Ok(_) => println!("finished"),
    }
}

