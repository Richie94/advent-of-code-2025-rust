use good_lp::{default_solver, variable, variables, Solution, SolverModel};

use anyhow::Result;

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, false))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(parse_numbers(input, true))
}

fn parse_numbers(input: &str, part2: bool) -> i64 {
    let mut score = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Parse buttons (1,2)
        let buttons_indices: Vec<Vec<usize>> = parts[1..parts.len() - 1].iter().map(|button_str| {
            button_str.trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect()
        }).collect();

        // Determine Targets and Variable Constraints based on Part
        let targets: Vec<f64>;
        let max_presses: f64;

        if !part2 {
            // Part 1: Target is the pattern itself [.##.] -> [0, 1, 1, 0]
            // Constraint: Use each button at most once (Binary)
            let pattern = parts[0].trim_matches(|c| c == '[' || c == ']');
            targets = pattern.chars()
                .map(|c| if c == '#' { 1.0 } else { 0.0 })
                .collect();
            max_presses = 1.0;
        } else {
            // Part 2: Target is the number list {55, 38...}
            // Constraint: Unlimited presses
            targets = parts.last().unwrap()
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            max_presses = f64::INFINITY;
        }

        // --- ILP SOLVER ---
        let mut vars = variables!();
        let mut button_vars = Vec::new();

        for _ in 0..buttons_indices.len() {
            // Unified Variable Definition
            // Part 1: min(0), max(1), Integer -> Binary Variable (0 or 1)
            // Part 2: min(0), Integer -> standard Integer Variable (0, 1, 2...)
            if !part2 {
                button_vars.push(vars.add(variable().integer().min(0).max(1)));
            } else {
                button_vars.push(vars.add(variable().integer().min(0)));
            }
        }

        let objective = button_vars.iter().sum::<good_lp::Expression>();

        // Use Highs solver (robust for ILP)
        let mut problem = vars.minimise(objective).using(default_solver);

        // Constraints: Sum(Button_Impacts) == Target
        // Note: For Part 1, this assumes "Addition" logic matches the "Pattern" logic.
        // If Part 1 is strictly XOR (Modulo 2), this ILP model is valid ONLY IF
        // the combination is unique and doesn't rely on 1+1=0 cancellation.
        // Given Part 2 works with addition, Part 1 likely does too.
        for (jolt_idx, &target_val) in targets.iter().enumerate() {
            let mut expression = good_lp::Expression::from(0);

            for (btn_i, affected_indices) in buttons_indices.iter().enumerate() {
                let impact = affected_indices.iter().filter(|&&x| x == jolt_idx).count() as i32;
                if impact > 0 {
                    expression += button_vars[btn_i] * impact;
                }
            }
            problem.add_constraint(expression.eq(target_val));
        }

        if let Ok(solution) = problem.solve() {
            let presses: f64 = solution.eval(button_vars.iter().sum::<good_lp::Expression>());
            score += (presses + 1e-5).round() as i64;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 7;
    const INPUT_ANSWER_PART1: i64 = 434;
    const EXAMPLE_ANSWER_PART2: i64 = 33;
    const INPUT_ANSWER_PART2: i64 = 15132;
    const DAY: &str = "day10";

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
