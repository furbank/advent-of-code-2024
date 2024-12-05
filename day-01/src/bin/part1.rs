#![allow(unused)]
use std::{ iter::zip, str::FromStr };

fn main() {
    let input = include_str!("../input");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut total: u32 = 0;
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    for line in &lines {
        let row: Vec<u32> = line
            .split_whitespace()
            .take(2)
            .filter_map(|number| u32::from_str(number).ok())
            .collect();

        left_list.push(row[0]);
        right_list.push(row[1]);
    }

    left_list.sort();
    right_list.sort();
    let total: u32 = zip(left_list, right_list)
        .map(|(l, r)| dist(l, r))
        .sum();
    total
}

fn dist(a: u32, b: u32) -> u32 {
    if a == b { 0 } else if a > b { a - b } else { b - a }
}
