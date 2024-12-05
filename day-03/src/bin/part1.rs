#![allow(unused)]
use regex::Regex;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> u32 {
    let regex = Regex::new(r"(?m)mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();

    let result: Vec<(u32, u32)> = regex
        .captures_iter(input)
        .map(|c| {
            let a: u32 = c.name("a").unwrap().as_str().parse::<u32>().ok().unwrap();
            let b: u32 = c.name("b").unwrap().as_str().parse::<u32>().ok().unwrap();
            (a, b)
        })
        .collect();

    result
        .into_iter()
        .map(|(a, b)| a * b)
        .sum::<u32>()
}
