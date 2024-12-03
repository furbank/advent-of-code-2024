use regex::Regex;

fn main() {
    let input = include_str!("../input");
    dbg!(part2(input));
}

fn part2(input:&str) -> u32 {
    let regex = Regex::new(r"(?m)do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let result: Vec<&str> = regex.find_iter(input).map(|m| m.as_str()).collect();
    let mut total: u32 = 0;
    let mut ex: bool = true;

    for opp in result{
        match opp.split_at(3).0 {
            "mul" => {
                if ex {
                    let num: Vec<u32> = opp.replace("mul(", "")
                                        .replace(")", "")
                                        .split(",")
                                        .map(|a| a.parse::<u32>().unwrap())
                                        .collect();
                    total += num[0] * num[1];
                }
            },
            "don" => ex = false,
            "do(" => ex = true,
            _ => {}
        }
    }
    total
}
