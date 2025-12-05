use anyhow::Result;

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}

fn parse_numbers(input: &str, part2: bool) -> i64 {
    // split in two parts separated by a blank line
    let parts: Vec<&str> = input.split("\n\n").collect();
    // first part: ranges
    let ranges: Vec<(i64, i64)> = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<&str> = line.trim().split("-").collect();
            if nums.len() != 2 {
                return (0, 0);
            }
            let start: i64 = nums[0].parse().unwrap_or(0);
            let end: i64 = nums[1].parse().unwrap_or(0);
            (start, end)
        })
        .collect();

    if part2 {
        // we want the count of numbers in any of the ranges
        // we merge overlapping ranges first
        let mut merged_ranges: Vec<(i64, i64)> = vec![];
        let mut sorted_ranges = ranges.clone();
        sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));
        for (start, end) in sorted_ranges {
            if let Some(last) = merged_ranges.last_mut() {
                if start <= last.1 {
                    // overlap, merge
                    if end > last.1 {
                        last.1 = end;
                    }
                } else {
                    merged_ranges.push((start, end));
                }
            } else {
                merged_ranges.push((start, end));
            }
        }
        // now count numbers in merged ranges
        let mut count = 0;
        for (start, end) in merged_ranges {
            count += end - start + 1;
        }
        return count;
    }

    // second part: numbers
    let numbers: Vec<&str> = parts[1].lines().collect();

    numbers
        .iter()
        .filter(|x| {
            let num: i64 = x.trim().parse().unwrap_or(-1);
            for (start, end) in &ranges {
                if num >= *start && num <= *end {
                    return true;
                }
            }
            false
        })
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 3;
    const INPUT_ANSWER_PART1: i64 = 607;
    const EXAMPLE_ANSWER_PART2: i64 = 14;
    const INPUT_ANSWER_PART2: i64 = 342433357244012;

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", "day05", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn input_part1() {
        let path = repo_path(&["inputs", "day05", "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART1);
    }

    #[test]
    fn example_part2() {
        let path = repo_path(&["inputs", "day05", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART2);
    }

    #[test]
    fn input_part2() {
        let path = repo_path(&["inputs", "day05", "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART2);
    }
}
