use anyhow::{Context, Error, Result};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::RangeInclusive;
use std::str::FromStr;

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq)]
struct PasswordRule {
    letter: char,
    range: RangeInclusive<usize>,
}

impl FromStr for PasswordRule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" ");
        let mut range = parts
            .next()
            .context("Failed to parse PasswordRule range")?
            .split("-");
        let start = range
            .next()
            .context("Failed to parse PasswordRule range start")?
            .parse()?;
        let end = range
            .next()
            .context("Failed to parse PasswordRule range end")?
            .parse()?;
        let letter = parts
            .next()
            .context("Failed to parse PasswordRule letter")?
            .parse()?;
        Ok(Self {
            letter,
            range: start..=end,
        })
    }
}

#[derive(Debug, PartialEq)]
struct PasswordEntry {
    rule: PasswordRule,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(": ");
        let rule = parts
            .next()
            .context("Failed to parse PasswordEntry rule")?
            .parse()?;
        let password = parts
            .next()
            .context("Failed to parse PasswordEntry password")?
            .to_owned();
        Ok(Self { rule, password })
    }
}

impl PasswordEntry {
    fn validate_occurences(&self) -> bool {
        self.rule.range.contains(
            &self
                .password
                .chars()
                .filter(|&c| c == self.rule.letter)
                .count(),
        )
    }

    fn validate_positions(&self) -> bool {
        let mut chars = self.password.chars();
        let left = chars.nth(self.rule.range.start() - 1) == Some(self.rule.letter);
        let right = chars.nth(self.rule.range.end() - self.rule.range.start() - 1)
            == Some(self.rule.letter);
        left ^ right
    }
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| line.unwrap().parse::<PasswordEntry>().unwrap())
        .filter(|entry| entry.validate_occurences())
        .count())
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| line.unwrap().parse::<PasswordEntry>().unwrap())
        .filter(|entry| entry.validate_positions())
        .count())
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
    fn parses_input() {
        let file = File::open(TEST_INPUT).unwrap();
        let reader = BufReader::new(file);
        let input: Vec<PasswordEntry> = reader
            .lines()
            .map(|line| line.unwrap().parse::<PasswordEntry>().unwrap())
            .collect();
        assert_eq!(
            input[0],
            PasswordEntry {
                rule: PasswordRule {
                    letter: 'a',
                    range: 1..=3,
                },
                password: "abcde".to_string(),
            }
        );
        assert_eq!(
            input[1],
            PasswordEntry {
                rule: PasswordRule {
                    letter: 'b',
                    range: 1..=3,
                },
                password: "cdefg".to_string(),
            }
        );
        assert_eq!(
            input[2],
            PasswordEntry {
                rule: PasswordRule {
                    letter: 'c',
                    range: 2..=9,
                },
                password: "ccccccccc".to_string(),
            }
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 2);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 1);
    }
}
