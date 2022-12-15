use color_eyre::eyre::*;
use std::fmt::Display;
use std::time::Duration;

macro_rules! result {
    ($res: expr) => {
        Ok(Box::new($res))
    };
}

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
mod day12;
mod day13;
mod day14;
mod day15;

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
pub struct Day12;
pub struct Day13;
pub struct Day14;
pub struct Day15;

pub trait Aoc {
    fn part1(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>>;
    fn part2(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>>;
    fn run<P: AsRef<std::path::Path>>(&self, fpath: P) -> Result<()> {
        let input = std::fs::read(fpath)?;

        let lines = byte_lines(&input);

        let t0 = std::time::SystemTime::now();
        let res1 = self.part1(&lines);
        let t1 = t0.elapsed();
        let t0 = std::time::SystemTime::now();
        let res2 = self.part2(&lines);
        let t2 = t0.elapsed();

        println!("part 1: {} ({:?})", res1?, t1?);
        println!("part 2: {} ({:?})", res2?, t2?);

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
