use anyhow::Result;

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Tree,
    Empty,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Tree,
            _ => Self::Empty,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tree => write!(f, "#"),
            Self::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Slope {
    topography: Vec<Vec<Cell>>,
}

impl fmt::Display for Slope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.topography {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|cell| format!("{}", cell))
                    .collect::<Vec<String>>()
                    .join("")
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct Vector {
    x: usize,
    y: usize,
}

impl Slope {
    fn get_cell(&self, position: &Vector) -> Cell {
        let row = &self.topography[position.y];
        row[position.x % row.len()]
    }

    fn trees_in_traversal(&self, descent: &Vector) -> usize {
        let mut position = Vector { x: 0, y: 0 };
        let mut tree_count = match self.get_cell(&position) {
            Cell::Tree => 1,
            _ => 0,
        };
        while position.y < self.topography.len() - 1 {
            position.x += descent.x;
            position.y += descent.y;
            tree_count += match self.get_cell(&position) {
                Cell::Tree => 1,
                _ => 0,
            };
        }
        tree_count
    }
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let slope = Slope {
        topography: reader
            .lines()
            .map(|line| line.unwrap().chars().map(Cell::from).collect::<Vec<Cell>>())
            .collect(),
    };
    Ok(slope.trees_in_traversal(&Vector { x: 3, y: 1 }))
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let slope = Slope {
        topography: reader
            .lines()
            .map(|line| line.unwrap().chars().map(Cell::from).collect::<Vec<Cell>>())
            .collect(),
    };
    let descents = vec![
        Vector { x: 1, y: 1 },
        Vector { x: 3, y: 1 },
        Vector { x: 5, y: 1 },
        Vector { x: 7, y: 1 },
        Vector { x: 1, y: 2 },
    ];
    Ok(descents
        .iter()
        .map(|descent| slope.trees_in_traversal(&descent))
        .product())
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
        let slope = Slope {
            topography: reader
                .lines()
                .map(|line| line.unwrap().chars().map(Cell::from).collect::<Vec<Cell>>())
                .collect(),
        };

        let file = File::open(TEST_INPUT).unwrap();
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        reader.read_to_string(&mut buf).unwrap();

        assert_eq!(format!("{}", slope), buf);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 7);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 336);
    }
}
