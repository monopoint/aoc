#![allow(unused, unused_imports)]
use std::{fs, num};
fn main() {
    let input_char: Vec<char> = fs::read_to_string("./input")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
}
