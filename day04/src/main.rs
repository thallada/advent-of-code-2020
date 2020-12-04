use anyhow::{anyhow, Context, Error, Result};

use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq, Default)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut passport_string = String::new();
        if let Some(birth_year) = &self.birth_year {
            passport_string.push_str(&format!("byr:{} ", birth_year));
        }
        if let Some(issue_year) = &self.issue_year {
            passport_string.push_str(&format!("iyr:{} ", issue_year));
        }
        if let Some(expiration_year) = &self.expiration_year {
            passport_string.push_str(&format!("eyr:{} ", expiration_year));
        }
        if let Some(height) = &self.height {
            passport_string.push_str(&format!("hgt:{} ", height));
        }
        if let Some(hair_color) = &self.hair_color {
            passport_string.push_str(&format!("hcl:{} ", hair_color));
        }
        if let Some(eye_color) = &self.eye_color {
            passport_string.push_str(&format!("ecl:{} ", eye_color));
        }
        if let Some(passport_id) = &self.passport_id {
            passport_string.push_str(&format!("pid:{} ", passport_id));
        }
        if let Some(country_id) = &self.country_id {
            passport_string.push_str(&format!("cid:{} ", country_id));
        }
        writeln!(f, "{}", passport_string.trim_end())
    }
}

impl FromStr for Passport {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut passport = Self::default();
        let fields = s.split_whitespace();
        for field in fields {
            let mut field = field.split(":");
            let name = field.next().context("Failed to parse field name")?;
            let value = field.next().context("Failed to parse field value")?;
            match name {
                "byr" => passport.birth_year = Some(value.to_owned()),
                "iyr" => passport.issue_year = Some(value.to_owned()),
                "eyr" => passport.expiration_year = Some(value.to_owned()),
                "hgt" => passport.height = Some(value.to_owned()),
                "hcl" => passport.hair_color = Some(value.to_owned()),
                "ecl" => passport.eye_color = Some(value.to_owned()),
                "pid" => passport.passport_id = Some(value.to_owned()),
                "cid" => passport.country_id = Some(value.to_owned()),
                _ => return Err(anyhow!("Unrecognized field name: {}", name)),
            }
        }
        Ok(passport)
    }
}

impl Passport {
    fn from_reader<R: BufRead>(reader: R) -> Result<Vec<Self>> {
        let mut passport_buf = String::new();
        let mut passports: Vec<Passport> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() && !passport_buf.is_empty() {
                passports.push(passport_buf.parse()?);
                passport_buf.truncate(0);
            } else {
                passport_buf.push_str(" ");
                passport_buf.push_str(&line);
            }
        }
        if !passport_buf.is_empty() {
            passports.push(passport_buf.parse()?);
        }
        Ok(passports)
    }

    fn validate(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn validate_birth_year(&self) -> bool {
        match &self.birth_year {
            Some(birth_year) => {
                matches!(birth_year.parse::<u32>(), Ok(year) if (1920..=2002).contains(&year))
            }
            None => false,
        }
    }

    fn validate_issue_year(&self) -> bool {
        match &self.issue_year {
            Some(issue_year) => {
                matches!(issue_year.parse::<u32>(), Ok(year) if (2010..=2020).contains(&year))
            }
            None => false,
        }
    }

    fn validate_expiration_year(&self) -> bool {
        match &self.expiration_year {
            Some(expiration_year) => {
                matches!(expiration_year.parse::<u32>(), Ok(year) if (2020..=2030).contains(&year))
            }
            None => false,
        }
    }

    fn validate_height(&self) -> bool {
        match &self.height {
            Some(height) => {
                if let Some(height) = height.strip_suffix("cm") {
                    matches!(height.parse::<u32>(), Ok(height) if (150..=193).contains(&height))
                } else if let Some(height) = height.strip_suffix("in") {
                    matches!(height.parse::<u32>(), Ok(height) if (59..=76).contains(&height))
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn validate_hair_color(&self) -> bool {
        match &self.hair_color {
            Some(hair_color) => {
                hair_color.starts_with("#")
                    && hair_color.chars().skip(1).all(|c| c.is_ascii_hexdigit())
            }
            None => false,
        }
    }

    fn validate_eye_color(&self) -> bool {
        match &self.eye_color {
            Some(eye_color) => {
                matches!(
                    eye_color.as_str(),
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                )
            }
            None => false,
        }
    }

    fn validate_passport_id(&self) -> bool {
        match &self.passport_id {
            Some(passport_id) => {
                passport_id.len() == 9 && passport_id.chars().all(|c| c.is_ascii_digit())
            }
            None => false,
        }
    }

    fn strict_validate(&self) -> bool {
        self.validate_birth_year()
            && self.validate_issue_year()
            && self.validate_expiration_year()
            && self.validate_height()
            && self.validate_hair_color()
            && self.validate_eye_color()
            && self.validate_passport_id()
    }
}

fn solve_part1(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let passports = Passport::from_reader(reader)?;

    Ok(passports
        .iter()
        .filter(|passport| passport.validate())
        .count())
}

fn solve_part2(input_path: &str) -> Result<usize> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let passports = Passport::from_reader(reader)?;

    Ok(passports
        .iter()
        .filter(|passport| passport.strict_validate())
        .count())
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT).unwrap());
    println!("Part 2: {}", solve_part2(INPUT).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "input/test1.txt";
    const TEST_INPUT2: &str = "input/test2.txt";

    #[test]
    fn parses_input() {
        let file = File::open(TEST_INPUT1).unwrap();
        let reader = BufReader::new(file);
        let passports = Passport::from_reader(reader).unwrap();

        assert_eq!(passports.len(), 4);
        assert_eq!(
            format!("{}", passports[0]),
            "byr:1937 iyr:2017 eyr:2020 hgt:183cm hcl:#fffffd ecl:gry pid:860033327 cid:147\n"
        );
        assert_eq!(
            format!("{}", passports[1]),
            "byr:1929 iyr:2013 eyr:2023 hcl:#cfa07d ecl:amb pid:028048884 cid:350\n"
        );
        assert_eq!(
            format!("{}", passports[2]),
            "byr:1931 iyr:2013 eyr:2024 hgt:179cm hcl:#ae17e1 ecl:brn pid:760753108\n"
        );
        assert_eq!(
            format!("{}", passports[3]),
            "iyr:2011 eyr:2025 hgt:59in hcl:#cfa07d ecl:brn pid:166559648\n"
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT1).unwrap(), 2);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT2).unwrap(), 4);
    }
}
