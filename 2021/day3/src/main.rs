#![allow(unused)]
use std::{fs, num};

fn main() {
    let input = fs::read_to_string("./input").expect("Couldnt read file");
    let data = input.split('\n').collect::<Vec<&str>>();
    const DATA_WIDTH: usize = 12;

    let most_common = most_common_by_position(&data, 0);

    let gamma = (0..DATA_WIDTH)
        .map(|pos| return most_common_by_position(&data, pos))
        .collect::<String>();
    println!("gamma is {}", gamma);

    let epsilon = (0..DATA_WIDTH)
        .map(|pos| return least_common_by_position(&data, pos))
        .collect::<String>();
    println!("gamma is {}", epsilon);

    let gamma_int = u32::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon_int = u32::from_str_radix(epsilon.as_str(), 2).unwrap();

    println!("Product is {}", gamma_int * epsilon_int);

    // Find Oxygen generator rating
    let data_range = (0..DATA_WIDTH);
    let mut oxygen = data;
    for i in data_range {
        let most_common = most_common_by_position(&oxygen, i);

        oxygen = *oxygen
            .iter()
            .filter(|d| d.chars().nth(i).unwrap() == most_common)
            .collect();
    }
    let mut oxygen = data
        .iter()
        .filter(|d| d.chars().nth(0).unwrap() == most_common_by_position(&data, 0));
}

fn most_common_by_position(arr: &Vec<&str>, position: usize) -> char {
    let length_of_dataset = arr.len();

    let number_of_ones = count_ones(arr, position);
    let number_og_zeros = length_of_dataset - number_of_ones;

    let number_og_zeros = length_of_dataset - number_of_ones;

    if number_of_ones > number_og_zeros {
        return '1';
    } else {
        return '0';
    }
}
fn least_common_by_position(arr: &Vec<&str>, position: usize) -> char {
    let length_of_dataset = arr.len();

    let number_of_ones = count_ones(arr, position);
    let number_og_zeros = length_of_dataset - number_of_ones;

    if number_of_ones > number_og_zeros {
        return '0';
    } else {
        return '1';
    }
}

fn count_ones(arr: &Vec<&str>, position: usize) -> usize {
    let number_of_ones = arr
        .iter()
        .filter(|s| !s.is_empty())
        .map(|a| a.chars().nth(position).unwrap())
        .filter(|a| *a == '0')
        .count();
    number_of_ones
}
