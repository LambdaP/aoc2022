use crate::{Aoc, Day08, Display, Result};
use std::collections::hash_set::HashSet as Set;

impl Aoc for Day08 {
    fn part1(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let height = lines.len();
        let width = lines[0].len();
        let mut visible = Set::<(usize, usize)>::new();

        for (i, line) in lines.iter().enumerate() {
            visible.insert((i, 0));
            visible.insert((i, width - 1));

            let mut max_asc = 0;
            let mut max_desc = 0;

            for (j, &x) in line.iter().enumerate() {
                if x > max_asc {
                    visible.insert((i, j));
                    max_asc = x;
                }
            }

            for (j, &x) in line.iter().enumerate().rev() {
                if x > max_desc {
                    visible.insert((i, j));
                    max_desc = x;
                }
            }
        }

        for j in 0..width {
            visible.insert((0, j));
            visible.insert((height - 1, j));

            let mut max_asc = 0;
            let mut max_desc = 0;

            for (i, line) in lines.iter().enumerate() {
                if line[j] > max_asc {
                    visible.insert((i, j));
                    max_asc = line[j];
                }
            }

            for (i, line) in lines.iter().enumerate().rev() {
                if line[j] > max_desc {
                    visible.insert((i, j));
                    max_desc = line[j];
                }
            }
        }

        result!(visible.len())
    }

    fn part2(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let height = lines.len();
        let width = lines[0].len();

        let res = (0..height)
            .flat_map(|i| (0..width).map(move |j| score(lines, i, j)))
            .max()
            .unwrap();

        result!(res)
    }
}

fn score(grid: &[&[u8]], i: usize, j: usize) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut up = 0;
    for x in (0..i).rev() {
        up += 1;
        if grid[i][j] <= grid[x][j] {
            break;
        }
    }
    let mut down = 0;
    for x in i + 1..height {
        down += 1;
        if grid[i][j] <= grid[x][j] {
            break;
        }
    }
    let mut left = 0;
    for y in (0..j).rev() {
        left += 1;
        if grid[i][j] <= grid[i][y] {
            break;
        }
    }
    let mut right = 0;
    for y in j + 1..width {
        right += 1;
        if grid[i][j] <= grid[i][y] {
            break;
        }
    }

    up * down * left * right
}
