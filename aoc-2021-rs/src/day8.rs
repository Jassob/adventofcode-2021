use std::{fs, iter};

use structopt::StructOpt;

use aoc_2021_rs::{Error, Part};

#[derive(StructOpt, Debug)]
#[structopt(name = "day8", about = "Solve day 8 of Advent Of Code 2021.")]
struct Opt {
    /// Select which part to run (1 or 2)
    #[structopt(short, long, default_value = "1")]
    part: Part,
    /// Select which input file to use
    #[structopt(short, long, default_value = "data/day8.txt")]
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

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| -> Box<dyn Iterator<Item = &str>> {
            match l.split("|").collect::<Vec<_>>()[..] {
                [_, outputs] => Box::new(outputs.split_whitespace()),
                _ => Box::new(iter::empty()),
            }
        })
        .fold(0, |acc, l| acc + nof_unique_signals(l))
}

fn nof_unique_signals<I: Iterator<Item = S>, S: AsRef<str>>(signals: I) -> u32 {
    const NOF_SEGMENTS_1: usize = 2;
    const NOF_SEGMENTS_4: usize = 4;
    const NOF_SEGMENTS_7: usize = 3;
    const NOF_SEGMENTS_8: usize = 7;

    signals
        .map(|s| {
            let len = s.as_ref().chars().count();
            match len {
                NOF_SEGMENTS_1 | NOF_SEGMENTS_4 | NOF_SEGMENTS_7 | NOF_SEGMENTS_8 => 1,
                _ => 0,
            }
        })
        .sum()
}

fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_1() {
        assert_eq!(part1(INPUT), 26)
    }

    #[test]
    fn test_2() {
        todo!()
    }
}
