use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Day(u8);

impl Day {
    pub fn into_index(self) -> usize {
        self.0 as usize - 1
    }
}

impl TryFrom<usize> for Day {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if (1..=25).contains(&value) {
            Ok(Self(value as u8))
        } else {
            anyhow::bail!("{value} is not a valid day")
        }
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            anyhow::bail!("{value} is not a valid day")
        } else {
            Day::try_from(u8::from_str(value)? as usize)
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Part {
    A,
    B,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => f.write_str("A"),
            Self::B => f.write_str("B"),
        }
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let bytes = value.as_bytes();

        match bytes.first().cloned().and_then(parse_part) {
            Some(part) if bytes.len() == 1 => Ok(part),
            _ => anyhow::bail!("'{value}' is not a valid part"),
        }
    }
}

fn parse_part(value: u8) -> Option<Part> {
    match value {
        b'A' | b'a' => Some(Part::A),
        b'B' | b'b' => Some(Part::B),
        _ => None,
    }
}
