use std::fs;

use structopt::StructOpt;

use aoc_2021_rs::{Error, Part};

#[derive(StructOpt, Debug)]
#[structopt(name = "day3", about = "Solve day 3 of Advent Of Code 2021.")]
struct Opt {
    /// Select which part to run (1 or 2)
    #[structopt(short, long, default_value = "1")]
    part: Part,
    /// Select which input file to use
    #[structopt(short, long, default_value = "data/day3.txt")]
    input: String,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let input = fs::read_to_string(&opt.input)
        .map_err(|e| Error::ReadInput(opt.input.clone(), Box::new(e)))?;
    let result = match opt.part {
        Part::Part1 => part1(&input),
        Part::Part2 => part2(&input),
    };
    println!("{}", result);
    Ok(())
}

fn part1(input: &str) -> u64 {
    let nums: Vec<u64> = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|d| d.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .reduce(|mut acc, line| {
            for i in 0..acc.len() {
                acc[i] += line[i];
            }
            acc
        })
        .unwrap();
    let len = input.lines().count() as u64;
    let mut gamma = 0u64;
    let mut epsilon = 0u64;
    for i in 0..nums.len() {
        if *nums.iter().rev().skip(i).next().unwrap() >= len / 2 {
            gamma += 2u64.pow(i as u32);
        } else {
            epsilon += 2u64.pow(i as u32);
        }
    }
    gamma * epsilon
}

fn part2(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn test_1() {
        let result = part1(INPUT);
        assert_eq!(result, 198);
    }

    #[test]
    fn test_2() {
        todo!()
    }
}
