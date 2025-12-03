use anyhow::Result;

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}

fn parse_numbers(input: &str, part2: bool) -> i64 {
    input
        .lines()
        .map(|line| {
            // find the highest digit in the line, but it may not be the last digit
            let lenght = if part2 { 12 } else { 2 };
            let mut start = 0;
            let mut numbers: Vec<u32> = vec![];
            for i in 0..lenght {
                // search the highest digit in line from start to len-i
                let mut max_digit = 0;
                let check_until = line.len() - lenght + i + 1;
                for c in line[start..check_until].chars() {
                    if let Some(d) = c.to_digit(10) {
                        if d > max_digit {
                            max_digit = d;
                        }
                    }
                }
                // find the position of the first occurrence of max_digit
                if let Some(pos) = line[start..check_until].find(&max_digit.to_string()) {
                    start = start + pos + 1;
                    numbers.push(max_digit);
                }
            }
            // join all numbers from numbers vec to a single number
            let mut result = 0;
            for n in numbers {
                result = result * 10 + n as i64;
            }
            println!("Line: {} Max Number: {}", line, result);
            result
        })
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 357;
    const EXAMPLE_ANSWER_PART2: i64 = 3121910778619;

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", "day03", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn example_part2() {
        let path = repo_path(&["inputs", "day03", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART2);
    }
}
