use anyhow::{anyhow, Result};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT: &str = "input/input.txt";

fn solve_part1(input_path: &str) -> Result<u32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut prev_lines = Vec::new();
    for line in reader.lines() {
        let number: u32 = line?.parse()?;
        for prev_number in prev_lines.iter() {
            if number + prev_number == 2020 {
                return Ok(number * prev_number);
            }
        }
        prev_lines.push(number);
    }

    Err(anyhow!("Found no pair of numbers that sums to 2020"))
}

fn solve_part2(input_path: &str) -> Result<u32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut prev_lines = Vec::new();
    for line in reader.lines() {
        let number: u32 = line?.parse()?;
        for prev_number in prev_lines.iter() {
            for third_number in prev_lines.iter() {
                if number + prev_number + third_number == 2020 {
                    return Ok(number * prev_number * third_number);
                }
            }
        }
        prev_lines.push(number);
    }

    Err(anyhow!("Found no three numbers that sum to 2020"))
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT).unwrap());
    println!("Part 2: {}", solve_part2(INPUT).unwrap());
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
