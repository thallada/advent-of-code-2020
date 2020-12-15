use anyhow::{anyhow, Result};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

const INPUT: &str = "input/input.txt";

fn take_turn(nums: &mut Vec<usize>) {
    if let Some(position) = nums[..nums.len() - 1]
        .iter()
        .rposition(|&num| num == nums[nums.len() - 1])
    {
        nums.push(nums.len() - (position + 1));
    } else {
        nums.push(0);
    }
}

#[derive(Debug)]
struct Game {
    turn: usize,
    last_num: usize,
    prev_nums: HashMap<usize, usize>,
}

impl Game {
    fn new(input: &str) -> Result<Self> {
        let mut nums = input
            .split(",")
            .enumerate()
            .map(|(position, num)| Ok((num.trim().parse()?, position + 1)))
            .collect::<Result<Vec<(usize, usize)>>>()?;
        let (last_num, turn) = nums.pop().ok_or_else(|| anyhow!("input is empty"))?;
        let prev_nums = nums.into_iter().collect();
        Ok(Self {
            turn,
            last_num,
            prev_nums,
        })
    }

    fn take_turn(&mut self) {
        self.turn += 1;
        self.last_num = match self.prev_nums.entry(self.last_num) {
            Entry::Occupied(mut entry) => {
                let prev_turn = entry.insert(self.turn - 1);
                self.turn - prev_turn - 1
            }
            Entry::Vacant(entry) => {
                entry.insert(self.turn - 1);
                0
            }
        };
    }
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let mut file = File::open(input_path)?;
    let mut nums = String::new();
    file.read_to_string(&mut nums)?;
    let mut nums: Vec<usize> = nums
        .split(",")
        .map(|num| Ok(num.trim().parse()?))
        .collect::<Result<Vec<usize>>>()?;
    let num_turns = 2020 - nums.len();
    for _ in 0..num_turns {
        take_turn(&mut nums);
    }
    Ok(nums.pop().expect("non-empty nums"))
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let mut file = File::open(input_path)?;
    let mut nums = String::new();
    file.read_to_string(&mut nums)?;
    let mut game = Game::new(&nums)?;
    let num_turns = 30000000 - game.turn;
    for _ in 0..num_turns {
        game.take_turn();
    }
    Ok(game.last_num)
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 436);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 175594);
    }
}
