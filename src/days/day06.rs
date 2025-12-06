use anyhow::Result;

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}

fn parse_numbers(input: &str, part2: bool) -> i64 {
    // the string is a grid of numbers, last row is the operator which we need to apply to each column
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines
        .first()
        .unwrap()
        .split(" ")
        .filter(|x5| !x5.is_empty())
        .count();
    let mut result: i64 = 0;
    let mut splits = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_index, char)| char != &' ')
        .map(|(index, _char)| index)
        .collect::<Vec<usize>>();
    // add an extra split at the end of the line
    splits.push(lines.iter().map(|line| line.len()).max().unwrap());
    for x in 0..width {
        // the indices for the split come from the last column where the operator is not empty string
        let filtered_lines = lines
            .iter()
            .map(|line| {
                if splits[x + 1] >= line.len() {
                    return &line[splits[x]..];
                }
                &line[splits[x]..splits[x + 1]]
            })
            .collect::<Vec<&str>>();
        let number_lines = filtered_lines
            .iter()
            .take(filtered_lines.len() - 1)
            .collect::<Vec<&&str>>();
        let mut column_numbers: Vec<i64> = vec![];
        if part2 {
            // we need to read the numbers column-wise
            // take the rightmost number of each filtered line except the last line,
            // then the second rightmost, etc.
            let lines_width = number_lines.iter().map(|x4| x4.len()).max().unwrap_or(0);
            for pos in 0..lines_width {
                let res = number_lines
                    .iter()
                    .flat_map(|c| {
                        if c.len() > pos {
                            let d = c.chars().nth(pos).unwrap();
                            if d == ' ' {
                                return None;
                            }
                            Some(d)
                        } else {
                            None
                        }
                    })
                    .collect::<String>();
                if res.is_empty() {
                    continue;
                }
                column_numbers.push(res.parse().unwrap());
            }
        } else {
            number_lines.into_iter().for_each(|c| {
                let d = c.trim().parse().unwrap_or(0);
                column_numbers.push(d as i64);
            });
        }
        if let Some(op) = lines[height - 1]
            .split(" ")
            .filter(|x2| !x2.is_empty())
            .nth(x)
        {
            let column_result = match op {
                "+" => column_numbers.iter().sum(),
                "*" => column_numbers.iter().product(),
                _ => 0,
            };
            result += column_result;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 4277556;
    const INPUT_ANSWER_PART1: i64 = 4076006202939;
    const EXAMPLE_ANSWER_PART2: i64 = 3263827;
    const INPUT_ANSWER_PART2: i64 = 7903168391557;

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", "day06", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn input_part1() {
        let path = repo_path(&["inputs", "day06", "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART1);
    }

    #[test]
    fn example_part2() {
        let path = repo_path(&["inputs", "day06", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART2);
    }

    #[test]
    fn input_part2() {
        let path = repo_path(&["inputs", "day06", "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART2);
    }
}
