#![allow(unused)]

use std::str::FromStr;

fn main() {
    let input: &str = include_str!("../input");
    let match_word: &str = "XMAS";
    let output = part1(input, match_word);
    dbg!(output);
}

fn part1(input:&str, match_word: &str) -> String {
    let mut total: u32 = 0;
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();

    let max_x: usize = lines[0].len();
    let max_y: usize = lines.len();

    for y in 0..max_y {
        for x in 0..max_x {
            if lines[y][x] == match_word.chars().nth(0).unwrap()
            {
                total += find_around(match_word, x, y, &lines, max_x, max_y);
            }
        }
    }
    total.to_string()
}

fn find_around( match_word: &str, x: usize, y: usize, lines: &Vec<Vec<char>>, max_x: usize, max_y: usize  ) -> u32 {

    let mut found: u32 = 0;
    let match_len = match_word.chars().count();

    if max_x - x >= match_len && y >= match_len - 1 {
        if match_word == [lines[y][x], lines[y-1][x+1], lines[y-2][x+2], lines[y-3][x+3]].iter().collect::<String>() {
            found += 1;
        }
    }

    if max_x - x >= match_len && max_y - y >= match_len {
        if match_word == [lines[y][x], lines[y+1][x+1], lines[y+2][x+2], lines[y+3][x+3]].iter().collect::<String>() {
            found += 1;
        }
    }

    if x >= match_len - 1 && max_y - y >= match_len {
        if match_word == [lines[y][x], lines[y+1][x-1], lines[y+2][x-2],lines[y+3][x-3]].iter().collect::<String>() {
            found += 1;
        }
    }

    if x >= match_len - 1 && y >= match_len - 1 {
        if match_word == [lines[y][x], lines[y-1][x-1],lines[y-2][x-2],lines[y-3][x-3]].iter().collect::<String>() {
            found += 1;
        }
    }

    if max_x - x >= match_len {
        if match_word == [lines[y][x], lines[y][x+1], lines[y][x+2], lines[y][x+3]].iter().collect::<String>() {
            found += 1;
        }
    }

    if max_y - y >= match_len {
        if match_word == [lines[y][x], lines[y+1][x], lines[y+2][x], lines[y+3][x]].iter().collect::<String>() {
            found += 1;
        }
    }

    if x >= match_len - 1 {
        if match_word == [lines[y][x], lines[y][x-1], lines[y][x-2], lines[y][x-3]].iter().collect::<String>() {
            found += 1;
        }
    }

    if y >= match_len - 1 {
        if match_word == [lines[y][x], lines[y-1][x], lines[y-2][x], lines[y-3][x]].iter().collect::<String>() {
            found += 1;
        }
    }

    found
}
