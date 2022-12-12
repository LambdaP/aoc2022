use crate::{Aoc, Day01, Display, Result};

impl Aoc for Day01 {
    fn part1(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        result!(parse(lines)?.into_iter().max().unwrap())
    }
    fn part2(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let mut elves: Vec<u32> = parse(lines)?;
        elves.sort_unstable();
        result!(elves.into_iter().rev().take(3).sum::<u32>())
    }
}

fn parse(lines: &[&[u8]]) -> Result<Vec<u32>> {
    lines
        .split(|&l| l.is_empty())
        .map(|ll| {
            ll.iter()
                .map::<Result<u32>, _>(|l| {
                    let s = std::str::from_utf8(l)?;
                    Ok(s.parse()?)
                })
                .sum()
        })
        .collect()
}
