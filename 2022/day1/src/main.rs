#![allow(unused)]
use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("Couldnt read file");

    let mut elf_by_calories = input
        .split("\n\n")
        .map(|elf| {
            let sum = elf
                .split("\n")
                .filter(|elf| !elf.is_empty())
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum();
            sum
        })
        .collect::<Vec<u32>>();

    elf_by_calories.sort();
    elf_by_calories.reverse();

    let elf_with_most_calories = elf_by_calories.get(0).unwrap();

    println!("Elf with most calories: {}", elf_with_most_calories);

    let top_elves_calories: u32 = elf_by_calories.split_at(3).0.iter().sum();

    println!("Top Elves combined calories: {}", top_elves_calories);
}
