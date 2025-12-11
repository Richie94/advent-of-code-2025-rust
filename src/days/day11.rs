use anyhow::Result;
use std::collections::HashMap;

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}

fn parse_numbers(input: &str, part2: bool) -> i64 {
    // each line is a node with outgoing edges to other nodes
    let mut result = 0;
    let edges: HashMap<String, Vec<String>> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let key = parts[0].trim().to_string();

            let values = parts
                .get(1)
                .unwrap_or(&"")
                .trim()
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect();
            (key, values)
        })
        .collect();

    if !part2 {
        // start a bfs from node "you" to node "out" and find the number of possible paths
        let start_node = "you".to_string();
        let target_node = "out".to_string();
        let mut queue: Vec<(String, Vec<String>)> =
            vec![(start_node.clone(), vec![start_node.clone()])];
        while let Some((current_node, path)) = queue.pop() {
            if current_node == target_node {
                result += 1;
                continue;
            }

            if let Some(neighbors) = edges.get(&current_node) {
                for neighbor in neighbors {
                    if !path.contains(neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor.clone());
                        queue.push((neighbor.clone(), new_path));
                    }
                }
            }
        }
    } else {
        // Recursive function with memoization to count paths between two nodes
        fn count_paths(
            curr: &str,
            target: &str,
            edges: &HashMap<String, Vec<String>>,
            memo: &mut HashMap<String, i64>,
        ) -> i64 {
            if curr == target {
                return 1;
            }
            if let Some(&count) = memo.get(curr) {
                return count;
            }

            let mut total = 0;
            if let Some(neighbors) = edges.get(curr) {
                for neighbor in neighbors {
                    total += count_paths(neighbor, target, edges, memo);
                }
            }

            memo.insert(curr.to_string(), total);
            total
        }

        let get_segment_count = |start: &str, end: &str| -> i64 {
            let mut memo = HashMap::new();
            count_paths(start, end, &edges, &mut memo)
        };

        let start = "svr";
        let target = "out";
        let mid1 = "fft";
        let mid2 = "dac";

        // Case 1: svr -> fft -> dac -> out
        let leg1 = get_segment_count(start, mid1);
        let leg2 = get_segment_count(mid1, mid2);
        let leg3 = get_segment_count(mid2, target);
        result += leg1 * leg2 * leg3;

        // Case 2: svr -> dac -> fft -> out
        let leg1 = get_segment_count(start, mid2);
        let leg2 = get_segment_count(mid2, mid1);
        let leg3 = get_segment_count(mid1, target);
        result += leg1 * leg2 * leg3;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 5;
    const INPUT_ANSWER_PART1: i64 = 791;
    const EXAMPLE_ANSWER_PART2: i64 = 2;
    const INPUT_ANSWER_PART2: i64 = 520476725037672;
    const DAY: &str = "day11";

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
        let path = repo_path(&["inputs", DAY, "example2.txt"]);
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
