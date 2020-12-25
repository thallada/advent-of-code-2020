use anyhow::{Context, Error, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Instant;

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq, Eq)]
enum Rule {
    Char(char),
    Seq(Vec<usize>),
    Or((Vec<usize>, Vec<usize>)),
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains("|") {
            let mut rule = s.split(" | ");
            let left = rule.next().context("Failed to parse OR rule left side")?;
            let left = left
                .split(" ")
                .map(|num| Ok(num.parse()?))
                .collect::<Result<Vec<usize>>>()?;
            let right = rule.next().context("Failed to parse OR rule right side")?;
            let right = right
                .split(" ")
                .map(|num| Ok(num.parse()?))
                .collect::<Result<Vec<usize>>>()?;
            Ok(Rule::Or((left, right)))
        } else if s.contains("\"") {
            let c = s
                .chars()
                .skip(1)
                .next()
                .context("Failed to parse rule char")?;
            Ok(Rule::Char(c))
        } else {
            let nums = s
                .split(" ")
                .map(|num| Ok(num.parse()?))
                .collect::<Result<Vec<usize>>>()?;
            Ok(Rule::Seq(nums))
        }
    }
}

fn parse_indexed_rule(s: &str) -> Result<(usize, Rule)> {
    let mut rule = s.split(": ");
    let index = rule.next().context("Failed to parse rule index")?.parse()?;
    let rule = rule.next().context("Failed to parse rule")?.parse()?;
    Ok((index, rule))
}

fn string_matches_rule<'a>(
    s: &'a str,
    rules: &HashMap<usize, Rule>,
    rule: &Rule,
    ends: bool,
) -> Vec<(&'a str, bool)> {
    match rule {
        Rule::Char(c) => {
            if s.len() < 1 {
                vec![(s, false)]
            } else if s.chars().next().expect("non-empty string") == *c {
                let rest = &s[1..];
                if !ends || (ends && rest.is_empty()) {
                    vec![(rest, true)]
                } else {
                    vec![(rest, false)]
                }
            } else {
                vec![(s, false)]
            }
        }
        Rule::Seq(seq) => {
            let mut possibilities: Vec<(&'a str, bool)> = Vec::new();
            for (i, rule_index) in seq.iter().enumerate() {
                if i == 0 {
                    possibilities = string_matches_rule(
                        s,
                        rules,
                        &rules[rule_index],
                        if i == seq.len() - 1 { ends } else { false },
                    );
                } else {
                    let mut new_possibilities = Vec::new();
                    for possibility in possibilities {
                        if possibility.1 {
                            new_possibilities.append(&mut string_matches_rule(
                                possibility.0,
                                rules,
                                &rules[rule_index],
                                if i == seq.len() - 1 { ends } else { false },
                            ));
                        }
                    }
                    possibilities = new_possibilities;
                }
            }
            possibilities
        }
        Rule::Or((left, right)) => {
            let mut possibilities = string_matches_rule(s, rules, &Rule::Seq(left.to_vec()), ends);
            possibilities.append(&mut string_matches_rule(
                s,
                rules,
                &Rule::Seq(right.to_vec()),
                ends,
            ));
            possibilities
        }
    }
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    loop {
        let line = lines.next().context("Unexpected end of input")??;
        if line.is_empty() {
            break;
        }
        let (index, rule) = parse_indexed_rule(&line)?;
        rules.insert(index, rule);
    }

    let strings: Vec<String> =
        lines.collect::<std::result::Result<Vec<String>, std::io::Error>>()?;

    Ok(strings
        .iter()
        .filter(|s| {
            let results = string_matches_rule(&s, &rules, &rules[&0], true);
            results.iter().any(|result| result.1 && result.0.is_empty())
        })
        .count())
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    loop {
        let line = lines.next().context("Unexpected end of input")??;
        if line.is_empty() {
            break;
        }
        let (index, rule) = parse_indexed_rule(&line)?;
        let rule = if index == 8 {
            Rule::Or((vec![42], vec![42, 8]))
        } else if index == 11 {
            Rule::Or((vec![42, 31], vec![42, 11, 31]))
        } else {
            rule
        };
        rules.insert(index, rule);
    }

    let strings: Vec<String> =
        lines.collect::<std::result::Result<Vec<String>, std::io::Error>>()?;
    Ok(strings
        .into_iter()
        .filter(|s| {
            let results = string_matches_rule(&s, &rules, &rules[&0], true);
            results.iter().any(|result| result.1 && result.0.is_empty())
        })
        .count())
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
    use maplit::hashmap;

    const TEST_INPUT1: &str = "input/test1.txt";
    const TEST_INPUT2: &str = "input/test2.txt";
    const TEST_INPUT3: &str = "input/test3.txt";

    #[test]
    fn parses_input() {
        let file = File::open(TEST_INPUT1).unwrap();
        let reader = BufReader::new(file);
        let rules: HashMap<usize, Rule> = reader
            .lines()
            .take_while(|line| !line.as_ref().map_or(true, |line| line.is_empty()))
            .map(|line| parse_indexed_rule(&line.unwrap()).unwrap())
            .collect();

        assert_eq!(
            rules,
            hashmap! {
                0 => Rule::Seq(vec![4, 1, 5]),
                1 => Rule::Or((vec![2, 3], vec![3, 2])),
                2 => Rule::Or((vec![4, 4], vec![5, 5])),
                3 => Rule::Or((vec![4, 5], vec![5, 4])),
                4 => Rule::Char('a'),
                5 => Rule::Char('b'),
            }
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT1).unwrap(), 2);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part1(TEST_INPUT2).unwrap(), 3);
        assert_eq!(solve_part1(TEST_INPUT3).unwrap(), 1);
        assert_eq!(solve_part2(TEST_INPUT2).unwrap(), 12);
    }
}
