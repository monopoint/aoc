#![allow(unused, unused_imports)]
use core::str::Lines;
use std::char::MAX;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::ops::Range;
use std::time::Instant;
use std::cmp::max;

use std::{fs, num};
#[macro_use]
extern crate scan_fmt;
use measure_time::{debug_time, error_time, info_time, print_time, trace_time};

fn main() {
    let input_char: Vec<char> = fs::read_to_string("./input").unwrap().chars().collect::<Vec<char>>();
    let input_u8: Vec<u8> = fs::read_to_string("./input").unwrap().chars().map(|c| c as u8).collect::<Vec<u8>>();
    //print_time!("measure function");
    let now = Instant::now();
    println!("exit_compare :: Part 1: {}", evaluate_by_exit(&input_char, 4));
    println!("exit_compare :: Part 2: {}", evaluate_by_exit(&input_char, 14));
    let duration = now.elapsed();
    println!("took {}µs\n", duration.as_micros());

    let now = Instant::now();
    println!("Karlsens metode:");
    karlsens_metode(&input_char);
    let duration = now.elapsed();
    println!("took {}µs\n", duration.as_micros());

    let now = Instant::now();
    println!("Speed Check :: Part 1 {}", speed_check(&input_u8, 4));
    println!("Speed Check :: Part 2 {}", speed_check(&input_u8, 14));
    let duration = now.elapsed();
    println!("took {}µs\n", duration.as_micros());
}

fn find_unique_permutations(input: &[char], window: usize) -> usize{
    input.windows(window).enumerate().find(|(i, w)| {
        w.iter().sorted().tuple_windows::<(&char, &char)>().all(| w| w.0 != w.1)
    }).unwrap().0+window
}

fn all_unique(slice: &[char]) -> bool {
    slice.iter().sorted().tuple_windows::<(&char, &char)>().all(| w| w.0 != w.1)
}

fn all_unique2(slice: &[char]) -> bool {
   for i in 0..slice.len() {
    if slice[i + 1 as usize..slice.len()].contains(&slice[i]) { return false}
   }
   true
}

fn evaluate_by_exit(input: &[char], window: usize) -> usize{
    input.windows(window+1).enumerate().find(|(i, w)| {
     if w[1..w.len()].contains(&w[0]) { // next iteration removes a duplicate
        all_unique2(&w[1..w.len()])
    } else {
        false
    }
    }).unwrap().0+window+1
}

fn karlsens_metode(input: &[char]) {
    let mut marker_1 = 4;
    let mut marker_2 = 14;
    let mut ans_1: i32 = -1;
    let mut prev_pos = &mut [0;27];
    let mut i = 0;
    loop {
        let c = input[i] as usize - 'a' as usize;
        let pp = prev_pos[c];

        if i-pp < 4 {
            marker_1 = max(marker_1, pp+4);
        }
        if i-pp < 14 {
            marker_2 = max(marker_2, pp+14);
        }

        if ans_1 == -1 && i == marker_1 {
            ans_1 = 1 + marker_1 as i32;
        }

        if (i == marker_2) {
            println!("1: {}", ans_1);
            println!("2: {}", 1 + i);
            break;
        }

        prev_pos[c] = i;
        i += 1;
    }
}

fn speed_check(bytes: &[u8], size: usize) -> usize{
    bytes
        .windows(size)
        .position(move |set| {
             let mut data: usize = 0;
             for &c in set {
                 let prev = data;
                 data |= 1 << (c -b'a');
                 if prev == data{
                     return false;
                 }
             }
             return true;
         }).unwrap() + size
}

