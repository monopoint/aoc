#![allow(unused, unused_imports)]
use core::str::Lines;
use itertools::Itertools;
use std::any::type_name;
use std::char::MAX;
use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::ops::Range;
use std::time::Instant;

use std::{fs, num};
#[macro_use]
extern crate scan_fmt;
use measure_time::{debug_time, error_time, info_time, print_time, trace_time};

fn main() {
    let input_char: Vec<char> = fs::read_to_string("./input")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let input_u8: Vec<u8> = fs::read_to_string("./input")
        .unwrap()
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();

    let now = Instant::now();

    //part1(&input_char);
    test();

    let duration = now.elapsed();
    println!("took {}µs\n", duration.as_micros());
}

fn part1(input: &[char]) {}

fn test() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
        .split("$");

    input.into_iter().for_each(|command| {
        let c = parse_command(&command.to_string());
        println!("{}", command);
    })
}

struct File {
    name: String,
    size: u32,
    list: Option<Box<Vec<File>>>,
    parent: Option<Box<File>>,
}

impl File {
    fn sum(&self) -> u32 {
        match &self.list {
            Some(inner) => (*inner).iter().map(|a| a.size).sum::<u32>() + self.size,
            None => self.size,
        }
    }
}

struct Command {
    name: String,
    arg: Option<String>,
    output: Option<Vec<String>>,
}

fn parse_command(s: &String) -> Command {
    let mut block = s.split('\n').collect_vec();
    let com = block.remove(0);
    let output = block.iter().map(|s| s.to_string()).collect_vec();

    if com.starts_with("$ ls") {
        let c = Command {
            name: String::from("ls"),
            arg: None,
            output: Some(output),
        };
        c
    } else {
        // cd
        let c = Command {
            name: String::from("cd"),
            arg: Some(String::from(com.split(" ").last().unwrap())),
            output: None,
        };
        c
    }
}

fn parse_file(s: &String, parent: File) -> File {
    let s = s.clone();
    let mut sp = s.split(" ").collect_vec();
    //let first = sp.remove(0);

    // File
    if sp.first().unwrap().chars().next().unwrap().is_ascii_digit() {
        let f = File {
            name: sp.last().unwrap().to_string(),
            size: sp.first().unwrap().parse::<u32>().unwrap().clone(),
            list: None,
            parent: None,
        };
        f

    // Dir
    } else {
        let f = File {
            name: sp.last().unwrap().to_string(),
            size: sp.first().unwrap().parse::<u32>().unwrap().clone(),
            list: None,
            parent: None,
        };
        f
    }
}

// ------------------------------------
// ------------------------------------

fn find_unique_permutations(input: &[char], window: usize) -> usize {
    input
        .windows(window)
        .enumerate()
        .find(|(i, w)| {
            w.iter()
                .sorted()
                .tuple_windows::<(&char, &char)>()
                .all(|w| w.0 != w.1)
        })
        .unwrap()
        .0
        + window
}

fn all_unique(slice: &[char]) -> bool {
    slice
        .iter()
        .sorted()
        .tuple_windows::<(&char, &char)>()
        .all(|w| w.0 != w.1)
}

fn all_unique2(slice: &[char]) -> bool {
    for i in 0..slice.len() {
        if slice[i + 1 as usize..slice.len()].contains(&slice[i]) {
            return false;
        }
    }
    true
}

fn evaluate_by_exit(input: &[char], window: usize) -> usize {
    input
        .windows(window + 1)
        .enumerate()
        .find(|(i, w)| {
            if w[1..w.len()].contains(&w[0]) {
                // next iteration removes a duplicate
                all_unique2(&w[1..w.len()])
            } else {
                false
            }
        })
        .unwrap()
        .0
        + window
        + 1
}

fn karlsens_metode(input: &[char]) {
    let mut marker_1 = 4;
    let mut marker_2 = 14;
    let mut ans_1: i32 = -1;
    let mut prev_pos = &mut [0; 27];
    let mut i = 0;
    loop {
        let c = input[i] as usize - 'a' as usize;
        let pp = prev_pos[c];

        if i - pp < 4 {
            marker_1 = max(marker_1, pp + 4);
        }
        if i - pp < 14 {
            marker_2 = max(marker_2, pp + 14);
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

fn speed_check(bytes: &[u8], size: usize) -> usize {
    bytes
        .windows(size)
        .position(move |set| {
            let mut data: usize = 0;
            for &c in set {
                let prev = data;
                data |= 1 << (c - b'a');
                if prev == data {
                    return false;
                }
            }
            return true;
        })
        .unwrap()
        + size
}
