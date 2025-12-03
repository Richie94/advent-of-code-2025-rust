use anyhow::Result;
use aoc2025::days::day03;
use aoc2025::read_input;

fn main() -> Result<()> {
    let input = read_input(3)?;
    let p1 = day03::part1(&input)?;
    let p2 = day03::part2(&input)?;
    println!("Day 03\n  Part 1: {p1}\n  Part 2: {p2}");
    Ok(())
}