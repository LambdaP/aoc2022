use crate::{Aoc, Day1, Result};

impl Aoc<u32> for Day1 {
    fn part1(&self, lines: &[&[u8]]) -> Result<u32> {
        Ok(parse(lines)?.into_iter().max().unwrap())
    }
    fn part2(&self, lines: &[&[u8]]) -> Result<u32> {
        let mut elves: Vec<u32> = parse(lines)?;
        elves.sort_unstable();
        Ok(elves.into_iter().rev().take(3).sum())
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
