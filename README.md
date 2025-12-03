# Advent of Code 2025 — Rust Starter Template

This is a lightweight template geared towards beginners. Each day has:

- A library module with two functions: `part1(&str) -> anyhow::Result<T>` and `part2(&str) -> anyhow::Result<T>`
- Unit tests using the included example input(s)
- A small binary you can run to print answers for your real input

Requirements

- Rust (stable). Install with rustup if you don’t have it.

Layout
.
├── Cargo.toml
├── src
│ ├── lib.rs # shared helpers + days module
│ ├── days
│ │ ├── mod.rs
│ │ └── day01.rs # example/template day implementation + tests
│ └── bin
│ └── day01.rs # binary runner for day 01
└── inputs
└── day01
├── example.txt # used by unit tests
└── input.txt # your real puzzle input

Running tests

- Run all tests: cargo test
- Run only Day 01 tests: cargo test day01

Running a day’s binary

- Put your puzzle input in inputs/day01/input.txt
- Run: cargo run --bin day01

Adding a new day

1. Create a new module file src/days/dayXX.rs (copy day01.rs as a starting point)
    - Implement `pub fn part1(input: &str) -> anyhow::Result<...>`
    - Implement `pub fn part2(input: &str) -> anyhow::Result<...>`
    - Keep or add unit tests that read from inputs/dayXX/example.txt
2. Register it in src/days/mod.rs with `pub mod dayXX;`
3. Add a binary at src/bin/dayXX.rs:
   use anyhow::Result;
   use aoc2025::read_input;
   use aoc2025::days::dayXX;

   fn main() -> Result<()> {
   let input = read_input(XX)?; // replace XX with the day number
   println!("Part 1: {}", dayXX::part1(&input)?);
   println!("Part 2: {}", dayXX::part2(&input)?);
   Ok(())
   }
4. Add inputs:
    - inputs/dayXX/example.txt for tests
    - inputs/dayXX/input.txt for the real input

Tips for learning

- Keep parsing separate from logic to simplify testing.
- Start with &str input functions, return anyhow::Result<T> so you can use ? for error handling.
- Add small unit tests for sub-functions if a day becomes more complex.

Happy hacking!