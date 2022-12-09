#![allow(unused, unused_imports)]
use std::{fs, num, time::Instant};

use itertools::Itertools;
use scan_fmt::parse;
use transpose::{transpose, transpose_inplace};

fn main() {
    let input_string = fs::read_to_string("./test").unwrap();

    let width = input_string.chars().position(|c| c == '\n').unwrap();
    let height = input_string.lines().count();

    let trees: Vec<Tree> = input_string
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|n| Tree {
            height: n.to_digit(10).unwrap(),
            seen: false,
            scenic: 1,
        })
        .collect_vec();
    let trees2 = trees.clone();

    let now = Instant::now();
    println!("Part 1: {}", part1(trees, &width, &height));
    //println!("Part 2: {}", part2(trees2, &width, &height));
    println!("Part 1_2: {}", part1_2(trees2, &width));

    let duration = now.elapsed();
    println!("1 iteration took {}µs\n", duration.as_micros());
}

#[derive(Debug, Copy, Clone)]
struct Tree {
    height: u32,
    seen: bool,
    scenic: u32,
}
fn part1(mut trees: Vec<Tree>, width: &usize, height: &usize) -> u32 {
    mark_seen(&mut trees, width, height);
    trees.reverse();
    mark_seen(&mut trees, width, height);
    let mut trees_transposed = trees.clone();
    transpose(&trees, &mut trees_transposed, *width, *height);
    mark_seen(&mut trees_transposed, width, height);
    trees_transposed.reverse();
    mark_seen(&mut trees_transposed, width, height);

    let seen_trees = trees_transposed
        .iter()
        .map(|t| t.seen)
        .filter(|b| *b)
        .count();

    print_trees(&trees_transposed, *width);
    seen_trees as u32
}

fn part2(mut trees: Vec<Tree>, width: &usize, height: &usize) -> u32 {
    mark_score(&mut trees, width, height);
    trees.reverse();
    mark_score(&mut trees, width, height);
    let mut trees_transposed = trees.clone();
    transpose(&trees, &mut trees_transposed, *width, *height);
    mark_score(&mut trees_transposed, width, height);
    trees_transposed.reverse();
    mark_score(&mut trees_transposed, width, height);

    let seen_trees = trees_transposed
        .iter()
        .map(|t| t.seen)
        .filter(|b| *b)
        .count();

    trees.iter().map(|t| t.scenic).max().unwrap()
}

fn mark_seen(trees: &mut [Tree], width: &usize, height: &usize) {
    for y in 0..*height {
        let mut max = 0;
        for x in 0..*width {
            let mut tree = trees.get_mut(x + (height * y)).unwrap();
            if tree.height > max || x == 0 {
                tree.seen = true;
                max = tree.height;
            }
        }
    }
}

fn mark_score(trees: &mut Vec<Tree>, width: &usize, height: &usize) {
    let trav_trees = trees.clone();
    for y in 0..*height {
        let mut max = 0;
        for x in 0..*width {
            let mut tree = trees.get_mut(x + (height * y)).unwrap();
            if (x == *width - 1) {
                tree.scenic = 0;
                continue;
            }
            let ss = (x + 1..*width)
                .into_iter()
                .map(|i| trav_trees.get(i + (height * y)).unwrap())
                .enumerate()
                .find(|(j, t)| t.height >= tree.height)
                .map(|f| f.0 + 1);
            tree.scenic *= ss.unwrap_or((*width - 1) - x) as u32;
        }
    }
}

fn part1_2(mut trees: Vec<Tree>, size: &usize) -> u32 {
    println!("Size: {}", size);
    mark_seen_by_range(&mut trees, 0, size - 1);
    mark_seen_by_range(&mut trees, size - 1, 0);

    let seen_trees = trees.iter().map(|t| t.seen).filter(|b| *b).count();
    seen_trees as u32
}

use std::cmp;
use std::ops::Range;
fn mark_seen_by_range(trees: &mut [Tree], x1: usize, x2: usize) {
    println!("{} {}", x1, x2);
    for y in x1..=x2 {
        let mut max1 = 0;
        let mut max2 = 0;
        for x in x1..=x2 {
            let idx = x + (y * cmp::max(x1 + 1, x2 + 1));
            let idx2 = y + (x * cmp::max(x1 + 1, x2 + 1));

            let mut tree = trees.get_mut(idx).unwrap();
            if tree.height > max1 || x == x1 || x == x2 || y == x1 || y == x2 {
                tree.seen = true;
                max1 = tree.height;
            }

            let mut tree = trees.get_mut(idx2).unwrap();
            if tree.height > max2 || x == x1 || x == x2 || y == x1 || y == x2 {
                tree.seen = true;
                max2 = tree.height;
            }
        }
    }
    print_trees(trees, cmp::max(x1, x2) + 1);
}

fn print_trees(trees: &[Tree], width: usize) {
    trees.chunks(width as usize).for_each(|c| {
        c.iter().for_each(|t| {
            let mut s = 0;
            if t.seen {
                s = 1;
            }
            print!("{}", s);
        });
        print!("\n");
    })
}
