use anyhow::Result;
use std::collections::HashMap;

pub fn part1(input: &str, example: bool) -> Result<i64> {
    Ok(parse_numbers(input, false, example))
}

pub fn part2(input: &str, example: bool) -> Result<i64> {
    Ok(parse_numbers(input, true, example))
}

fn parse_numbers(input: &str, part2: bool, example: bool) -> i64 {
    // parse the list of 3-d coordinates
    let points: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .trim()
                .split(',')
                .map(|s| s.parse().unwrap_or(0))
                .collect();
            (coords[0], coords[1], coords[2])
        })
        .collect();
    // now we search for the euclidean distance between all points, to find the closest pair
    let mut distances: Vec<((i64, i64, i64), (i64, i64, i64), i64)> = vec![];
    for (i, point_a) in points.iter().enumerate() {
        for (j, point_b) in points.iter().enumerate() {
            if i >= j {
                continue;
            }
            let dist = ((point_a.0 - point_b.0).pow(2)
                + (point_a.1 - point_b.1).pow(2)
                + (point_a.2 - point_b.2).pow(2))
                .isqrt();
            distances.push((*point_a, *point_b, dist));
        }
    }
    // sort distances by distance
    distances.sort_by(|a, b| a.2.cmp(&b.2));

    let mut point_to_group: HashMap<(i64, i64, i64), usize> = HashMap::new();
    let mut groups: HashMap<usize, Vec<(i64, i64, i64)>> = HashMap::new();
    let mut next_group_id = 0;

    let iterations = if example { 10 } else { 1000 };
    let mut iter = 0;

    for (min_point_a, min_point_b, _) in distances {
        if !part2 && iter >= iterations {
            break;
        }
        iter += 1;

        let a_group = point_to_group.get(&min_point_a).copied();
        let b_group = point_to_group.get(&min_point_b).copied();

        match (a_group, b_group) {
            (None, None) => {
                let id = next_group_id;
                next_group_id += 1;
                point_to_group.insert(min_point_a, id);
                point_to_group.insert(min_point_b, id);
                groups.insert(id, vec![min_point_a, min_point_b]);
            }
            (Some(id), None) => {
                point_to_group.insert(min_point_b, id);
                groups.get_mut(&id).unwrap().push(min_point_b);
            }
            (None, Some(id)) => {
                point_to_group.insert(min_point_a, id);
                groups.get_mut(&id).unwrap().push(min_point_a);
            }
            (Some(id_a), Some(id_b)) if id_a != id_b => {
                // Merge groups: move all from b to a
                if let Some(members_b) = groups.remove(&id_b) {
                    let group_a_vec = groups.get_mut(&id_a).unwrap();
                    for point in members_b {
                        point_to_group.insert(point, id_a);
                        group_a_vec.push(point);
                    }
                }
            }
            _ => {} // Already in the same group
        }

        if part2 {
            // If we have only one group left and it contains all points, we are done
            if groups.len() == 1 && groups.values().next().unwrap().len() == points.len() {
                return min_point_a.0 * min_point_b.0;
            }
        }
    }
    // find the 3 largest groups
    let mut result_groups: Vec<&Vec<(i64, i64, i64)>> = groups.values().collect();
    result_groups.sort_by(|a, b| b.len().cmp(&a.len()));
    let largest_groups = &result_groups[..3.min(result_groups.len())];
    let mut result: i64 = 1;
    for group in largest_groups {
        result *= group.len() as i64;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 40;
    const INPUT_ANSWER_PART1: i64 = 153328;
    const EXAMPLE_ANSWER_PART2: i64 = 25272;
    const INPUT_ANSWER_PART2: i64 = 6095621910;

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", "day08", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input, true).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn input_part1() {
        let path = repo_path(&["inputs", "day08", "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input, false).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART1);
    }

    #[test]
    fn example_part2() {
        let path = repo_path(&["inputs", "day08", "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input, true).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART2);
    }

    #[test]
    fn input_part2() {
        let path = repo_path(&["inputs", "day08", "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part2(&input, false).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART2);
    }
}
