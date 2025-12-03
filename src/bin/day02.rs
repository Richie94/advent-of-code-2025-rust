use anyhow::Result;
use aoc2025::days::day02;
use aoc2025::read_input;

fn main() -> Result<()> {
    let input = read_input(2)?;
    let p1 = day02::part1(&input)?;
    let p2 = day02::part2(&input)?;
    println!("Day 02\n  Part 1: {p1}\n  Part 2: {p2}");
    Ok(())
}