use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

use anyhow::Result;

const INPUT: &str = "input/input.txt";

fn find_jolt_differences(
    adapters: &mut HashSet<usize>,
    input_jolt: usize,
    jolt_differences: &mut HashMap<usize, usize>,
) {
    for i in 1..=3 {
        if adapters.remove(&(input_jolt + i)) {
            let entry = jolt_differences.entry(i).or_insert(0);
            *entry += 1;
            find_jolt_differences(adapters, input_jolt + i, jolt_differences)
        }
    }
}

fn count_adapter_combinations(
    adapters: &HashSet<usize>,
    input_jolt: usize,
    target_jolt: usize,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(count) = cache.get(&input_jolt) {
        *count
    } else {
        let count = if input_jolt == target_jolt {
            1
        } else {
            (1..=3)
                .map(|i| {
                    if adapters.contains(&(input_jolt + i)) {
                        count_adapter_combinations(adapters, input_jolt + i, target_jolt, cache)
                    } else {
                        0
                    }
                })
                .sum()
        };
        cache.insert(input_jolt, count);
        count
    }
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut adapters = reader
        .lines()
        .map(|line| Ok(line?.parse()?))
        .collect::<Result<HashSet<usize>>>()?;
    let mut differences = HashMap::new();
    differences.insert(1, 0);
    differences.insert(2, 0);
    differences.insert(3, 0);
    find_jolt_differences(&mut adapters, 0, &mut differences);
    Ok(differences.get(&1).unwrap() * (differences.get(&3).unwrap() + 1))
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let adapters = reader
        .lines()
        .map(|line| Ok(line?.parse()?))
        .collect::<Result<HashSet<usize>>>()?;
    let target_jolt = *adapters.iter().max().expect("non-empty input");
    let mut cache = HashMap::new();
    Ok(count_adapter_combinations(
        &adapters,
        0,
        target_jolt,
        &mut cache,
    ))
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
        assert_eq!(solve_part1(TEST_INPUT1).unwrap(), 35);
        assert_eq!(solve_part1(TEST_INPUT2).unwrap(), 220);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT1).unwrap(), 8);
        assert_eq!(solve_part2(TEST_INPUT2).unwrap(), 19208);
    }
}
