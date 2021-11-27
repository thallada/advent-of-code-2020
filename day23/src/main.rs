use anyhow::{Error, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::str::FromStr;
use std::fmt;

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq, Eq)]
struct Circle {
    cups: Vec<u64>,
    current_cup: usize,
}

impl FromStr for Circle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let cups = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        Ok(Circle { cups, current_cup: 0 })
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, cup) in self.cups.iter().enumerate() {
            if self.current_cup == index {
                write!(f, " ({}) ", cup)?;
            } else {
                write!(f, " {} ", cup)?;
            }
        }
        writeln!(f, "")?;
        Ok(())
    }
}

impl Circle {
    // need to rewrite this for part 2 to use a secondary "next" Vec that uses a cup's label as the
    // index and the next (clockwise) cup's label as the value.
    fn run_move(&mut self) {
        let current_cup_label = self.cups[self.current_cup];
        if self.current_cup + 4 > self.cups.len() {
            self.cups.rotate_left(3);
            self.current_cup = self.current_cup - 3;
        }
        let three_cups_start = (self.current_cup + 1) % self.cups.len();
        let three_cups_end = (self.current_cup + 4) % (self.cups.len() + 1);
        let three_cups: Vec<u64> = self.cups.drain(three_cups_start..three_cups_end).collect();
        let mut destination_cup_label = (current_cup_label - 1) % 10;
        loop {
            let destination_cup = self.cups.iter().position(|&c| c == destination_cup_label);
            if let Some(destination_cup) = destination_cup {
                let destination_cup = destination_cup + 1 % self.cups.len();
                self.cups.splice(destination_cup..destination_cup, three_cups);
                break;
            }
            destination_cup_label = destination_cup_label.checked_sub(1).unwrap_or(9);
        }
        self.current_cup = self.cups.iter().position(|&c| c == current_cup_label).unwrap();
        self.current_cup = (self.current_cup + 1) % self.cups.len();
    }

    fn expand_for_part2(&mut self) {
        let mut max = *self.cups.iter().max().unwrap() as u64;
        while max <= 1000000 {
            self.cups.push(max);
            max += 1;
        }
    }
}

fn solve_part1(input_path: &str, n: usize) -> Result<u64> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;
    let mut circle = Circle::from_str(&line)?;
    for _ in 0..n {
        circle.run_move();
    }
    let one_index = circle.cups.iter().position(|&c| c == 1).unwrap();
    circle.cups.rotate_left(one_index);
    Ok(circle.cups.iter().skip(1).map(|cup| cup.to_string()).collect::<Vec<String>>().join("").parse()?)
}

fn solve_part2(input_path: &str, n: usize) -> Result<u64> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;
    let mut circle = Circle::from_str(&line)?;
    circle.expand_for_part2();
    for _ in 0..n {
        circle.run_move();
    }
    let one_index = circle.cups.iter().position(|&c| c == 1).unwrap();
    Ok(
        circle.cups[(one_index + 1) % circle.cups.len()] as u64 *
        circle.cups[(one_index + 2) % circle.cups.len()] as u64
    )
}

fn main() {
    let mut now = Instant::now();
    println!("Part 1: {}", solve_part1(INPUT, 100).unwrap());
    println!("(elapsed: {:?})", now.elapsed());
    now = Instant::now();
    println!("");
    println!("Part 2: {}", solve_part2(INPUT, 10000000).unwrap());
    println!("(elapsed: {:?})", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "input/test.txt";

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT, 10).unwrap(), 92658374);
        assert_eq!(solve_part1(TEST_INPUT, 100).unwrap(), 67384529);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            solve_part2(TEST_INPUT, 10000000).unwrap(),
            149245887792
        );
    }
}
