#[macro_use]
extern crate lazy_static;
extern crate maplit;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use anyhow::{anyhow, Result};
use regex::Regex;

const INPUT: &str = "input/input.txt";

lazy_static! {
    static ref CONTAINER_BAG: Regex =
        Regex::new(r"(?P<container_color>[\w\s]+) bags contain").unwrap();
    static ref INNER_BAGS: Regex =
        Regex::new(r"(?P<bag_quantity>\d+) (?P<bag_color>[\w\s]+) bags?(, )?").unwrap();
}

type InnerToContainerBags = HashMap<String, HashSet<String>>;
type ContainerToInnerBags = HashMap<String, Vec<(u32, String)>>;

fn reverse_map_from_reader<R: BufRead>(reader: R) -> Result<InnerToContainerBags> {
    let mut bags = HashMap::new();
    for rule in reader.lines() {
        let rule = rule?;
        let container_color = match CONTAINER_BAG.captures(&rule) {
            None => Err(anyhow!("Malformed container bag rule")),
            Some(captures) => Ok(captures["container_color"].to_string()),
        }?;

        for captures in INNER_BAGS.captures_iter(&rule) {
            let color = captures["bag_color"].to_string();
            let inner_bags = bags.entry(color).or_insert(HashSet::new());
            inner_bags.insert(container_color.clone());
        }
    }

    Ok(bags)
}

fn map_from_reader<R: BufRead>(reader: R) -> Result<ContainerToInnerBags> {
    let mut bags = HashMap::new();
    for rule in reader.lines() {
        let rule = rule?;
        let container_color = match CONTAINER_BAG.captures(&rule) {
            None => Err(anyhow!("Malformed container bag rule")),
            Some(captures) => Ok(captures["container_color"].to_string()),
        }?;

        bags.insert(
            container_color,
            INNER_BAGS
                .captures_iter(&rule)
                .map(|captures| {
                    let color = captures["bag_color"].to_string();
                    let quantity = captures["bag_quantity"].parse()?;
                    Ok((quantity, color))
                })
                .collect::<Result<Vec<(u32, String)>>>()?,
        );
    }

    Ok(bags)
}

fn visit_container_bags<'a>(
    bags: &'a InnerToContainerBags,
    visited: &mut HashSet<&'a str>,
    color: &'a str,
) {
    if visited.contains(color) {
        return;
    }
    visited.insert(color);

    if !bags.contains_key(color) {
        return;
    } else {
        for container_bag in bags[color].iter() {
            visit_container_bags(bags, visited, container_bag);
        }
    };
}

fn count_inner_bags<'a>(
    bags: &'a ContainerToInnerBags,
    cached: &mut HashMap<&'a str, u32>,
    color: &'a str,
) -> u32 {
    if cached.contains_key(color) {
        return cached[color];
    }

    let val = if !bags.contains_key(color) {
        1
    } else {
        1 + bags[color]
            .iter()
            .map(|(quantitiy, inner_bag)| quantitiy * count_inner_bags(bags, cached, inner_bag))
            .sum::<u32>()
    };
    cached.insert(color, val);
    val
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let bags = reverse_map_from_reader(reader)?;

    let mut visited = HashSet::new();
    visit_container_bags(&bags, &mut visited, "shiny gold");
    Ok(visited.len() - 1)
}

fn solve_part2(input_path: &str) -> Result<u32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let bags = map_from_reader(reader)?;

    let mut cached = HashMap::new();
    Ok(count_inner_bags(&bags, &mut cached, "shiny gold") - 1)
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
    use maplit::{hashmap, hashset};

    use super::*;

    const TEST_INPUT1: &str = "input/test1.txt";
    const TEST_INPUT2: &str = "input/test2.txt";

    #[test]
    fn parses_input_to_reverse_map() {
        let file = File::open(TEST_INPUT1).unwrap();
        let reader = BufReader::new(file);
        let bags = reverse_map_from_reader(reader).unwrap();
        let expected = hashmap! {
            "vibrant plum".into() => hashset! {
                "shiny gold".into(),
            },
            "shiny gold".into() => hashset! {
                "bright white".into(),
                "muted yellow".into(),
            },
            "bright white".into() => hashset! {
                "dark orange".into(),
                "light red".into(),
            },
            "dark olive".into() => hashset! {
                "shiny gold".into(),
            },
            "dotted black".into() => hashset! {
                "vibrant plum".into(),
                "dark olive".into(),
            },
            "muted yellow".into() => hashset! {
                "light red".into(),
                "dark orange".into(),
            },
            "faded blue".into() => hashset! {
                "dark olive".into(),
                "vibrant plum".into(),
                "muted yellow".into(),
            },
        };
        assert_eq!(bags, expected);
    }

    #[test]
    fn parses_input_to_map() {
        let file = File::open(TEST_INPUT1).unwrap();
        let reader = BufReader::new(file);
        let bags = map_from_reader(reader).unwrap();
        let expected = hashmap! {
            "light red".into() => std::vec![
                (1, "bright white".into()),
                (2, "muted yellow".into()),
            ],
            "dark orange".into() => std::vec![
                (3, "bright white".into()),
                (4, "muted yellow".into()),
            ],
            "bright white".into() => std::vec![
                (1, "shiny gold".into()),
            ],
            "muted yellow".into() => std::vec![
                (2, "shiny gold".into()),
                (9, "faded blue".into()),
            ],
            "shiny gold".into() => std::vec![
                (1, "dark olive".into()),
                (2, "vibrant plum".into()),
            ],
            "dark olive".into() => std::vec![
                (3, "faded blue".into()),
                (4, "dotted black".into()),
            ],
            "vibrant plum".into() => std::vec![
                (5, "faded blue".into()),
                (6, "dotted black".into()),
            ],
            "faded blue".into() => std::vec![],
            "dotted black".into() => std::vec![],
        };
        assert_eq!(bags, expected);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT1).unwrap(), 4);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT1).unwrap(), 32);
        assert_eq!(solve_part2(TEST_INPUT2).unwrap(), 126);
    }
}
