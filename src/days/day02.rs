use anyhow::Result;

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}


fn parse_numbers(input: &str, part2: bool) -> i64 {
    let mut invalid_numbers: Vec<i64> = vec![];
    let text = input.split(",");
    for range in text {
        // two numbers separated by a dash
        let nums: Vec<&str> = range.trim().split("-").collect();
        if nums.len() != 2 {
            continue;
        }
        let start: i64 = nums[0].parse().unwrap_or(0);
        let end: i64 = nums[1].parse().unwrap_or(0);
        // we need to find every number which is twice (part 2: n-times) repeated in this range, e.g. range 11-50 has 11,22,33,44
        for i in start..=end {
            let s = i.to_string();
            // max length could be in part2 the full length of the string of the end number
            let end_check = if part2 { nums[1].to_string().len() } else { 2 };

            for n in 2..=end_check {
                let n_half = s.len() / n;
                let first_half = &s[..n_half];
                if part2 && n_half <= 0 || s.len() % n != 0 {
                    continue;
                }
                for j in 0..n {
                    let part = &s[j * n_half..(j + 1) * n_half];
                    if part != first_half {
                        break;
                    }
                    if j == n - 1 && !invalid_numbers.contains(&i) {
                        invalid_numbers.push(i);
                    }
                }
            }
        }
    }
    println!("Invalid numbers: {:?}", invalid_numbers);
    invalid_numbers.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 1227775554;
    const EXAMPLE_ANSWER_PART2: i64 = 4174379265;

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", "day02", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn example_part2() {
        let path = repo_path(&["inputs", "day02", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART2);
    }
}
