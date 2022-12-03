#[allow(unused_imports)]
use core::str::Lines;
use itertools::Itertools;
use std::{char, fs};

#[allow(unused)]
fn main() {
    let input: Vec<String> = fs::read_to_string("./input")
        .expect("no file")
        .lines()
        .map(|l| l.to_string())
        .collect();

    // Part 1
    let part1: usize = input
        .clone()
        .iter()
        .map(|a| {
            let splitter = a.split_at(a.len() / 2);
            find_dupes(splitter.0, splitter.1)
                .chars()
                .take(1)
                .next()
                .unwrap()
        })
        .map(|a| pri(a))
        .sum();
    println!("part1: {}", part1);

    // Part 2
    let part2: usize = input
        .chunks(3)
        .map(|chunk| {
            find_dupes(
                chunk.get(0).unwrap(),
                &find_dupes(chunk.get(1).unwrap(), chunk.get(2).unwrap()),
            )
            .chars()
            .take(1)
            .next()
            .unwrap()
        })
        .map(pri)
        .sum();
    println!("part 2: {}", part2);
}

fn find_dupes(a: &str, b: &str) -> String {
    a.chars().filter(|x| b.contains(*x)).unique().collect()
}

fn pri(c: char) -> usize {
    let mut pri: Vec<char> = (b'a'..=b'z').map(|a| a as char).collect();
    pri.extend((b'A'..=b'Z').map(|a| a as char).collect::<Vec<char>>());
    pri.iter().position(|b| b == &c).unwrap() + 1
}
