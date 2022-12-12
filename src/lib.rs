use color_eyre::eyre::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

pub struct Day01;
pub struct Day02;
pub struct Day03;
pub struct Day04;
pub struct Day05;
pub struct Day06;
pub struct Day07;
pub struct Day08;
pub struct Day09;
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
