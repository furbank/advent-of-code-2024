use std::str::FromStr;

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
        let row: Vec<u32>= line.split_whitespace().take(2).filter_map(|number| u32::from_str(number).ok()).collect();

        left_list.push(row[0]);
        right_list.push(row[1]);
    }

    for l in left_list{
        total += (TryInto::<u32>::try_into(right_list.iter().filter(|&n| *n == l).count())).unwrap() * l;
    }
    total
}
