use crate::{bail, Aoc, Day04, Display, FileRep, Result};

impl Aoc for Day04 {
    fn part1(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.byte_lines;
        let res: Result<u32> = lines
            .iter()
            .map::<Result<u32>, _>(|l| {
                let [a, b, x, y] = parse_line(l)?[..] else {
                    bail!("parse error");
                };
                Ok(u32::from((a <= x && y <= b) || (x <= a && b <= y)))
            })
            .sum();
        result!(res?)
    }

    fn part2(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.byte_lines;
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
        result!(res)
    }
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
