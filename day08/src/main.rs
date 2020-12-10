use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Context, Error, Result};

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq)]
enum Operation {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut instruction = s.split(" ");
        let op = instruction.next().context("Failed to parse operation")?;
        let offset: i32 = instruction
            .next()
            .context("Failed to parse operation offset")?
            .parse()?;
        match op {
            "nop" => Ok(Operation::Nop(offset)),
            "acc" => Ok(Operation::Acc(offset)),
            "jmp" => Ok(Operation::Jmp(offset)),
            _ => Err(anyhow!("Unrecognized operation: {}", op)),
        }
    }
}

fn find_infinite_loop(program: &Vec<Operation>) -> i32 {
    let mut accumulator: i32 = 0;
    let mut pointer: i32 = 0;
    let mut visited = HashSet::new();
    while !visited.contains(&pointer) {
        visited.insert(pointer);
        match program[pointer as usize] {
            Operation::Nop(_) => pointer += 1,
            Operation::Acc(offset) => {
                accumulator += offset;
                pointer += 1;
            }
            Operation::Jmp(offset) => pointer += offset,
        }
    }
    accumulator
}

fn find_terminated_value(program: &Vec<Operation>, flip_pointer: i32) -> Option<i32> {
    let mut accumulator: i32 = 0;
    let mut pointer: i32 = 0;
    let mut visited = HashSet::new();
    while !visited.contains(&pointer) {
        if pointer == program.len() as i32 {
            return Some(accumulator);
        } else if pointer >= program.len() as i32 || pointer < 0 {
            return None;
        }
        visited.insert(pointer);
        match program[pointer as usize] {
            Operation::Nop(offset) if flip_pointer == pointer => pointer += offset,
            Operation::Nop(_) => pointer += 1,
            Operation::Acc(offset) => {
                accumulator += offset;
                pointer += 1;
            }
            Operation::Jmp(_) if flip_pointer == pointer => pointer += 1,
            Operation::Jmp(offset) => pointer += offset,
        }
    }
    None
}

fn solve_part1(input_path: &str) -> Result<i32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let program: Vec<Operation> = reader
        .lines()
        .map(|line| Operation::from_str(&line.unwrap()).unwrap())
        .collect();
    Ok(find_infinite_loop(&program))
}

fn solve_part2(input_path: &str) -> Result<i32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let program: Vec<Operation> = reader
        .lines()
        .map(|line| Operation::from_str(&line.unwrap()).unwrap())
        .collect();

    let mut flip_pointer: i32 = 0;
    while flip_pointer < program.len() as i32 {
        if matches!(program[flip_pointer as usize], Operation::Nop(_) | Operation::Jmp(_)) {
            if let Some(result) = find_terminated_value(&program, flip_pointer) {
                return Ok(result);
            }
        }
        flip_pointer += 1;
    }
    Err(anyhow!("No fix found"))
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
    fn parses_input() {
        let file = File::open(TEST_INPUT).unwrap();
        let reader = BufReader::new(file);

        let instructions: Vec<Operation> = reader
            .lines()
            .map(|line| Operation::from_str(&line.unwrap()).unwrap())
            .collect();

        assert_eq!(
            instructions,
            vec![
                Operation::Nop(0),
                Operation::Acc(1),
                Operation::Jmp(4),
                Operation::Acc(3),
                Operation::Jmp(-3),
                Operation::Acc(-99),
                Operation::Acc(1),
                Operation::Jmp(-4),
                Operation::Acc(6),
            ]
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 5);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 8);
    }
}
