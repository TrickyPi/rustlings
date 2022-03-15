// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)
use std::convert::TryInto;

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> u32 {
    (num * num).try_into().unwrap()
}
