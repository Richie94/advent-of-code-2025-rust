use anyhow::Result;
use aoc2025::days::day08;
use aoc2025::read_input;

fn main() -> Result<()> {
    let input = read_input(8)?;
    let now = std::time::Instant::now();
    let p1 = day08::part1(&input)?;
    let elapsed = now.elapsed();
    let p2 = day08::part2(&input)?;
    let elapsed2 = now.elapsed();
    println!("Day 08\n  Part 1: {p1} in {:?}\n  Part 2: {p2} {:?}", elapsed, elapsed2);
    Ok(())
}