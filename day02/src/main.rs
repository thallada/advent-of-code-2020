use anyhow::{anyhow, Error, Result};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;
use std::str::FromStr;

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq)]
struct PasswordRule {
    letter: char,
    range: Range<usize>,
}

impl FromStr for PasswordRule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" ");
        let mut range = parts
            .next()
            .ok_or_else(|| anyhow!("Failed to parse PasswordRule range"))?
            .split("-");
        let start = range
            .next()
            .ok_or_else(|| anyhow!("Failed to parse PasswordRule range start"))?
            .parse()?;
        let end: usize = range
            .next()
            .ok_or_else(|| anyhow!("Failed to parse PasswordRule range end"))?
            .parse()?;
        let letter = parts
            .next()
            .ok_or_else(|| anyhow!("Failed to parse PasswordRule letter"))?
            .parse()?;
        Ok(Self {
            letter,
            range: Range {
                start,
                end: end + 1,
            },
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
            .ok_or_else(|| anyhow!("Failed to parse PasswordRule range"))?
            .parse()?;
        let password = parts
            .next()
            .ok_or_else(|| anyhow!("Failed to parse PasswordRule range start"))?
            .to_owned();
        Ok(Self { rule, password })
    }
}

impl PasswordEntry {
    fn validate_occurences(&self) -> bool {
        self.rule
            .range
            .contains(&self.password.chars().fold(0, |acc, c| match c {
                _ if c == self.rule.letter => acc + 1,
                _ => acc,
            }))
    }

    fn validate_positions(&self) -> bool {
        let mut chars = self.password.chars();
        let left = chars.nth(self.rule.range.start - 1) == Some(self.rule.letter);
        let right =
            chars.nth(self.rule.range.end - self.rule.range.start - 2) == Some(self.rule.letter);
        left ^ right
    }
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| PasswordEntry::from_str(&line.unwrap()).unwrap())
        .filter(|entry| entry.validate_occurences())
        .count())
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| PasswordEntry::from_str(&line.unwrap()).unwrap())
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
            .map(|line| PasswordEntry::from_str(&line.unwrap()).unwrap())
            .collect();
        assert_eq!(
            input[0],
            PasswordEntry {
                rule: PasswordRule {
                    letter: 'a',
                    range: Range { start: 1, end: 4 }
                },
                password: "abcde".to_string(),
            }
        );
        assert_eq!(
            input[1],
            PasswordEntry {
                rule: PasswordRule {
                    letter: 'b',
                    range: Range { start: 1, end: 4 }
                },
                password: "cdefg".to_string(),
            }
        );
        assert_eq!(
            input[2],
            PasswordEntry {
                rule: PasswordRule {
                    letter: 'c',
                    range: Range { start: 2, end: 10 }
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
