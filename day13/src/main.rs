use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

use anyhow::{anyhow, Result};

const INPUT: &str = "input/input.txt";

// Shamelessly copied from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
// I did not like this problem.

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn solve_part1(input_path: &str) -> Result<u32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let earliest: u32 = lines
        .next()
        .expect("first line contains earliest timestamp")?
        .parse()?;
    dbg!(earliest);
    let (bus, min) = lines
        .next()
        .expect("second line contains bus ids")?
        .split(",")
        .filter_map(|id| id.parse().ok())
        .map(|bus: u32| {
            let mut time = bus;
            while time < earliest {
                time += bus;
            }
            (bus, time)
        })
        .min_by_key(|&(_, time)| time)
        .expect("buses list is not empty");
    Ok((min - earliest) * bus)
}

fn solve_part2(input_path: &str) -> Result<i64> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let buses: Vec<(i64, i64)> = reader
        .lines()
        .skip(1)
        .next()
        .expect("second line contains bus ids")?
        .split(",")
        .enumerate()
        .filter_map(|(index, bus)| {
            if let Ok(bus) = bus.parse::<i64>() {
                Some((index as i64, bus))
            } else {
                None
            }
        })
        .collect();
    dbg!(&buses);

    let modulii: Vec<i64> = buses.iter().map(|&(_, bus)| bus).collect();
    let residues: Vec<i64> = buses.iter().map(|&(index, bus)| bus - index).collect();

    chinese_remainder(residues.as_slice(), modulii.as_slice()).ok_or_else(|| anyhow!("no result"))
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 295);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 1068781);
    }
}
