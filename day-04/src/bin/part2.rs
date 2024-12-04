#![allow(unused)]

use std::str::FromStr;

fn main() {
    let input: &str = include_str!("../input");
    let match_word: &str = "XMAS";
    let output = part2(input, match_word);
    // println!("{:?}", &input);
    dbg!(output);
}

fn part2(input:&str, match_word: &str) -> String {
    let mut total: u32 = 0;
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();

    // for l in &lines{
    //     println!("{:?}", l.iter().map(|c| c.to_string()).collect::<String>());
    // }

    let max_x: usize = lines[0].len();
    let max_y: usize = lines.len();

    for y in 0..max_y {
        // println!("{:?}", lines[y]);
        for x in 0..max_x {
            // println!("{:?}, x:{}, y:{}", lines[y][x], x ,y);
            if lines[y][x] == match_word.chars().nth(0).unwrap()
            {
                //println!("{}, {}, {} {}", x, y, lines[y][x], find_around(match_word, x, y, &lines, max_x, max_y));
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
        // println!("1 ");
        // println!("{}", lines[y-1][x+1]);
        if match_word == [lines[y][x], lines[y-1][x+1], lines[y-2][x+2], lines[y-3][x+3]].iter().collect::<String>() {
            // println!("1, {} {}", x, y);
            found += 1;
        }
    }

    if max_x - x >= match_len && max_y - y >= match_len {
        // println!("3");
        // println!("{}", lines[y+1][x+1]);
        if match_word == [lines[y][x], lines[y+1][x+1], lines[y+2][x+2], lines[y+3][x+3]].iter().collect::<String>() {
            // println!("3, {} {}", x, y);
            found += 1;
        }
    }

    if x >= match_len - 1 && max_y - y >= match_len {
        // println!("5");
        // println!("{}", lines[y+1][x-1]);
        if match_word == [lines[y][x], lines[y+1][x-1], lines[y+2][x-2],lines[y+3][x-3]].iter().collect::<String>() {
            // println!("5, {} {}", x, y);
            found += 1;
        }
    }

    if x >= match_len - 1 && y >= match_len - 1 {
        // println!("7");
        // println!("{}", lines[y-1][x-1]);
        if match_word == [lines[y][x], lines[y-1][x-1],lines[y-2][x-2],lines[y-3][x-3]].iter().collect::<String>() {
            // println!("7, {} {}", x, y);
            found += 1;
        }
    }

    if max_x - x >= match_len {
        // println!("2");
        // println!("{}", lines[y][x+1]);
        if match_word == [lines[y][x], lines[y][x+1], lines[y][x+2], lines[y][x+3]].iter().collect::<String>() {
            // println!("2, {} {}", x, y);
            found += 1;
        }
    }

    if max_y - y >= match_len {
        // println!("4");
        // println!("{}", lines[y+1][x]);
        if match_word == [lines[y][x], lines[y+1][x], lines[y+2][x], lines[y+3][x]].iter().collect::<String>() {
            // println!("4, {} {}", x, y);
            found += 1;
        }
    }

    if x >= match_len - 1 {
        // println!("6");
        // println!("{}", lines[y][x-1]);
        if match_word == [lines[y][x], lines[y][x-1], lines[y][x-2], lines[y][x-3]].iter().collect::<String>() {
            // println!("6, {} {}", x, y);
            found += 1;
        }
    }

    if y >= match_len - 1 {
        // println!("8");
        // println!("{}", lines[y-1][x]);
        if match_word == [lines[y][x], lines[y-1][x], lines[y-2][x], lines[y-3][x]].iter().collect::<String>() {
            // println!("8, {} {}", x, y);
            found += 1;
        }
    }

//    let t: String = [lines[y-1][x+1],lines[y-2][x+2],lines[y-3][x+3]].iter().collect();
    found
}
