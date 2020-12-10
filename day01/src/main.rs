use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

use anyhow::{anyhow, Result};

const INPUT: &str = "input/input.txt";

fn solve_part1(input_path: &str) -> Result<i32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut prev_numbers = HashSet::new();
    for line in reader.lines() {
        let number: i32 = line?.parse()?;
        let other_half = 2020 - number;
        if prev_numbers.contains(&other_half) {
            return Ok(number * other_half);
        }
        prev_numbers.insert(number);
    }

    Err(anyhow!("Found no pair of numbers that sums to 2020"))
}

fn solve_part2(input_path: &str) -> Result<i32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut prev_numbers = HashSet::new();
    for line in reader.lines() {
        let number: i32 = line?.parse()?;
        for prev_number in prev_numbers.iter() {
            let other_third = 2020 - prev_number - number;
            if prev_numbers.contains(&other_third) {
                return Ok(number * prev_number * other_third);
            }
        }
        prev_numbers.insert(number);
    }

    Err(anyhow!("Found no three numbers that sum to 2020"))
}

fn main() {
    let mut now = Instant::now();
    println!("Part 1: {}", solve_part1(INPUT).unwrap());
    println!("(elapsed: {:?})", now.elapsed());
    now = Instant::now();
    println!("");
    println!("Part 2: {}", solve_part2(INPUT).unwrap());
    println!("(elapsed: {:?})", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "input/test.txt";

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 514579);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 241861950);
    }
}
