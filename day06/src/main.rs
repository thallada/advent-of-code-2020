use anyhow::Result;

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

const INPUT: &str = "input/input.txt";

fn solve_part1(input_path: &str) -> Result<usize> {
    let answers = read_to_string(input_path)?;
    Ok(answers
        .split("\n\n")
        .map(|group| {
            let mut answered = HashSet::new();
            for person_answers in group.split("\n") {
                for c in person_answers.chars() {
                    answered.insert(c);
                }
            }
            answered.len()
        })
        .sum())
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let answers = read_to_string(input_path)?;
    Ok(answers
        .split("\n\n")
        .map(|group| {
            let mut answer_counts = HashMap::new();
            let mut group_len = 0;
            for person_answers in group.trim().split("\n") {
                for c in person_answers.chars() {
                    let counter = answer_counts.entry(c).or_insert(0);
                    *counter += 1;
                }
                group_len += 1;
            }
            answer_counts
                .values()
                .filter(|&&count| count == group_len)
                .count()
        })
        .sum())
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 11);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 6);
    }
}
