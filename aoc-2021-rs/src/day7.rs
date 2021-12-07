use std::{collections::HashMap, fs};

use structopt::StructOpt;

use aoc_2021_rs::{self as lib, Error, Part};

#[derive(StructOpt, Debug)]
#[structopt(name = "day7", about = "Solve day 7 of Advent Of Code 2021.")]
struct Opt {
    /// Select which part to run (1 or 2)
    #[structopt(short, long, default_value = "1")]
    part: Part,
    /// Select which input file to use
    #[structopt(short, long, default_value = "data/day7.txt")]
    input: String,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let input = fs::read_to_string(&opt.input)
        .map_err(|e| Error::ReadInput(opt.input.clone(), Box::new(e)))?;
    let nums: Vec<u64> = lib::parse_str_as_nums(&input)?;
    let result = match opt.part {
        Part::Part1 => part1(&nums),
        Part::Part2 => part2(&nums),
    }?;
    println!("{}", result);
    Ok(())
}

fn get_closest_distance(nums: &[u64], distance_fn: fn(u64, u64) -> u64) -> Result<u64, Error> {
    let min = nums.iter().min().ok_or(Error::Message(format!(
        "failed to get min value from {:?}",
        nums
    )))?;
    let max = nums.iter().max().ok_or(Error::Message(format!(
        "failed to get max value from {:?}",
        nums
    )))?;
    let mut distances = (*min..*max).map(|i| (i, 0)).collect::<HashMap<u64, u64>>();
    // get distances from every horizontal position for every possible position
    nums.iter().for_each(|n| {
        distances
            .iter_mut()
            .for_each(|(i, v)| *v += distance_fn(*n, *i))
    });
    let closest_point = distances
        .iter()
        .min_by(|t1, t2| t1.1.cmp(t2.1))
        .ok_or(Error::Message(format!(
            "failed to get closest horizontal point from {:?}",
            distances
        )))?;
    Ok(*closest_point.1)
}

fn part1(input: &[u64]) -> Result<u64, Error> {
    get_closest_distance(input, |p1, p2| p1.max(p2) - p1.min(p2))
}

fn part2(input: &[u64]) -> Result<u64, Error> {
    get_closest_distance(input, |p1, p2| {
        let distance = p1.max(p2) - p1.min(p2);
        (0..=distance).sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_1() {
        let nums = lib::parse_str_as_nums(INPUT).unwrap();
        assert_eq!(part1(&nums).expect("should not fail"), 37)
    }

    #[test]
    fn test_2() {
        let nums = lib::parse_str_as_nums(INPUT).unwrap();
        assert_eq!(part2(&nums).expect("should not fail"), 168)
    }
}
