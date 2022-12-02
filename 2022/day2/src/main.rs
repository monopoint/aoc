#[allow(unused)]
use std::fs;
use core::str::Lines;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Shape {
    Rock     = 1,
    Paper    = 2,
    Scissors = 3
}


fn main() {
    let file = fs::read_to_string("./input").expect("no file");
    let input = file.lines();

    // Part 1
    println!("Part one score is {}", part1(input.clone()));

    // Part 2
    println!("Part two score is {}", part2(input));
}

// Part 1
fn part1(input: Lines) -> i32 {
    let score: i32 = input
    .map(|round| match round.split_once(" ") {
            Some((first, second)) => [input_to_shape(first), input_to_shape(second)],
            None => panic!("Bad data"),
    })
    .map(|parsed_round| score(&parsed_round[0], &parsed_round[1]))
    .sum();
    score
}

fn part2(input: Lines) -> i32 {
    let score: i32 = input.map(|round| {
        let split_round: Vec<&str> = round.split(" ").collect();
        let opponent_shape = input_to_shape(&split_round.get(0).unwrap());
        let my_shape = match split_round.get(1).unwrap() {
            &"X" => get_loser(opponent_shape),
            &"Y" => opponent_shape,
            _ => get_winner(opponent_shape) 
        };
        [opponent_shape, my_shape]
    }).map(|shapes| {
        let score = score(&shapes[0], &shapes[1]);
        score
    }).sum();
    score
}



fn get_winner(shape: Shape) -> Shape {
    match shape {
        Shape::Paper => Shape::Scissors,
        Shape::Rock => Shape::Paper,
        _ => Shape::Rock
    }
}

fn get_loser(shape: Shape) -> Shape {
    match shape {
        Shape::Paper => Shape::Rock,
        Shape::Rock => Shape::Scissors,
        _ => Shape::Paper
    }
}

fn score(opponent_shape: &Shape, my_shape: &Shape) -> i32 {
    let winner = winner(&opponent_shape, &my_shape);
    let shape_score = *my_shape as i32;

    if winner == None {
        3 + shape_score
    } else {
        if &winner.unwrap() == my_shape {
            shape_score + 6
        } else {
            shape_score + 0
        }
    }
}

fn winner(shape1: &Shape, shape2: &Shape) -> Option<Shape> {
    if shape1 == shape2 {
        None
    } else {
        let shapes = [shape1, shape2];
        if shapes.contains(&&Shape::Paper) && shapes.contains(&&Shape::Rock) {
            Some(Shape::Paper)
        } else if shapes.contains(&&Shape::Paper) && shapes.contains(&&Shape::Scissors) {
            Some(Shape::Scissors)
        } else {
            Some(Shape::Rock)
        }   
    }
}

fn input_to_shape(input: &str) -> Shape{
    match input {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        _ => Shape::Scissors
    }
}
