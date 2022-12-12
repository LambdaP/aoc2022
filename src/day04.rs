use crate::{bail, Aoc, Day04, Result};

impl Aoc<u32> for Day04 {
    fn part1(&self, lines: &[&[u8]]) -> Result<u32> {
        return part1(lines);
    }
    fn part2(&self, lines: &[&[u8]]) -> Result<u32> {
        return part2(lines);
    }
}

pub fn part1(lines: &[&[u8]]) -> Result<u32> {
    lines
        .iter()
        .map(|l| {
            if let [a, b, x, y] = parse_line(l)?[..] {
                Ok(u32::from((a <= x && y <= b) || (x <= a && b <= y)))
            } else {
                bail!("parse error")
            }
        })
        .sum()
}

pub fn part2(lines: &[&[u8]]) -> Result<u32> {
    let mut res = 0;
    for l in lines {
        if let [a, b, x, y] = parse_line(l)?[..] {
            if (a <= x && x <= b) || (x <= a && a <= y) {
                res += 1;
            }
        } else {
            bail!("parse error");
        }
    }
    Ok(res)
}

fn parse_line(line: &[u8]) -> Result<Vec<u32>> {
    line.split(|c| *c == b',')
        .flat_map(|s| s.split(|c| *c == b'-'))
        .map::<Result<u32>, _>(|s| {
            let s = std::str::from_utf8(s)?;
            Ok(s.parse()?)
        })
        .collect()
}
