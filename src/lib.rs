use std::fs;
use std::path::{Path, PathBuf};

/// Read input for a given day as a String.
/// It tries the following paths (first existing wins):
/// - inputs/dayXX/input.txt
/// - input/dayXX.txt
/// - inputs/dayXX.txt
pub fn read_input(day: u8) -> anyhow::Result<String> {
    let candidates = [
        format!("inputs/day{day:02}/input.txt"),
        format!("input/day{day:02}.txt"),
        format!("inputs/day{day:02}.txt"),
    ];

    for c in candidates {
        let p = Path::new(&c);
        if p.exists() {
            return Ok(fs::read_to_string(p)?);
        }
    }

    anyhow::bail!("No input file found for day {day:02}")
}

/// Convenience: split input into non-empty trimmed lines
pub fn lines(input: &str) -> impl Iterator<Item=&str> {
    input.lines().map(|l| l.trim()).filter(|l| !l.is_empty())
}

/// Helper to form a path inside the repository (useful for tests/fixtures)
pub fn repo_path(parts: &[&str]) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for part in parts {
        p.push(part);
    }
    p
}

pub mod days;
