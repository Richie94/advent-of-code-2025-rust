use anyhow::Result;

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}

fn parse_numbers(input: &str, part2: bool) -> i64 {
    // parse the list of 2-d coordinates
    let red_points: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .trim()
                .split(',')
                .map(|s| s.parse().unwrap_or(0))
                .collect();
            (coords[0], coords[1])
        })
        .collect();
    // now we search for the max area of two points
    let mut max_area = 0;
    let num_points = red_points.len();
    for (i, point_a) in red_points.iter().enumerate() {
        for (j, point_b) in red_points.iter().enumerate() {
            if i >= j {
                continue;
            }

            if part2 {
                let min_x = point_a.0.min(point_b.0);
                let max_x = point_a.0.max(point_b.0);
                let min_y = point_a.1.min(point_b.1);
                let max_y = point_a.1.max(point_b.1);

                let mut crossing = false;
                for k in 0..num_points {
                    let p1 = red_points[k];
                    let p2 = red_points[(k + 1) % num_points];
                    if intersects_rect(p1, p2, min_x, max_x, min_y, max_y) {
                        crossing = true;
                        break;
                    }
                }
                if crossing {
                    continue;
                }
            }

            let dist = ((point_a.0 - point_b.0).abs() + 1) * ((point_a.1 - point_b.1).abs() + 1);
            if dist > max_area {
                max_area = dist;
            }
        }
    }
    max_area
}

fn intersects_rect(p1: (i64, i64), p2: (i64, i64), min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> bool {
    // Check if the segment p1-p2 strictly intersects the interior of the rectangle
    // We look for a parameter t in (0, 1) such that p(t) is strictly inside (min_x, max_x) x (min_y, max_y)

    let get_interval = |p_start: i64, p_end: i64, min_val: i64, max_val: i64| -> Option<(f64, f64)> {
        let d = (p_end - p_start) as f64;
        if d == 0.0 {
            if p_start > min_val && p_start < max_val {
                Some((-f64::INFINITY, f64::INFINITY))
            } else {
                None
            }
        } else {
            let t1 = (min_val - p_start) as f64 / d;
            let t2 = (max_val - p_start) as f64 / d;
            Some((t1.min(t2), t1.max(t2)))
        }
    };

    let x_interval = get_interval(p1.0, p2.0, min_x, max_x);
    let y_interval = get_interval(p1.1, p2.1, min_y, max_y);

    match (x_interval, y_interval) {
        (Some((t0_x, t1_x)), Some((t0_y, t1_y))) => {
            let start = 0.0f64.max(t0_x).max(t0_y);
            let end = 1.0f64.min(t1_x).min(t1_y);
            // If the intersection interval is valid and non-empty (start < end), we have a crossing
            start < end
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 50;
    const INPUT_ANSWER_PART1: i64 = 4741848414;
    const EXAMPLE_ANSWER_PART2: i64 = 24;
    const INPUT_ANSWER_PART2: i64 = 6095621910;
    const DAY: &str = "day09";

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", DAY, "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn input_part1() {
        let path = repo_path(&["inputs", DAY, "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART1);
    }

    #[test]
    fn example_part2() {
        let path = repo_path(&["inputs", DAY, "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART2);
    }

    #[test]
    fn input_part2() {
        let path = repo_path(&["inputs", DAY, "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART2);
    }
}
