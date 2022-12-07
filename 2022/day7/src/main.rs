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

fn test<'a>() {
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
        .split('$');

    let mut root  = File { name: String::from("root"), size: (0), list: None, parent: None };
    let current = &root;

    input.into_iter().for_each(|command| {
        let c = parse_command(command);
        println!("{}", command);

        if c.name == "ls" {
            let mut current_list = current.list.clone().unwrap_or(Vec::<&File>::new());
            for l in c.output.unwrap() {
                let f = parse_file(l, &current).clone();
                current_list.push(&f); // WELL GOD DAMN!
            }            
        }
    })
}

#[derive(Clone)]
struct File<'b> {
    name: String,
    size: u32,
    list: Option<Vec<&'a File<'b>>>,
    parent: Option<&'a File<'b>>,
}

impl File<'_> {
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

fn parse_command(s: &str) -> Command {
    let mut block = s.split('\n').collect_vec();
    let com = block.remove(0);
    let output = block.iter().map(|s| s.to_string()).collect_vec();

    if com.starts_with("$ ls") {
        Command {
            name: String::from("ls"),
            arg: None,
            output: Some(output),
        }
   
    } else {
        // cd
       Command {
            name: String::from("cd"),
            arg: Some(String::from(com.split(' ').last().unwrap())),
            output: None,
        }
    }
}

fn parse_file<'a, 'b>(s: String, parent: &'a File) -> File <'b>{
    let s = s;
    let mut sp = s.split(' ').collect_vec();

    // File
    if sp.first().unwrap().chars().next().unwrap().is_ascii_digit() {
        let f = File {
            name: sp.last().unwrap().to_string(),
            size: sp.first().unwrap().parse::<u32>().unwrap(),
            list: None,
            parent: Some(parent),
        };
        f

    // Dir
    } else {
        let f = File {
            name: sp.last().unwrap().to_string(),
            size: 0,
            list: None,
            parent: Some(parent),
        };
        f
    }
}
