use anyhow::Result;

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}


fn parse_numbers(input: &str, part2: bool) -> i64 {
    let mut zeros = 0;
    let mut current = 50;
    for acc in input.lines() {
        // starts with 50, if the string starts with "R", add the number after it modulo 99
        // if its left with "L", subtract the number after it modulo 99
        println!("Current: {} Acc: {} Zero: {}", current, acc, zeros);
        current = acc.trim().chars().next().map(|c| {
            let num: i64 = acc.trim()[1..].parse().unwrap_or(0);
            let mod_num = num % 100;
            let amount = (num / 100).abs();
            match c {
                'R' => {
                    let next = current + mod_num;
                    if part2 {
                        zeros = zeros + amount;
                        if current > 0 && next > 100 {
                            zeros = zeros + 1;
                        }
                    }
                    next % 100
                }
                'L' => {
                    let next = current - mod_num;
                    if part2 {
                        zeros = zeros + amount;
                        if current > 0 && next < 0 {
                            zeros = zeros + 1;
                        }
                    }
                    (next + 1000000) % 100
                }
                _ => current,
            }
        }).unwrap_or(current);
        if current == 0 {
            zeros = zeros + 1;
        }
    }
    zeros
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 3;
    const EXAMPLE_ANSWER_PART2: i64 = 6;
    const EXAMPLE_ANSWER_PART2B: i64 = 20;

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", "day01", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn example_part2() {
        let path = repo_path(&["inputs", "day01", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART2);
    }

    #[test]
    fn example_part2b() {
        let path = repo_path(&["inputs", "day01", "example2.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART2B);
    }
}
