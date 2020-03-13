// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x: i32 = input.trim().parse().unwrap();

    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
