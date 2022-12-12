use crate::{bail, Aoc, Day10, Display, Result};

impl Aoc for Day10 {
    fn part1(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let instructions = parse(lines)?;

        let instructions = instructions.into_iter().flat_map(|i| match i {
            Some(x) => vec![None, Some(x)],
            None => vec![None],
        });

        // cycle counter starts at 1
        let instructions = [None].into_iter().chain(instructions);

        let res: i32 = instructions
            .scan(1, |state, x| {
                let old = *state;
                if let Some(x) = x {
                    *state += x;
                }
                Some(old)
            })
            .enumerate()
            .filter_map(|(i, x)| {
                if i % 40 == 20 {
                    Some((i as i32) * x)
                } else {
                    None
                }
            })
            .sum();

        result!(res)
    }

    fn part2(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let instructions = parse(lines)?;

        let instructions = instructions.into_iter().flat_map(|i| match i {
            Some(x) => vec![None, Some(x)],
            None => vec![None],
        });

        let pos = instructions.scan(1, |state, x| {
            let old = *state;
            if let Some(x) = x {
                *state += x;
            }
            Some(old)
        });

        let raw_display = std::iter::repeat(0_i32..40_i32)
            .flatten()
            .zip(pos)
            .map(|(i, x)| if (x - i).abs() <= 1 { b'#' } else { b'.' })
            .collect::<Vec<u8>>();

        let display = raw_display
            .chunks(40)
            .map::<Result<_>, _>(|l| Ok(std::str::from_utf8(l)?))
            .collect::<Result<Vec<&str>>>()?;

        let mut res = String::new();
        for row in display {
            res.push('\n');
            res.push_str(row);
        }

        result!(res)
    }
}

fn parse(lines: &[&[u8]]) -> Result<Vec<Option<i32>>> {
    lines
        .iter()
        .map(|l| {
            let instr = match &l[0..4] {
                b"noop" => None,
                b"addx" => Some(std::str::from_utf8(&l[5..])?.parse()?),
                _ => bail!("parse error"),
            };
            Ok(instr)
        })
        .collect()
}
