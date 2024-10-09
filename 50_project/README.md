# create project
```
$ cargo new sample_project
```

# project setting 
for sample_project/Cargo.toml

## [package]
### original
```
[package]
name = "sample_project"
version = "0.1.0"
edition = "2021"
```

### change
```
[package]
name = "sample_project"
version = "0.1.0"
authors = ["user"]
edition = "2024"

```
## [dependencies]
### original
```
[dependencies]
```

### change
```
[dependencies]
rand = "0.8.3" # impost rand
```

# src
for sample_project/src/main.rs
```
extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let i: i32 = rng.gen();
    let f: f32 = rng.gen();
    println!("i = {}", i);
    println!("f = {}", f);
}
```

# run 
```
$ cargo build
$ cargo run
```