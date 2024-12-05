#![allow(unused)]

use std::str::FromStr;

fn main() {
    let input: &str = include_str!("../input");
    let match_word: &str = "XMAS";
    let output = part2(input, match_word);
    dbg!(output);
}

fn part2(input: &str, match_word: &str) -> String {
    let mut total: u32 = 0;
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let max_x: usize = lines[0].len();
    let max_y: usize = lines.len();

    for y in 1..max_y - 1 {
        for x in 1..max_x - 1 {
            if lines[y][x] == 'A' {
                if
                    (lines[y - 1][x - 1] as u8) + (lines[y + 1][x + 1] as u8) == 160 &&
                    (lines[y - 1][x + 1] as u8) + (lines[y + 1][x - 1] as u8) == 160
                {
                    total += 1;
                }
            }
        }
    }
    total.to_string()
}
