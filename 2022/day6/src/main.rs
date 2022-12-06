#![allow(unused, unused_imports)]
use core::str::Lines;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::ops::Range;

use std::{fs, num};
#[macro_use]
extern crate scan_fmt;
use measure_time::{debug_time, error_time, info_time, print_time, trace_time};

fn main() {
    let input: Vec<char> = fs::read_to_string("./input").unwrap().chars().collect::<Vec<char>>();
    print_time!("measure function");
    println!("Part 1: {}", find_unique_permutations(&input, 4));
    println!("Part 2: {}", find_unique_permutations(&input, 14));
}

fn find_unique_permutations(input: &[char], window: usize) -> usize{
    input.windows(window).enumerate().find(|(i, w)| {
        w.iter().sorted().tuple_windows::<(&char, &char)>().all(| w| w.0 != w.1)
    }).unwrap().0+window
}
