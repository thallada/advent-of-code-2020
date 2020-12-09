use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

use anyhow::Result;

const INPUT: &str = "input/input.txt";

fn find_invalid_num(nums: &[usize], preamble: usize) -> Option<usize> {
    if let Some(num) = nums.windows(preamble + 1).find(|chunk| {
        for num in &chunk[0..preamble] {
            for other_num in &chunk[0..preamble] {
                if num != other_num && num + other_num == chunk[preamble] {
                    return false;
                }
            }
        }
        true
    }) {
        return Some(num[preamble]);
    }
    None
}

fn find_encryption_weakness(nums: &[usize], invalid_num: usize) -> Option<usize> {
    let mut window_size = 2;
    while window_size < 1000 {
        if let Some(weakness) = nums
            .windows(window_size)
            .find(|chunk| chunk.iter().sum::<usize>() == invalid_num)
        {
            return Some(
                weakness.iter().min().expect("non-empty slice")
                    + weakness.iter().max().expect("non-empty slice"),
            );
        } else {
            window_size += 1;
        }
    }
    None
}

fn solve_part1(input_path: &str, preamble: usize) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let nums = reader
        .lines()
        .map(|line| Ok(line?.parse()?))
        .collect::<Result<Vec<usize>>>()?;
    Ok(find_invalid_num(&nums, preamble).unwrap())
}

fn solve_part2(input_path: &str, invalid_num: usize) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let nums = reader
        .lines()
        .map(|line| Ok(line?.parse()?))
        .collect::<Result<Vec<usize>>>()?;
    Ok(find_encryption_weakness(&nums, invalid_num).unwrap())
}

fn main() {
    let mut now = Instant::now();
    let part1 = solve_part1(INPUT, 25).unwrap();
    println!("Part 1: {}", part1);
    println!(
        "(elapsed: {} ms)",
        now.elapsed().as_micros() as f32 / 1000_f32
    );
    now = Instant::now();
    println!("");
    println!("Part 2: {}", solve_part2(INPUT, part1).unwrap());
    println!(
        "(elapsed: {} ms)",
        now.elapsed().as_micros() as f32 / 1000_f32
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "input/test.txt";

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT, 5).unwrap(), 127);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT, 127).unwrap(), 62);
    }
}
