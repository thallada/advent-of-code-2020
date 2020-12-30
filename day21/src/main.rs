#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const INPUT: &str = "input/input.txt";

lazy_static! {
    static ref INGREDIENT_LIST: Regex =
        Regex::new(r"(?P<ingredients>(?:\w+ )+)\(contains (?P<allergens>(:?\w+(:?, )?)+)\)")
            .unwrap();
}

fn map_from_reader<R: BufRead>(
    reader: R,
) -> Result<(HashMap<String, u32>, HashMap<String, HashSet<String>>)> {
    let mut all_ingredients = HashMap::new();
    let mut allergen_ingredient_counts = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        for caps in INGREDIENT_LIST.captures_iter(&line) {
            let ingredients: HashSet<String> = caps["ingredients"]
                .split(' ')
                .filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.to_owned())
                    }
                })
                .collect();
            let allergens: HashSet<String> = caps["allergens"]
                .split(", ")
                .filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.to_owned())
                    }
                })
                .collect();
            for allergen in allergens.iter() {
                match allergen_ingredient_counts.entry(allergen.clone()) {
                    Entry::Occupied(entry) => {
                        let entry: &mut HashSet<String> = entry.into_mut();
                        *entry = entry.intersection(&ingredients).cloned().collect();
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(ingredients.iter().cloned().collect());
                    }
                }
            }
            for ingredient in ingredients.into_iter() {
                let entry = all_ingredients.entry(ingredient).or_insert(0);
                *entry += 1;
            }
        }
    }
    Ok((all_ingredients, allergen_ingredient_counts))
}

fn solve_part1(input_path: &str) -> Result<u32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let (mut all_ingredients, mut allergen_ingredient_counts) = map_from_reader(reader)?;
    let mut ingredient;
    loop {
        if let Some(ingredients) = allergen_ingredient_counts
            .values_mut()
            .find(|ingredients| ingredients.len() == 1)
        {
            ingredient = ingredients.drain().next().expect("non-empty set");
            all_ingredients.remove(&ingredient);
            for ingredients in allergen_ingredient_counts.values_mut() {
                ingredients.remove(&ingredient);
            }
        } else {
            break;
        }
    }
    Ok(all_ingredients.values().sum())
}

fn solve_part2(input_path: &str) -> Result<String> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut dangerous_ingredients = Vec::new();
    let (_, mut allergen_ingredient_counts) = map_from_reader(reader)?;
    let mut ingredient;
    loop {
        if let Some((allergen, ingredients)) = allergen_ingredient_counts
            .iter_mut()
            .find(|(_, ingredients)| ingredients.len() == 1)
        {
            let allergen = allergen.clone();
            ingredient = ingredients.drain().next().expect("non-empty set");
            for ingredients in allergen_ingredient_counts.values_mut() {
                ingredients.remove(&ingredient);
            }
            dangerous_ingredients.push((allergen, ingredient));
        } else {
            break;
        }
    }
    dangerous_ingredients.sort_unstable_by_key(|(allergen, _)| allergen.clone());
    Ok(dangerous_ingredients
        .iter()
        .map(|(_, ingredient)| ingredient.as_str())
        .collect::<Vec<&str>>()
        .join(","))
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 5);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            solve_part2(TEST_INPUT).unwrap(),
            "mxmxvkd,sqjhc,fvjkl".to_string()
        );
    }
}
