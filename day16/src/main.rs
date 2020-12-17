use anyhow::{Context, Error, Result};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::Instant;

const INPUT: &str = "input/input.txt";

#[derive(Debug)]
struct Rule {
    field: String,
    ranges: (RangeInclusive<usize>, RangeInclusive<usize>),
}

type Ticket = Vec<usize>;

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rule = s.split(": ");
        let field = rule
            .next()
            .context("Failed to parse rule field name")?
            .to_owned();
        let ranges = rule.next().context("Failed to parse rule field name")?;
        let mut ranges = ranges.split(" or ");
        let range1 = ranges.next().context("Failed to parse rule first range")?;
        let mut range1 = range1.split("-");
        let range1 = RangeInclusive::new(
            range1
                .next()
                .context("Failed to parse rule first range start")?
                .parse()?,
            range1
                .next()
                .context("Failed to parse rule first range end")?
                .parse()?,
        );
        let range2 = ranges.next().context("Failed to parse rule second range")?;
        let mut range2 = range2.split("-");
        let range2 = RangeInclusive::new(
            range2
                .next()
                .context("Failed to parse rule second range start")?
                .parse()?,
            range2
                .next()
                .context("Failed to parse rule second range end")?
                .parse()?,
        );
        Ok(Self {
            field,
            ranges: (range1, range2),
        })
    }
}

impl Rule {
    fn validate_num(&self, num: usize) -> bool {
        self.ranges.0.contains(&num) || self.ranges.1.contains(&num)
    }
}

fn validate_ticket(rules: &Vec<Rule>, ticket: &Ticket) -> bool {
    ticket
        .into_iter()
        .all(|&num| rules.iter().any(|rule| rule.validate_num(num)))
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut rules = Vec::new();
    loop {
        let line = lines.next().context("Unexpected end of input")??;
        if line.is_empty() {
            break;
        } else {
            rules.push(Rule::from_str(&line)?);
        }
    }
    let nearby_tickets: Vec<Ticket> = lines
        .skip(4)
        .map(|line| {
            Ok(line?
                .split(",")
                .map(|num| Ok(num.parse()?))
                .collect::<Result<Ticket>>()?)
        })
        .collect::<Result<Vec<Ticket>>>()?;

    Ok(nearby_tickets.iter().fold(0, |acc, ticket| {
        if let Some(invalid_num) = ticket
            .into_iter()
            .find(|&num| !rules.iter().any(|rule| rule.validate_num(*num)))
        {
            acc + invalid_num
        } else {
            acc
        }
    }))
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut rules = Vec::new();
    loop {
        let line = lines.next().context("Unexpected end of input")??;
        if line.is_empty() {
            break;
        } else {
            rules.push(Rule::from_str(&line)?);
        }
    }
    let your_ticket: Ticket = lines
        .by_ref()
        .skip(1)
        .next()
        .context("Failed to parse your ticket")??
        .split(",")
        .map(|num| Ok(num.parse()?))
        .collect::<Result<Ticket>>()?;
    let mut tickets: Vec<Ticket> = lines
        .skip(2)
        .map(|line| {
            Ok(line?
                .split(",")
                .map(|num| Ok(num.parse()?))
                .collect::<Result<Ticket>>()?)
        })
        .collect::<Result<Vec<Ticket>>>()?;
    tickets.push(your_ticket.clone());

    // TODO: might be able to get rid of this extra iteration
    let valid_tickets: Vec<Ticket> = tickets
        .into_iter()
        .filter(|ticket| validate_ticket(&rules, ticket))
        .collect();

    let mut rule_validations: HashMap<&str, HashMap<usize, usize>> = HashMap::new();
    for ticket in valid_tickets.iter() {
        for (i, num) in ticket.iter().enumerate() {
            for rule in rules.iter() {
                if rule.validate_num(*num) {
                    let rule_entry = rule_validations
                        .entry(&rule.field)
                        .or_insert_with(|| HashMap::new());
                    let validate_count = rule_entry.entry(i).or_insert(0);
                    *validate_count += 1;
                }
            }
        }
    }

    let mut rule_positions: HashMap<&str, &usize> = HashMap::new();
    let mut assigned_positions = HashSet::new();
    while rule_positions.len() != rules.len() {
        for (field, validations) in rule_validations.iter() {
            let possible_positions: Vec<(&usize, &usize)> = validations
                .iter()
                .filter(|&(position, count)| {
                    count == &valid_tickets.len() && !assigned_positions.contains(&position)
                })
                .collect();
            if possible_positions.len() == 1 {
                let position = possible_positions.last().expect("count is 1").0;
                rule_positions.insert(field, position);
                assigned_positions.insert(position);
            }
        }
    }

    Ok([
        "departure location",
        "departure station",
        "departure platform",
        "departure track",
        "departure date",
        "departure time",
    ]
    .iter()
    .map(|field| your_ticket[*rule_positions[field]])
    .product())
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

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT1).unwrap(), 71);
    }
}
