use anyhow::Result;
use std::collections::HashMap;

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}

fn parse_numbers(input: &str, part2: bool) -> i64 {
    let mut splits = 0;
    let mut counts: HashMap<(usize, usize), i64> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                counts.insert((x, y), 1);
            } else if c == '^' && y > 0 {
                if let Some(&count) = counts.get(&(x, y - 1)) {
                    *counts.entry((x - 1, y)).or_insert(0) += count;
                    *counts.entry((x + 1, y)).or_insert(0) += count;
                    if *counts.get(&(x - 1, y)).unwrap() == count
                        || *counts.get(&(x + 1, y)).unwrap() == count
                    {
                        // split happened
                        splits += 1;
                    }
                }
            } else if c == '.' && y > 0 {
                if let Some(&count) = counts.get(&(x, y - 1)) {
                    *counts.entry((x, y)).or_insert(0) += count;
                }
            }
        }
    }
    let max_y = counts.keys().map(|(_, y)| *y).max().unwrap_or(0);

    if !part2 {
        splits
    } else {
        counts
            .into_iter()
            .filter(|&((_, y), _)| y == max_y)
            .map(|(_, count)| count)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 21;
    const INPUT_ANSWER_PART1: i64 = 1535;
    const EXAMPLE_ANSWER_PART2: i64 = 40;
    const INPUT_ANSWER_PART2: i64 = 4404709551015;

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", "day07", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn input_part1() {
        let path = repo_path(&["inputs", "day07", "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART1);
    }

    #[test]
    fn example_part2() {
        let path = repo_path(&["inputs", "day07", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART2);
    }

    #[test]
    fn input_part2() {
        let path = repo_path(&["inputs", "day07", "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART2);
    }
}
