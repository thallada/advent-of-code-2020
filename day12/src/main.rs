use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Error, Result};

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Coordinate {
    fn rotate(&self, instruction: &NavInstruction) -> Self {
        let mut degrees = instruction.value;
        let mut coordinate = *self;
        while degrees > 0 {
            coordinate = match instruction.action {
                Action::Left => Self {
                    x: coordinate.y,
                    y: coordinate.x * -1,
                },
                Action::Right => Self {
                    x: coordinate.y * -1,
                    y: coordinate.x,
                },
                _ => return coordinate,
            };
            degrees -= 90;
        }
        coordinate
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl TryFrom<char> for Action {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            'N' => Ok(Action::North),
            'S' => Ok(Action::South),
            'E' => Ok(Action::East),
            'W' => Ok(Action::West),
            'L' => Ok(Action::Left),
            'R' => Ok(Action::Right),
            'F' => Ok(Action::Forward),
            _ => Err(anyhow!("Unrecognized action character: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct NavInstruction {
    action: Action,
    value: i32,
}

impl FromStr for NavInstruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (action, value) = s.split_at(1);
        let action = Action::try_from(
            action
                .chars()
                .next()
                .ok_or_else(|| anyhow!("No action char"))?,
        )?;
        let value = value.parse()?;
        Ok(Self { action, value })
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Bearing {
    North,
    South,
    East,
    West,
}

impl Bearing {
    fn rotate(&self, instruction: &NavInstruction) -> Self {
        let mut degrees = instruction.value;
        let mut bearing = *self;
        while degrees > 0 {
            bearing = match instruction.action {
                Action::Left => match bearing {
                    Bearing::North => Bearing::West,
                    Bearing::South => Bearing::East,
                    Bearing::East => Bearing::North,
                    Bearing::West => Bearing::South,
                },
                Action::Right => match bearing {
                    Bearing::North => Bearing::East,
                    Bearing::South => Bearing::West,
                    Bearing::East => Bearing::South,
                    Bearing::West => Bearing::North,
                },
                _ => return bearing,
            };
            degrees -= 90;
        }
        bearing
    }
}

#[derive(Debug, PartialEq)]
struct Ship {
    bearing: Bearing,
    position: Coordinate,
    waypoint: Coordinate,
}

impl Ship {
    fn new() -> Self {
        Self {
            bearing: Bearing::East,
            position: Coordinate { x: 0, y: 0 },
            waypoint: Coordinate { x: 10, y: -1 },
        }
    }

    fn apply_instruction(&mut self, instruction: &NavInstruction) {
        match instruction.action {
            Action::Left | Action::Right => self.bearing = self.bearing.rotate(instruction),
            Action::North => self.position.y -= instruction.value,
            Action::South => self.position.y += instruction.value,
            Action::East => self.position.x += instruction.value,
            Action::West => self.position.x -= instruction.value,
            Action::Forward => {
                match self.bearing {
                    Bearing::North => self.position.y -= instruction.value,
                    Bearing::South => self.position.y += instruction.value,
                    Bearing::East => self.position.x += instruction.value,
                    Bearing::West => self.position.x -= instruction.value,
                };
            }
        }
    }

    fn apply_waypoint_instruction(&mut self, instruction: &NavInstruction) {
        match instruction.action {
            Action::Left | Action::Right => self.waypoint = self.waypoint.rotate(instruction),
            Action::North => self.waypoint.y -= instruction.value,
            Action::South => self.waypoint.y += instruction.value,
            Action::East => self.waypoint.x += instruction.value,
            Action::West => self.waypoint.x -= instruction.value,
            Action::Forward => {
                let mut value = instruction.value;
                while value > 0 {
                    self.position += self.waypoint;
                    value -= 1;
                }
            }
        }
    }
}

fn solve_part1(input_path: &str) -> Result<i32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut ship = Ship::new();
    for line in reader.lines() {
        let instruction = NavInstruction::from_str(&line?)?;
        ship.apply_instruction(&instruction);
    }
    Ok(ship.position.x.abs() + ship.position.y.abs())
}

fn solve_part2(input_path: &str) -> Result<i32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut ship = Ship::new();
    for line in reader.lines() {
        let instruction = NavInstruction::from_str(&line?)?;
        ship.apply_waypoint_instruction(&instruction);
    }
    Ok(ship.position.x.abs() + ship.position.y.abs())
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
        let instructions: Vec<NavInstruction> = reader
            .lines()
            .map(|line| NavInstruction::from_str(&line.unwrap()).unwrap())
            .collect();

        assert_eq!(
            instructions,
            vec![
                NavInstruction {
                    action: Action::Forward,
                    value: 10,
                },
                NavInstruction {
                    action: Action::North,
                    value: 3,
                },
                NavInstruction {
                    action: Action::Forward,
                    value: 7,
                },
                NavInstruction {
                    action: Action::Right,
                    value: 90,
                },
                NavInstruction {
                    action: Action::Forward,
                    value: 11,
                },
            ]
        );
    }

    #[test]
    fn rotates_ship() {
        let mut ship = Ship::new();
        ship.apply_instruction(&NavInstruction {
            action: Action::Right,
            value: 270,
        });
        assert_eq!(ship.bearing, Bearing::North);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 25);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 286);
    }
}
