use color_eyre::eyre::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

pub struct Day1;
pub struct Day2;
pub struct Day3;
pub struct Day4;
pub struct Day5;
pub struct Day6;
pub struct Day7;
pub struct Day8;
pub struct Day9;
pub struct Day10;
pub struct Day11;

pub trait Aoc<T>
where
    T: std::fmt::Display,
{
    fn part1(&self, lines: &[&[u8]]) -> Result<T>;
    fn part2(&self, lines: &[&[u8]]) -> Result<T>;
    fn run<P: AsRef<std::path::Path>>(&self, fpath: P) -> Result<()> {
        let input = std::fs::read(fpath)?;

        let lines = byte_lines(&input);

        let res1 = self.part1(&lines);
        let res2 = self.part2(&lines);

        println!("part 1: {}", res1?);
        println!("part 2: {}", res2?);

        Ok(())
    }
}

fn byte_lines(input: &[u8]) -> Vec<&[u8]> {
    input
        .strip_suffix(&[b'\n'])
        .unwrap_or(input)
        .split(|b| *b == b'\n')
        .collect()
}
