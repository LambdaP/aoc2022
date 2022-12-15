use crate::{bail, Aoc, Day14, Result};
use std::fmt::Display;

// dummy input:
// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9

#[derive(Eq, PartialEq, Clone, Copy)]
enum Square {
    Empty,
    Rock,
    Sand,
}

use Square::*;

impl Aoc for Day14 {
    fn part1(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let mut paths: Vec<Vec<(usize, usize)>> =
            lines.iter().map(|l| parse_line(l)).collect::<Result<_>>()?;

        let max_y = paths
            .iter()
            .flat_map(|v| v.iter().map(|p| p.1))
            .max()
            .unwrap();

        for path in &mut paths {
            for t in path {
                t.0 -= 500 - max_y;
            }
        }

        let mut grid = vec![vec![Empty; max_y + 1]; 2 * max_y + 1];
        for path in paths {
            unroll_path(&path).for_each(|(x, y)| grid[x][y] = Rock)
        }

        let res =
            std::iter::repeat_with(|| insert_sand(&grid, max_y).map(|(x, y)| grid[x][y] = Sand))
                .take_while(|x| x.is_some())
                .count();

        result!(res)
    }

    fn part2(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let mut paths: Vec<Vec<(usize, usize)>> =
            lines.iter().map(|l| parse_line(l)).collect::<Result<_>>()?;

        let max_y = paths
            .iter()
            .flat_map(|v| v.iter().map(|p| p.1))
            .max()
            .unwrap();

        let max_y = max_y + 2;

        for path in &mut paths {
            for t in path {
                t.0 -= 500 - max_y;
            }
        }

        paths.push(vec![(0, max_y), (2 * max_y, max_y)]);

        let mut grid = vec![vec![Empty; max_y + 1]; 2 * max_y + 1];

        for path in paths {
            unroll_path(&path).for_each(|(x, y)| grid[x][y] = Rock)
        }

        let res =
            std::iter::repeat_with(|| insert_sand(&grid, max_y).map(|(x, y)| grid[x][y] = Sand))
                .take_while(|x| x.is_some())
                .count();

        result!(res)
    }
}

fn insert_sand(grid: &[Vec<Square>], x: usize) -> Option<(usize, usize)> {
    let (mut x, mut y) = (x, 0);
    if grid[x][y] != Empty {
        return None;
    }
    loop {
        let Some(&s) = grid[x].get(y+1) else {
            return None;
        };

        if s == Empty {
        } else if grid[x - 1][y + 1] == Empty {
            x -= 1;
        } else if grid[x + 1][y + 1] == Empty {
            x += 1;
        } else {
            return Some((x, y));
        }

        y += 1;
    }
}

fn parse_line(line: &[u8]) -> Result<Vec<(usize, usize)>> {
    std::str::from_utf8(line)?
        .split(" -> ")
        .map(|w| {
            let [x,y] = w.splitn(2, ',').collect::<Vec<_>>()[..] else {
                bail!("parse error");
            };
            Ok((x.parse::<usize>()?, y.parse::<usize>()?))
        })
        .collect()
}

// fn unroll_path(path: &[(usize,usize)]) -> Vec<(usize,usize)> {
fn unroll_path(path: &[(usize, usize)]) -> impl Iterator<Item = (usize, usize)> + '_ {
    path.windows(2).flat_map(|w| {
        let [(x1,y1),(x2,y2)] = w[..] else {
            panic!();
        };
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        (x1..=x2).flat_map(move |x| (y1..=y2).map(move |y| (x, y)))
    })
}
