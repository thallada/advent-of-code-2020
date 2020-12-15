use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

const INPUT: &str = "input/input.txt";

fn solve_part1(input_path: &str) -> Result<u64> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut or_mask = 0;
    let mut and_mask = u64::MAX;
    let mut memory = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("mask") {
            let mask = line
                .split(" = ")
                .skip(1)
                .next()
                .context("Failed to parse mask")?;
            or_mask = u64::from_str_radix(&mask.replace("X", "0"), 2)?;
            and_mask = u64::from_str_radix(&mask.replace("X", "1"), 2)?;
        } else {
            let mut write = line.split(" = ");
            let address: u64 = write.next().context("Failed to parse write address")?[4..]
                .trim_end_matches("]")
                .parse()?;
            let value: u64 = write
                .next()
                .context("Failed to parse write value")?
                .parse()?;
            memory.insert(address, (value | or_mask) & and_mask);
        }
    }
    Ok(memory.values().sum())
}

fn build_addresses(mask: &str, input_address: u64) -> Vec<u64> {
    let mut addresses = Vec::new();
    for (i, c) in mask.chars().rev().enumerate() {
        let input = (input_address >> i) & 1;
        match c {
            '0' => {
                if addresses.is_empty() {
                    addresses.push(input);
                } else {
                    addresses = addresses.iter_mut().map(|v| (*v << 1) | input).collect();
                }
            }
            '1' => {
                if addresses.is_empty() {
                    addresses.push(1);
                } else {
                    addresses = addresses.iter_mut().map(|v| (*v << 1) | 1).collect();
                }
            }
            'X' => {
                if addresses.is_empty() {
                    addresses.push(0);
                    addresses.push(1);
                } else {
                    let mut new_addresses = Vec::new();
                    for v in addresses.iter() {
                        new_addresses.push((*v << 1) | 0);
                        new_addresses.push((*v << 1) | 1);
                    }
                    addresses = new_addresses;
                }
            }
            _ => unreachable!(),
        }
    }
    addresses
}

fn solve_part2(input_path: &str) -> Result<u64> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut mask = String::new();
    let mut memory = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("mask") {
            mask = line
                .split(" = ")
                .skip(1)
                .next()
                .context("Failed to parse mask")?
                .to_owned();
        } else {
            let mut write = line.split(" = ");
            let address: u64 = write.next().context("Failed to parse write address")?[4..]
                .trim_end_matches("]")
                .parse()?;
            let value: u64 = write
                .next()
                .context("Failed to parse write value")?
                .parse()?;
            for address in build_addresses(&mask, address) {
                memory.insert(address, value);
            }
        }
    }
    Ok(memory.values().sum())
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

    const TEST_INPUT1: &str = "input/test1.txt";
    const TEST_INPUT2: &str = "input/test2.txt";

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT1).unwrap(), 165);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT2).unwrap(), 208);
    }
}
