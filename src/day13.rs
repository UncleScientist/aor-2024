use std::error::Error;
use std::fmt::Display;
use std::str::Split;

#[derive(Debug)]
pub enum ParseError {
    NoName,
    NoGoodDeeds,
    NoBadDeeds,
    InvalidGoodDeeds,
    InvalidBadDeeds,
}

// 2. Implement the Error trait for ParseError
impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParseError::NoName => "Name field is missing",
                ParseError::NoGoodDeeds => "Good deeds field is missing",
                ParseError::NoBadDeeds => "Bad deeds field is missing",
                ParseError::InvalidGoodDeeds => "Good deeds value is invalid",
                ParseError::InvalidBadDeeds => "Bad deeds value is invalid",
            }
        )
    }
}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Kid { name, niceness }
    }

    pub fn parse_row(csv_row: &str) -> Result<Kid, ParseError> {
        use ParseError::*;

        let get_int = |i: &mut Split<'_, char>, missing, bad| match i.next() {
            None | Some("") => Err(missing),
            Some(field) => field.parse::<u32>().map_err(|_| bad),
        };

        let mut fields = csv_row.split(',');

        let name = match fields.next() {
            None | Some("") => Err(NoName),
            Some(field) => Ok(field),
        }?
        .to_string();
        let good_deeds = get_int(&mut fields, NoGoodDeeds, InvalidGoodDeeds)?;
        let bad_deeds = get_int(&mut fields, NoBadDeeds, InvalidBadDeeds)?;

        Ok(Kid::new(name, good_deeds, bad_deeds))
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}
