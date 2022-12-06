#![allow(unused, unused_imports)]
use core::str::Lines;
use itertools::Itertools;
use std::fs;
use std::ops::Range;

#[allow(unused)]
#[allow(unused_imports)]
fn main() {
    let input = fs::read_to_string("./input").unwrap();
    //.expect("no file")
    //.lines()
    //.map(|l| l.to_string());
    //.collect();
    let example = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let out2 = fs::read_to_string("./input").expect("noo");

    //let out = example
    let out = input
        .lines()
        .flat_map(|s| {
            s.split(&['-', ','][..])
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        //.collect::<Vec<_>>()
        //.chunks(2)
        .collect_tuple::<u32>()
        .map(|a| (a.first().unwrap()..=a.get(1).unwrap()))
        .collect::<Vec<_>>()
        .chunks(2)
        .filter(|c| {
            let r1 = c.first().unwrap();
            let r2 = c.get(1).unwrap();
            return r1.contains(r2.start())
                || r1.contains(r2.end())
                || r2.contains(r1.start())
                || r2.contains(r1.end());
        })
        .count();
    println!("{:?}", out);

    let v: Vec<&str> = "abc1d-ef2ghi"
        .split(|c: char| c.is_ascii_punctuation())
        .collect();
    println!("{:?}", v)
}

fn to_range(s: String) {
    let nums: Vec<u32> = s.split('-').map(|s| s.parse().unwrap()).collect();
    let r = Range {
        start: nums.first().unwrap(),
        end: nums.get(1).unwrap(),
    };
}
