use anyhow::Result;
use aoc2025::days::day01;
use aoc2025::read_input;

fn main() -> Result<()> {
    let input = read_input(1)?;
    let p1 = day01::part1(&input)?;
    let p2 = day01::part2(&input)?;
    println!("Day 01\n  Part 1: {p1}\n  Part 2: {p2}");
    Ok(())
}
