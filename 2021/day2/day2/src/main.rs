#![allow(unused)]
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read file");
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;

    input
        .split('\n')
        .filter(|inst| !inst.is_empty())
        .for_each(|inst| {
            let mut instruction_parts = inst.split_whitespace();
            let command = instruction_parts.next().expect("No command");
            let value: u32 = instruction_parts.next().expect("No value").parse().unwrap();

            match command {
                "up" => {
                    aim -= value;
                }
                "down" => {
                    aim += value;
                }
                "forward" => {
                    distance += value;
                    depth += (aim * value);
                }
                _ => {
                    println!("No match at {}", inst);
                }
            }
        });

    println!("done");
    println!("Depth {}", depth);
    println!("distance {}", distance);
    println!("product {}", depth * distance);
}
