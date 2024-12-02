#![allow(unused)]
use std::str::FromStr;

fn main() {
    let input = include_str!("../input");
    let output = part1(input);
    dbg!(output);

// Rules
// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.

}

fn part1(input:&str) -> String {

    let rows: Vec<Vec<u32>> = input.lines()
        .map(|l: &str| l.split_whitespace()
            .filter_map(|n| (u32::from_str(n).ok()))
            .collect::<Vec<u32>>())
        .collect();

    println!("{:?}", rows);

    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    //use crate::part1;
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("");
        assert_eq!(result, "4".to_string());
    }
}
