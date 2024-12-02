use std::str::FromStr;
use std::iter::zip;

fn main() {
    let input = include_str!("../input");
    let output = part1(input);
    dbg!(output);
}

fn part1(input:&str) -> String {

    let mut total_safe: u32 = 0;

    let rows: Vec<Vec<i32>> = input.lines()
        .map(|l: &str| l.split_whitespace()
            .filter_map(|n| (i32::from_str(n).ok()))
            .collect::<Vec<i32>>())
        .collect();

    for report in rows {
        if test_report(&report) == [] { total_safe += 1 }
        else if brute_fix( &report ){ total_safe +=1 }
    }
    total_safe.to_string()
}

fn test_report(report:&Vec<i32>) -> Vec<usize> {
    // Rules
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.
    let level_diffs: Vec<i32> = find_diffs(&report);
    let mut location: usize = 0;
    let mut problem_locations: Vec<usize> =Vec::new();

    for change in &level_diffs {
        match &level_diffs[0] {
            1..=3 => {
                match change {
                    0 => problem_locations.push(location),
                    1..=3 => {},
                    4.. => problem_locations.push(location),
                    _ => problem_locations.push(location)
                    }
            },
            -3..=-1 => {
                match change {
                    0 => problem_locations.push(location),
                    -3..=-1 => {},
                    ..-3 => problem_locations.push(location),
                    _ => problem_locations.push(location)
                }
            },
            _ => problem_locations.push(location)
    }

        location += 1;
    }
    problem_locations
}

fn find_diffs(report: &Vec<i32>) -> Vec<i32> {
    zip( &report[..report.len()-1], &report[1..])
        .into_iter()
        .map(|(a,b)| (a - b))
        .collect::<Vec<i32>>()
}

fn brute_fix(report: &Vec<i32>) -> bool {
    for i in 0..report.len(){
        let try_fix_report: &mut Vec<i32> = &mut report.clone();
        try_fix_report.remove(i);
        if test_report(try_fix_report) == []{ return true };
    }
    false
}