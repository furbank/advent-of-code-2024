use std::str::FromStr;
use std::iter::zip;

fn main() {
    let input = include_str!("../input");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut total_safe: u32 = 0;

    let rows: Vec<Vec<i32>> = input
        .lines()
        .map(|l: &str|
            l
                .split_whitespace()
                .filter_map(|n| i32::from_str(n).ok())
                .collect::<Vec<i32>>()
        )
        .collect();

    for report in rows {
        if is_safe(&report) {
            total_safe += 1;
        }
    }

    total_safe.to_string()
}

fn is_safe(report: &Vec<i32>) -> bool {
    // Rules
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.
    let level_diffs: Vec<i32> = find_diffs(&report);

    for change in &level_diffs {
        match &level_diffs[0] {
            1..=3 => {
                match change {
                    1..=3 => {}
                    _ => {
                        return false;
                    }
                }
            }
            -3..=-1 => {
                match change {
                    -3..=-1 => {}
                    _ => {
                        return false;
                    }
                }
            }
            _ => {
                return false;
            }
        }
    }
    true
}

fn find_diffs(report: &Vec<i32>) -> Vec<i32> {
    zip(&report[..report.len() - 1], &report[1..])
        .into_iter()
        .map(|(a, b)| a - b)
        .collect::<Vec<i32>>()
}
