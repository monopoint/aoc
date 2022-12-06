#![allow(unused, unused_imports)]
use core::str::Lines;
use itertools::Itertools;
use std::collections::VecDeque;
use std::ops::Range;
use std::{fs, num};
#[macro_use]
extern crate scan_fmt;
use measure_time::{debug_time, error_time, info_time, print_time, trace_time};

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    //print_time!("measure function");
    let stacks = parse_stacks(&input);
    let moves = parse_moves(&input, &stacks);

    //part1(stacks.clone(), &moves);
    //part2(stacks, &moves);

    println!(
        "{}",
        "abcdefghijlkmnopqrstuvwxyzæøå".chars().nth(8).unwrap()
    );
    println!(
        "{}",
        "abcdefghijlkmnopqrstuvwxyzæøå".chars().nth(6).unwrap()
    );
    println!(
        "{}",
        "abcdefghijlkmnopqrstuvwxyzæøå".chars().nth(22).unwrap()
    );
    println!(
        "{}",
        "abcdefghijlkmnopqrstuvwxyzæøå".chars().nth(2).unwrap()
    );
}

fn part1(mut stacks: Vec<VecDeque<char>>, moves: &Vec<(u32, usize, usize)>) {
    for m in moves {
        let (number_of_crates, fstack, tstack) = m;
        for i in 0..*number_of_crates {
            let c = stacks.get_mut(*fstack - 1).unwrap().pop_front().unwrap();
            stacks.get_mut(*tstack - 1).unwrap().push_front(c);
        }
    }
    let output: String = stacks.iter().map(|s| s.front().unwrap()).collect();
    println!("Part 1: {}", output);
}

fn part2(mut stacks: Vec<VecDeque<char>>, moves: &Vec<(u32, usize, usize)>) {
    for m in moves {
        let (number_of_crates, fstack, tstack) = m;
        let from_stack = stacks.get_mut(fstack - 1).unwrap();
        let new_from_stack = from_stack.split_off(*number_of_crates as usize);
        let mut crates = from_stack.clone();
        stacks[fstack - 1] = new_from_stack;
        crates.append(stacks.get_mut(tstack - 1).unwrap());
        stacks[tstack - 1] = crates;
    }

    let output: String = stacks.iter().map(|s| s.front().unwrap()).collect();
    println!("Part 2: {}", output);
}

fn parse_moves(input: &str, stacks: &[VecDeque<char>]) -> Vec<(u32, usize, usize)> {
    input
        .lines()
        .skip(10)
        .map(|line| scan_fmt!(line, "move {} from {} to {}", u32, usize, usize).unwrap())
        .collect::<Vec<(u32, usize, usize)>>()
}

fn parse_stacks(input: &str) -> Vec<VecDeque<char>> {
    let stack_input = input.lines().take(8).collect_vec();
    let mut stacks = std::iter::repeat(VecDeque::<char>::new())
        .take(9)
        .collect::<Vec<VecDeque<char>>>();

    let extr = stack_input
        .iter()
        .map(|l| {
            l.chars()
                .chunks(4)
                .into_iter()
                .map(|mut c| c.nth(1).unwrap())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    for line in &extr {
        for (i, c) in line.iter().enumerate() {
            if c.is_ascii_alphabetic() {
                let mut stack = stacks.get_mut(i).unwrap();
                stack.push_back(*c);
            }
        }
    }
    stacks
}
