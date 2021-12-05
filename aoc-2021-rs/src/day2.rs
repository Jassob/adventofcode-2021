use std::{fs, str::FromStr};

use structopt::StructOpt;

use aoc_2021_rs::{Error, Part};

#[derive(StructOpt, Debug)]
#[structopt(name = "day2", about = "Solve day 2 of Advent Of Code 2021.")]
struct Opt {
    /// Select which part to run (1 or 2)
    #[structopt(short, long, default_value = "1")]
    part: Part,
    /// Select which input file to use
    #[structopt(short, long, default_value = "data/day2.txt")]
    input: String,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        if words.len() != 2 {
            return Err(format!(
                "failed to parse {} as a direction: expected a string on the pattern \"DIR NUM\"",
                s
            ));
        }
        let amount = words[1]
            .parse()
            .map_err(|e| format!("failed to parse {} as a number: {}", s, e))?;
        match words[0] {
            "forward" => Ok(Self::Forward(amount)),
            "up" => Ok(Self::Up(amount)),
            "down" => Ok(Self::Down(amount)),
            _ => Err(format!("failed to parse {} as a direction, expected one of \"forward\", \"up\" or \"down\"", words[0])),
        }
    }
}

impl Direction {
    fn from_lines(s: &str) -> Result<Vec<Self>, String> {
        s.lines().map(|l| l.parse()).collect()
    }
}

struct Pos {
    horizontal: u32,
    depth: u32,
}

impl Default for Pos {
    fn default() -> Self {
        Pos {
            horizontal: 0,
            depth: 0,
        }
    }
}

impl Pos {
    fn follow_direction(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Forward(x) => Self {
                horizontal: self.horizontal + x,
                ..*self
            },
            Direction::Up(d) => Self {
                depth: self.depth - d,
                ..*self
            },
            Direction::Down(d) => Self {
                depth: self.depth + d,
                ..*self
            },
        }
    }

    fn follow_directions(self, dirs: &[Direction]) -> Self {
        dirs.iter().fold(self, |acc, dir| acc.follow_direction(dir))
    }
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let input = fs::read_to_string(&opt.input)
        .map_err(|e| Error::ReadInput(opt.input.clone(), Box::new(e)))?;
    let dirs = Direction::from_lines(&input).map_err(Error::Message)?;
    let result = match opt.part {
        Part::Part1 => part1(&dirs),
        Part::Part2 => part2(&[]),
    };
    println!("{}", result);
    Ok(())
}

fn part1(input: &[Direction]) -> u32 {
    let end_pos = Pos::default().follow_directions(input);
    end_pos.depth * end_pos.horizontal
}

fn part2(_input: &[u32]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_parse() {
        let expected = vec![
            Direction::Forward(5),
            Direction::Down(5),
            Direction::Forward(8),
            Direction::Up(3),
            Direction::Down(8),
            Direction::Forward(2),
        ];
        let dirs = Direction::from_lines(INPUT);
        assert_eq!(dirs, Ok(expected));
    }

    #[test]
    fn test_1() {
        let dirs = Direction::from_lines(INPUT).expect("should not fail");
        assert_eq!(part1(&dirs), 150)
    }

    #[test]
    fn test_2() {
        todo!()
    }
}
