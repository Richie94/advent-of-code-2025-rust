use anyhow::Result;

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}

fn parse_numbers(input: &str, part2: bool) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().map(|l| l.len()).unwrap_or(0);

    // Use a flat Vec<bool>
    // true = '@', false = '.' or removed.
    let mut grid = vec![false; width * height];

    for (y, line) in lines.iter().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'@' {
                grid[y * width + x] = true;
            }
        }
    }

    let mut total_removed = 0;
    // Reusable buffer to store indices to remove in the current step
    let mut to_remove = Vec::with_capacity(128);

    loop {
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;

                // Skip if empty or already removed
                if !grid[idx] {
                    continue;
                }

                let mut adjacent = 0;

                // Manual neighbor check with boundary guards
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }

                        let ny = y as isize + dy;
                        let nx = x as isize + dx;

                        if ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize {
                            if grid[ny as usize * width + nx as usize] {
                                adjacent += 1;
                            }
                        }
                    }
                }

                if adjacent < 4 {
                    to_remove.push(idx);
                }
            }
        }

        if to_remove.is_empty() {
            return total_removed;
        }

        total_removed += to_remove.len() as i64;

        // Apply changes
        for &idx in &to_remove {
            grid[idx] = false;
        }

        // Clear buffer for next iteration without deallocating memory
        to_remove.clear();

        if !part2 {
            return total_removed;
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
