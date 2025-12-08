use anyhow::Result;

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

    let mut groups: Vec<Vec<(i64, i64, i64)>> = vec![];
    let iterations = if example { 10 } else { 1000 };
    // check for the minimal distance in distances
    let mut iter = 0;
    loop {
        if !part2 && iter >= iterations {
            break;
        }
        iter += 1;
        let (min_point_a, min_point_b, _min_distance) = distances[0];

        // remove this distance from the list
        distances.remove(0);

        // check if point_a is already in a group
        let mut contained_in_groups: Vec<usize> = vec![];
        for (index, group) in groups.iter().enumerate() {
            if group.contains(&min_point_a) || group.contains(&min_point_b) {
                contained_in_groups.push(index);
            }
        }
        if contained_in_groups.len() == 0 {
            // create a new group
            groups.push(vec![min_point_a, min_point_b]);
        } else if contained_in_groups.len() == 1 {
            // add both points to the existing group
            let group_index = contained_in_groups[0];
            if !groups[group_index].contains(&min_point_a) {
                groups[group_index].push(min_point_a);
            }
            if !groups[group_index].contains(&min_point_b) {
                groups[group_index].push(min_point_b);
            }
        } else {
            // merge all groups
            let mut new_group: Vec<(i64, i64, i64)> = vec![];
            for group_index in contained_in_groups.iter() {
                let group = &groups[*group_index];
                for point in group.iter() {
                    if !new_group.contains(point) {
                        new_group.push(*point);
                    }
                }
                // remove the old group
            }
            for group_index in contained_in_groups.iter().rev() {
                groups.remove(*group_index);
            }
            // add the two points
            if !new_group.contains(&min_point_a) {
                new_group.push(min_point_a);
            }
            if !new_group.contains(&min_point_b) {
                new_group.push(min_point_b);
            }
            groups.push(new_group);
        }

        if part2 && groups[0].len() == points.len() {
            return min_point_a.0 * min_point_b.0;
        }
    }

    // find the 3 largest groups
    groups.sort_by(|a, b| b.len().cmp(&a.len()));
    let largest_groups = &groups[..3.min(groups.len())];
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
