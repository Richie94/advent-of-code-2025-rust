use std::io;
use std::io::BufRead;

fn main(input: &str) -> io::Result<i64> {
    let mut answer: i64 = 0;
    let shape_area: i64 = 9; // matches your Python `presents*9`

    for line in input.lines() {
        if !line.contains('x') {
            continue;
        }

        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        // parts[0] is like "12x5:" -> remove trailing ':'
        let size_part = parts[0].trim_end_matches(':');
        let mut it = size_part.split('x');
        let w: i64 = it.next().unwrap().parse().unwrap();
        let h: i64 = it.next().unwrap().parse().unwrap();

        let area = w * h;

        let presents: i64 = parts[1..]
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .sum();

        if area >= presents * shape_area {
            answer += 1;
        }
    }

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 2;
    const INPUT_ANSWER_PART1: i64 = 487;
    const DAY: &str = "day12";

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", DAY, "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = main(&input).unwrap();
        println!("ans = {}", ans);
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn input_part1() {
        let path = repo_path(&["inputs", DAY, "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = main(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART1);
    }
}
