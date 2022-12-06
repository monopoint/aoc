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
    let input = fs::read_to_string("./input").unwrap();
    print_time!("measure function");

    part1(&input);
}

fn part1(input: &String) {
    let pos = input
        .chars()
        .tuple_windows::<(char, char, char, char)>()
        .find_position(|t| {
            let (a, b, c, d) = t;
            let chars = [a, b, c, d];
            let set = chars.iter().collect::<HashSet<_>>();
            dbg!(&set);
            set.len() == 4
        })
        .map(|p| p.0)
        .unwrap();

    println!("pos {}", pos + 4);
}
