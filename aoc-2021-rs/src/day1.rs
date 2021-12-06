use std::fs;

use structopt::StructOpt;

use aoc_2021_rs::{self as lib, Error, Part};

#[derive(StructOpt, Debug)]
#[structopt(name = "day1", about = "Solve day 1 of Advent Of Code 2021.")]
struct Opt {
    /// Select which part to run (1 or 2)
    #[structopt(short, long, default_value = "1")]
    part: Part,
    /// Select which input file to use
    #[structopt(short, long, default_value = "data/day1.txt")]
    input: String,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let input = fs::read_to_string(&opt.input)
        .map_err(|e| Error::ReadInput(opt.input.clone(), Box::new(e)))?;
    let result = match opt.part {
        Part::Part1 => lib::parse_lines_as_nums(&input).map(|nums| part1(&nums)),
        Part::Part2 => lib::parse_lines_as_nums(&input).map(|nums| part2(&nums)),
    }?;
    println!("{}", result);
    Ok(())
}

fn part1(input: &[u32]) -> u32 {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(x, y)| x < y)
        .count() as u32
}

fn part2(input: &[u32]) -> u32 {
    if input.len() <= 3 {
        return 0;
    }
    let mut x = input[0];
    let mut y = input[1];
    let mut z = input[2];
    let mut increases = 0;
    for v in input.iter().skip(3) {
        if v > &x {
            increases += 1;
        }
        x = y;
        y = z;
        z = *v;
    }
    increases
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_1() {
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2(INPUT), 5);
    }
}
