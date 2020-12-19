extern crate nom;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1 as digit, space0 as space},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Instant;

const INPUT: &str = "input/input.txt";

// From: https://github.com/Geal/nom/blob/master/tests/arithmetic.rs
fn flat_parens(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), flat_expr, tag(")")), space)(i)
}

fn flat_factor(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        flat_parens,
    ))(i)
}

fn flat_expr(i: &str) -> IResult<&str, i64> {
    let (i, init) = flat_factor(i)?;

    fold_many0(
        pair(alt((char('+'), char('*'))), flat_factor),
        init,
        |acc, (op, val): (char, i64)| {
            if op == '+' {
                acc + val
            } else if op == '*' {
                acc * val
            } else {
                acc
            }
        },
    )(i)
}

fn precedent_parens(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), precedent_expr, tag(")")), space)(i)
}

fn precedent_factor(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        precedent_parens,
    ))(i)
}

fn sum(i: &str) -> IResult<&str, i64> {
    let (i, init) = precedent_factor(i)?;

    fold_many0(
        pair(char('+'), precedent_factor),
        init,
        |acc, (op, val): (char, i64)| {
            if op == '+' {
                acc + val
            } else {
                acc
            }
        },
    )(i)
}

fn precedent_expr(i: &str) -> IResult<&str, i64> {
    let (i, init) = sum(i)?;

    fold_many0(pair(char('*'), sum), init, |acc, (op, val): (char, i64)| {
        if op == '*' {
            acc * val
        } else {
            acc
        }
    })(i)
}

fn solve_part1(input_path: &str) -> Result<i64> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| flat_expr(&line.unwrap()).unwrap().1)
        .sum::<i64>())
}

fn solve_part2(input_path: &str) -> Result<i64> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| precedent_expr(&line.unwrap()).unwrap().1)
        .sum::<i64>())
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

    #[test]
    fn solves_part1() {
        assert_eq!(flat_expr("1 + 2 * 3 + 4 * 5 + 6"), Ok(("", 71)));
        assert_eq!(flat_expr("1 + (2 * 3) + (4 * (5 + 6))"), Ok(("", 51)));
        assert_eq!(flat_expr("2 * 3 + (4 * 5)"), Ok(("", 26)));
        assert_eq!(flat_expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(("", 437)));
        assert_eq!(
            flat_expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(("", 12240))
        );
        assert_eq!(
            flat_expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(("", 13632))
        );
    }

    #[test]
    fn solves_part2() {
        assert_eq!(precedent_expr("1 + 2 * 3 + 4 * 5 + 6"), Ok(("", 231)));
        assert_eq!(precedent_expr("1 + (2 * 3) + (4 * (5 + 6))"), Ok(("", 51)));
        assert_eq!(precedent_expr("2 * 3 + (4 * 5)"), Ok(("", 46)));
        assert_eq!(
            precedent_expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
            Ok(("", 1445))
        );
        assert_eq!(
            precedent_expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(("", 669060))
        );
        assert_eq!(
            precedent_expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(("", 23340))
        );
    }
}
