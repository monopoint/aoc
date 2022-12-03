#[allow(unused_imports)]
use core::str::Lines;
use itertools::Itertools;
use std::{char, fs};

#[allow(unused)]
fn main() {
    let file = fs::read_to_string("./input").expect("no file");
    let part1_input = file.lines();

    // Tests
    let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp";
    let test_splitter = test_input.split_at(test_input.len() / 2);

    // Part 1
    let part1: usize = part1_input.map(|a| find_duplicate(a)).map(|a| pri(a)).sum();
    println!("part1: {}", part1);

    // Part 2
    let part2_input = file
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let part2: usize = part2_input
        .chunks(3)
        .map(|chunk| {
            let dup1 = find_dupes(chunk.get(0).unwrap(), chunk.get(1).unwrap());
            let dup2 = find_dupes(chunk.get(2).unwrap(), &dup1);
            let c = dup2.chars().take(1).next().unwrap();
            c
        })
        .map(pri)
        .sum();

    println!("part 2: {}", part2);
}

fn find_duplicate(input: &str) -> char {
    let splitter = input.split_at(input.len() / 2);
    let duplicate = splitter
        .0
        .chars()
        .find(|a| splitter.1.contains(*a))
        .unwrap();
    duplicate
}

fn find_dupes(a: &str, b: &str) -> String {
    let dupes: String = a.chars().filter(|x| b.contains(*x)).unique().collect();
    dupes
}

fn pri(c: char) -> usize {
    let mut pri: Vec<char> = (b'a'..=b'z').map(|a| a as char).collect();
    pri.extend((b'A'..=b'Z').map(|a| a as char).collect::<Vec<char>>());
    let priority = pri.iter().position(|b| b == &c).unwrap() + 1;
    priority
}

// fn priority(a: char) -> i32 {}
