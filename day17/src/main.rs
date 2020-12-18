#[macro_use]
extern crate lazy_static;

use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::time::Instant;

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Add for Coordinate {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

fn adjacent_vectors3() -> [Coordinate; 26] {
    let mut i = 0;
    let mut vectors: [Coordinate; 26] = [Coordinate {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
    }; 26];
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                if !(x == 0 && y == 0 && z == 0) {
                    vectors[i].x = x;
                    vectors[i].y = y;
                    vectors[i].z = z;
                    i += 1;
                }
            }
        }
    }
    vectors
}

fn adjacent_vectors4() -> [Coordinate; 80] {
    let mut i = 0;
    let mut vectors: [Coordinate; 80] = [Coordinate {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
    }; 80];
    for w in -1..=1 {
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if !(x == 0 && y == 0 && z == 0 && w == 0) {
                        vectors[i].x = x;
                        vectors[i].y = y;
                        vectors[i].z = z;
                        vectors[i].w = w;
                        i += 1;
                    }
                }
            }
        }
    }
    vectors
}

lazy_static! {
    static ref ADJACENT_VECTORS3: [Coordinate; 26] = adjacent_vectors3();
    static ref ADJACENT_VECTORS4: [Coordinate; 80] = adjacent_vectors4();
}

#[derive(Clone)]
enum Cube {
    Active,
    Inactive,
}

impl TryFrom<char> for Cube {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '#' => Ok(Cube::Active),
            '.' => Ok(Cube::Inactive),
            _ => Err(anyhow!("Unrecognized cube character: {}", c)),
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cube::Active => write!(f, "#"),
            Cube::Inactive => write!(f, "."),
        }
    }
}

impl Cube {
    fn is_active(&self) -> bool {
        match self {
            Cube::Active => true,
            Cube::Inactive => false,
        }
    }
}

struct Grid {
    cubes: HashMap<Coordinate, Cube>,
    bounds: Bounds,
}

struct Bounds {
    min: Coordinate,
    max: Coordinate,
}

impl Grid {
    fn from_reader<R: BufRead>(reader: R) -> Result<Self> {
        let mut cubes = HashMap::new();
        let mut max = Coordinate {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        };
        for (y, line) in reader.lines().enumerate() {
            for (x, c) in line?.chars().enumerate() {
                cubes.insert(
                    Coordinate {
                        x: x as isize,
                        y: y as isize,
                        z: 0,
                        w: 0,
                    },
                    Cube::try_from(c)?,
                );
                max.x = x as isize;
            }
            max.y = y as isize;
        }
        let bounds = Bounds {
            min: Coordinate {
                x: 0,
                y: 0,
                z: 0,
                w: 0,
            },
            max,
        };
        Ok(Self { cubes, bounds })
    }

    fn increase_bounds(&mut self) {
        self.bounds.min.x -= 1;
        self.bounds.min.y -= 1;
        self.bounds.min.z -= 1;
        self.bounds.max.x += 1;
        self.bounds.max.y += 1;
        self.bounds.max.z += 1;
    }

    fn increase_hypercube_bounds(&mut self) {
        self.bounds.min.x -= 1;
        self.bounds.min.y -= 1;
        self.bounds.min.z -= 1;
        self.bounds.min.w -= 1;
        self.bounds.max.x += 1;
        self.bounds.max.y += 1;
        self.bounds.max.z += 1;
        self.bounds.max.w += 1;
    }

    fn get_active_neighbors(&self, coord: &Coordinate) -> usize {
        ADJACENT_VECTORS3
            .iter()
            .filter_map(|vector| {
                let neighbor_coord = *coord + *vector;
                match self.cubes.get(&neighbor_coord) {
                    Some(cube) if cube.is_active() => Some(()),
                    _ => None,
                }
            })
            .count()
    }

    fn new_cube_state(cube: Option<&Cube>, active_neighbors: usize) -> Cube {
        match cube {
            Some(Cube::Active) => {
                if active_neighbors == 2 || active_neighbors == 3 {
                    Cube::Active
                } else {
                    Cube::Inactive
                }
            }
            _ => {
                if active_neighbors == 3 {
                    Cube::Active
                } else {
                    Cube::Inactive
                }
            }
        }
    }

    fn run_cycle(&mut self) {
        self.increase_bounds();
        let mut new_cubes = HashMap::new();
        for z in self.bounds.min.z..=self.bounds.max.z {
            for y in self.bounds.min.y..=self.bounds.max.y {
                for x in self.bounds.min.x..=self.bounds.max.x {
                    let coord = Coordinate { x, y, z, w: 0 };
                    let cube = self.cubes.get(&coord);
                    let active_neighbors = self.get_active_neighbors(&coord);
                    new_cubes.insert(coord, Grid::new_cube_state(cube, active_neighbors));
                }
            }
        }
        self.cubes = new_cubes;
    }

    fn get_active_hypercube_neighbors(&self, coord: &Coordinate) -> usize {
        ADJACENT_VECTORS4
            .iter()
            .filter_map(|vector| {
                let neighbor_coord = *coord + *vector;
                match self.cubes.get(&neighbor_coord) {
                    Some(cube) if cube.is_active() => Some(()),
                    _ => None,
                }
            })
            .count()
    }

    fn run_hypercube_cycle(&mut self) {
        self.increase_hypercube_bounds();
        let mut new_cubes = HashMap::new();
        for w in self.bounds.min.w..=self.bounds.max.w {
            for z in self.bounds.min.z..=self.bounds.max.z {
                for y in self.bounds.min.y..=self.bounds.max.y {
                    for x in self.bounds.min.x..=self.bounds.max.x {
                        let coord = Coordinate { x, y, z, w };
                        let active_neighbors = self.get_active_hypercube_neighbors(&coord);
                        let cube = self.cubes.get(&coord);
                        new_cubes.insert(coord, Grid::new_cube_state(cube, active_neighbors));
                    }
                }
            }
        }
        self.cubes = new_cubes;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for w in self.bounds.min.w..=self.bounds.max.w {
            for z in self.bounds.min.z..=self.bounds.max.z {
                writeln!(f, "z={} w={}", z, w)?;
                for y in self.bounds.min.y..=self.bounds.max.y {
                    for x in self.bounds.min.x..=self.bounds.max.x {
                        let cube = self
                            .cubes
                            .get(&Coordinate { x, y, z, w })
                            .expect("cube to exist within bounds");
                        write!(f, "{}", &cube.to_string())?
                    }
                    writeln!(f, "")?;
                }
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut grid = Grid::from_reader(reader)?;

    for _ in 0..6 {
        grid.run_cycle();
    }

    Ok(grid.cubes.values().filter(|cube| cube.is_active()).count())
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut grid = Grid::from_reader(reader)?;

    for _ in 0..6 {
        grid.run_hypercube_cycle();
    }

    Ok(grid.cubes.values().filter(|cube| cube.is_active()).count())
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
        let grid = Grid::from_reader(reader).unwrap();

        assert_eq!(
            grid.to_string(),
            r#"
z=0 w=0
.#.
..#
###

"#
            .trim_start()
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 112);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 848);
    }
}
