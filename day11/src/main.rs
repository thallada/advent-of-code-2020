use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use anyhow::{anyhow, Error, Result};

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq, Copy, Clone)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for Seat {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Seat::Floor),
            'L' => Ok(Seat::Empty),
            '#' => Ok(Seat::Occupied),
            _ => Err(anyhow!("Unrecognized seat character: {}", c)),
        }
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Floor => write!(f, "."),
            Self::Empty => write!(f, "L"),
            Self::Occupied => write!(f, "#"),
        }
    }
}

impl Seat {
    fn is_occupied(&self) -> bool {
        *self == Seat::Occupied
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Vector {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn apply_vector(&self, vector: &Vector) -> Result<Coordinate> {
        let x = vector.x.checked_add(self.x as isize);
        let y = vector.y.checked_add(self.y as isize);
        if let Some(x) = x {
            if let Some(y) = y {
                return Ok(Coordinate {
                    x: x as usize,
                    y: y as usize,
                });
            }
        }
        Err(anyhow!("Applying Vector results in an invalid Coordinate"))
    }
}

const ADJACENT_VECTORS: [Vector; 8] = [
    Vector { x: -1, y: -1 },
    Vector { x: 0, y: -1 },
    Vector { x: 1, y: -1 },
    Vector { x: -1, y: 0 },
    Vector { x: 1, y: 0 },
    Vector { x: -1, y: 1 },
    Vector { x: 0, y: 1 },
    Vector { x: 1, y: 1 },
];

#[derive(Debug, PartialEq)]
struct Grid {
    seats: HashMap<Coordinate, Seat>,
    row_len: usize,
}

impl Grid {
    fn from_reader<R: BufRead>(reader: R) -> Result<Self> {
        let mut row_len = 0;
        let mut seats = HashMap::new();
        let mut y = 0;
        for line in reader.lines() {
            let line = line?;
            if line.len() > row_len {
                row_len = line.len();
            }
            let mut x = 0;
            for c in line.chars() {
                seats.insert(Coordinate { x, y }, Seat::try_from(c)?);
                x += 1;
            }
            y += 1;
        }
        Ok(Self { seats, row_len })
    }

    fn seat_in_sight(&self, coord: &Coordinate, vector: &Vector, recurse: bool) -> Seat {
        if let Ok(check_coord) = coord.apply_vector(vector) {
            if let Some(&seat) = self.seats.get(&check_coord) {
                if recurse && seat == Seat::Floor {
                    return self.seat_in_sight(&check_coord, vector, recurse);
                } else {
                    return seat;
                }
            }
        }
        Seat::Floor
    }

    fn new_seat_state(
        &self,
        coord: &Coordinate,
        seat: &Seat,
        recurse: bool,
        empty_threshold: usize,
    ) -> Seat {
        match seat {
            Seat::Floor => Seat::Floor,
            Seat::Empty => {
                if ADJACENT_VECTORS
                    .iter()
                    .all(|vector| !self.seat_in_sight(coord, vector, recurse).is_occupied())
                {
                    Seat::Occupied
                } else {
                    Seat::Empty
                }
            }
            Seat::Occupied => {
                if ADJACENT_VECTORS
                    .iter()
                    .map(|vector| self.seat_in_sight(coord, vector, recurse).is_occupied() as usize)
                    .sum::<usize>()
                    >= empty_threshold
                {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
        }
    }

    fn run_round(&mut self, recurse: bool, empty_threshold: usize) -> bool {
        let new_seats = self
            .seats
            .iter()
            .map(|(coord, seat)| {
                (
                    *coord,
                    self.new_seat_state(coord, seat, recurse, empty_threshold),
                )
            })
            .collect();
        let changed = new_seats != self.seats;
        self.seats = new_seats;
        changed
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.seats.len() / self.row_len {
            for x in 0..self.row_len {
                write!(
                    f,
                    "{}",
                    self.seats
                        .get(&Coordinate { x, y })
                        .expect("seat exists in Grid bounds"),
                )?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut grid = Grid::from_reader(reader)?;
    while grid.run_round(false, 4) {}
    Ok(grid
        .seats
        .iter()
        .filter(|(_, seat)| seat.is_occupied())
        .count())
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut grid = Grid::from_reader(reader)?;
    while grid.run_round(true, 5) {}
    Ok(grid
        .seats
        .iter()
        .filter(|(_, seat)| seat.is_occupied())
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

    use std::io::prelude::*;

    const TEST_INPUT: &str = "input/test.txt";

    #[test]
    fn parses_input() {
        let file = File::open(TEST_INPUT).unwrap();
        let reader = BufReader::new(file);
        let grid = Grid::from_reader(reader).unwrap();

        let file = File::open(TEST_INPUT).unwrap();
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        reader.read_to_string(&mut buf).unwrap();

        assert_eq!(format!("{}", grid), buf);
    }

    #[test]
    fn runs_rounds_part1() {
        let file = File::open(TEST_INPUT).unwrap();
        let reader = BufReader::new(file);
        let mut grid = Grid::from_reader(reader).unwrap();

        grid.run_round(false, 4);
        let expected = r#"
            #.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);

        grid.run_round(false, 4);
        let expected = r#"
            #.LL.L#.##
            #LLLLLL.L#
            L.L.L..L..
            #LLL.LL.L#
            #.LL.LL.LL
            #.LLLL#.##
            ..L.L.....
            #LLLLLLLL#
            #.LLLLLL.L
            #.#LLLL.##
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);

        grid.run_round(false, 4);
        let expected = r#"
            #.##.L#.##
            #L###LL.L#
            L.#.#..#..
            #L##.##.L#
            #.##.LL.LL
            #.###L#.##
            ..#.#.....
            #L######L#
            #.LL###L.L
            #.#L###.##
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);

        grid.run_round(false, 4);
        let expected = r#"
            #.#L.L#.##
            #LLL#LL.L#
            L.L.L..#..
            #LLL.##.L#
            #.LL.LL.LL
            #.LL#L#.##
            ..L.L.....
            #L#LLLL#L#
            #.LLLLLL.L
            #.#L#L#.##
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);

        grid.run_round(false, 4);
        let expected = r#"
            #.#L.L#.##
            #LLL#LL.L#
            L.#.L..#..
            #L##.##.L#
            #.#L.LL.LL
            #.#L#L#.##
            ..L.L.....
            #L#L##L#L#
            #.LLLLLL.L
            #.#L#L#.##
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);
    }

    #[test]
    fn runs_rounds_part2() {
        let file = File::open(TEST_INPUT).unwrap();
        let reader = BufReader::new(file);
        let mut grid = Grid::from_reader(reader).unwrap();

        grid.run_round(true, 5);
        let expected = r#"
            #.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);

        grid.run_round(true, 5);
        let expected = r#"
            #.LL.LL.L#
            #LLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLL#
            #.LLLLLL.L
            #.LLLLL.L#
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);

        grid.run_round(true, 5);
        let expected = r#"
            #.L#.##.L#
            #L#####.LL
            L.#.#..#..
            ##L#.##.##
            #.##.#L.##
            #.#####.#L
            ..#.#.....
            LLL####LL#
            #.L#####.L
            #.L####.L#
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);

        grid.run_round(true, 5);
        let expected = r#"
            #.L#.L#.L#
            #LLLLLL.LL
            L.L.L..#..
            ##LL.LL.L#
            L.LL.LL.L#
            #.LLLLL.LL
            ..L.L.....
            LLLLLLLLL#
            #.LLLLL#.L
            #.L#LL#.L#
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);

        grid.run_round(true, 5);
        let expected = r#"
            #.L#.L#.L#
            #LLLLLL.LL
            L.L.L..#..
            ##L#.#L.L#
            L.L#.#L.L#
            #.L####.LL
            ..#.#.....
            LLL###LLL#
            #.LLLLL#.L
            #.L#LL#.L#
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);

        grid.run_round(true, 5);
        let expected = r#"
            #.L#.L#.L#
            #LLLLLL.LL
            L.L.L..#..
            ##L#.#L.L#
            L.L#.LL.L#
            #.LLLL#.LL
            ..#.L.....
            LLL###LLL#
            #.LLLLL#.L
            #.L#LL#.L#
        "#
        .trim_start()
        .replace(" ", "");
        assert_eq!(format!("{}", grid), expected);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 37);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 26);
    }
}
