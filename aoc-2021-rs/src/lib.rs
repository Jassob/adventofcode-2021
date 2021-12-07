use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum Error {
    ReadInput(String, Box<dyn std::error::Error + 'static>),
    ParseInput(Box<dyn std::error::Error + 'static>),
    Message(String),
    Todo,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadInput(file, e) => {
                f.write_str(&format!("failed to read input file {}: {}", file, e))
            }
            Self::ParseInput(e) => f.write_str(&format!("failed to parse input file: {}", e)),
            Self::Todo => f.write_str("not done yet"),
            Self::Message(msg) => f.write_str(msg),
        }
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Message(s.into())
    }
}

pub fn parse_lines_as_nums<U: FromStr>(s: &str) -> Result<Vec<U>, Error>
where
    U::Err: std::error::Error + 'static,
{
    s.trim()
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<U>, _>>()
        .map_err(|e| Error::ParseInput(Box::new(e)))
}

pub fn parse_str_as_nums<U: FromStr>(s: &str) -> Result<Vec<U>, Error>
where
    U::Err: std::error::Error + 'static,
{
    s.trim()
        .split(",")
        .map(|n| n.parse())
        .collect::<Result<Vec<U>, _>>()
        .map_err(|e| Error::ParseInput(Box::new(e)))
}

#[derive(Debug)]
pub enum Part {
    Part1,
    Part2,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .parse::<u8>()
            .map_err(|e| format!("failed to parse {} as a number: {}", s, e))?
        {
            1 => Ok(Part::Part1),
            2 => Ok(Part::Part2),
            _ => Err("failed to parse {} as part, expected \"1\" or \"2\"".into()),
        }
    }
}
