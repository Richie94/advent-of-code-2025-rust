use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}

fn parse_numbers(input: &str, part2: bool) -> i64 {
    // we have a 2d grid of . for empty and @ for filled
    // we count for every @ if there are more than 4 adjacent @
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut new_removable: HashSet<(isize, isize)> = HashSet::new();
    // use a hashmap to store removable positions
    let mut removable: HashSet<(isize, isize)> = HashSet::new();
    let mut grid: HashMap<(isize, isize), char> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let value = lines[y].chars().nth(x).unwrap_or('.');
            if value == '.' {
                continue;
            }
            grid.insert((x as isize, y as isize), value);
        }
    }
    loop {
        for entry in grid.iter() {
            let (x, y) = entry.0;
            if removable.contains(&entry.0) {
                continue;
            }
            let mut adjacent = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let ny = y + dy;
                    let nx = x + dx;
                    let next_key = (nx, ny);

                    if removable.contains(&next_key) {
                        continue;
                    }

                    if grid.get(&next_key).unwrap_or(&'.') == &'@' {
                        adjacent += 1;
                    }
                }
            }
            if adjacent < 4 {
                new_removable.insert(*entry.0);
            }
        }

        println!("New removable: {:?}, total removable {}", new_removable.len(), removable.len() + new_removable.len());

        if new_removable.is_empty() {
            return removable.len() as i64;
        }
        new_removable.iter().for_each(|k| {
            removable.insert(*k);
        });
        new_removable.clear();

        if !part2 {
            return removable.len() as i64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 13;
    const INPUT_ANSWER_PART1: i64 = 1491;
    const EXAMPLE_ANSWER_PART2: i64 = 43;
    const INPUT_ANSWER_PART2: i64 = 8722;

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", "day04", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn input_part1() {
        let path = repo_path(&["inputs", "day04", "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART1);
    }

    #[test]
    fn example_part2() {
        let path = repo_path(&["inputs", "day04", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART2);
    }

    #[test]
    fn input_part2() {
        let path = repo_path(&["inputs", "day04", "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART2);
    }
}
