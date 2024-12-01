# Advent of Code 2024
Dealing with how little rust I remember from last year by struggling through more problems this year.

## Handy Links
[Advent of code 2024](https://adventofcode.com/2024)

[How to set up Rust for Advent of Code](https://www.youtube.com/watch?v=fEQv-cqzbPg)

[GitIgnore for Rust](https://github.com/rust-lang/cargo/blob/master/.gitignore)

## Starting Boilerplate

```rust
#![allow(unused)]

fn main() {
    let input = include_str!("../input");
    let output = part1(input);
    println!("{:?}", &input);
    dbg!(output);
}

fn part1(_input:&str) -> String {
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
```