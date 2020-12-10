use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Error, Result};

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq)]
struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

impl FromStr for Seat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let row = s.chars().take(7).try_fold(0, |row, c| match c {
            'F' => Ok(row << 1),
            'B' => Ok(row << 1 | 1),
            _ => Err(anyhow!("Unrecognized row character: {}", c)),
        })?;
        let col = s.chars().skip(7).take(3).try_fold(0, |col, c| match c {
            'L' => Ok(col << 1),
            'R' => Ok(col << 1 | 1),
            _ => Err(anyhow!("Unrecognized col character: {}", c)),
        })?;
        Ok(Self { row, col })
    }
}

fn solve_part1(input_path: &str) -> Result<u32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| Seat::from_str(&line.unwrap()).unwrap())
        .map(|seat| seat.id())
        .max()
        .ok_or_else(|| anyhow!("No seats found in input"))?)
}

fn solve_part2(input_path: &str) -> Result<u32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut seat_ids: Vec<u32> = reader
        .lines()
        .map(|line| Seat::from_str(&line.unwrap()).unwrap())
        .map(|seat| seat.id())
        .collect();
    seat_ids.sort_unstable();

    let mut prev_seat = None;
    for seat_id in seat_ids {
        if let Some(prev_seat) = prev_seat {
            if seat_id != prev_seat + 1 {
                return Ok(prev_seat + 1);
            }
        }
        prev_seat = Some(seat_id);
    }

    Err(anyhow!("No missing seat was found in input"))
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
        let seats: Vec<Seat> = reader
            .lines()
            .map(|line| Seat::from_str(&line.unwrap()))
            .collect::<Result<Vec<Seat>>>()
            .unwrap();
        assert_eq!(seats[0], Seat { row: 70, col: 7 });
        assert_eq!(seats[0].id(), 567);
        assert_eq!(seats[1], Seat { row: 14, col: 7 });
        assert_eq!(seats[1].id(), 119);
        assert_eq!(seats[2], Seat { row: 102, col: 4 });
        assert_eq!(seats[2].id(), 820);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 820);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 120);
    }
}
