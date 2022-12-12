use crate::{bail, Aoc, Day09, Display, Result};
use std::collections::hash_set::HashSet as Set;

impl Aoc for Day09 {
    fn part1(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let moves = parse(lines)?;
        let res = run_nknots::<2>(&moves);
        result!(res)
    }
    fn part2(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let moves = parse(lines)?;
        let res = run_nknots::<10>(&moves);
        result!(res)
    }
}

fn parse(lines: &[&[u8]]) -> Result<Vec<(u8, u32)>> {
    lines
        .iter()
        .map(|l| {
            let dir = match (l[0], l[1]) {
                (c, b' ') if c == b'U' || c == b'D' || c == b'L' || c == b'R' => c,
                _ => bail!("parse error"),
            };
            let nsteps = std::str::from_utf8(&l[2..])?.parse::<u32>()?;
            Ok((dir, nsteps))
        })
        .collect()
}

fn run_nknots<const NKNOTS: usize>(moves: &[(u8, u32)]) -> usize {
    let mut knots: [(isize, isize); NKNOTS] = [(0, 0); NKNOTS];
    let mut visited: Set<(isize, isize)> = Set::new();
    visited.insert((0, 0));

    for &(dir, nsteps) in moves {
        let (x, y) = match dir {
            b'U' => (0, 1),
            b'D' => (0, -1),
            b'L' => (-1, 0),
            b'R' => (1, 0),
            _ => panic!(),
        };

        for _ in 0..nsteps {
            knots[0].0 += x;
            knots[0].1 += y;

            knots = move_rope::<NKNOTS>(knots);

            visited.insert(knots[NKNOTS - 1]);
        }
    }

    visited.len()
}

fn move_rope<const NKNOTS: usize>(mut knots: [(isize, isize); NKNOTS]) -> [(isize, isize); NKNOTS] {
    for i in 0..NKNOTS - 1 {
        let (hx, hy) = knots[i];
        let (mut tx, mut ty) = knots[i + 1];

        if hx - tx == 2 {
            tx += 1;
            match ty.cmp(&hy) {
                std::cmp::Ordering::Greater => ty += 1,
                std::cmp::Ordering::Less => ty -= 1,
                _ => {}
            }
        } else if hx - tx == -2 {
            tx -= 1;
            match ty.cmp(&hy) {
                std::cmp::Ordering::Greater => ty += 1,
                std::cmp::Ordering::Less => ty -= 1,
                _ => {}
            }
        } else if hy - ty == 2 {
            ty += 1;
            tx = hx;
        } else if hy - ty == -2 {
            ty -= 1;
            tx = hx;
        }

        knots[i + 1] = (tx, ty);
    }

    knots
}
