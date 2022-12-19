use crate::{bail, eyre, Aoc, Day13, FileRep, Result};
use std::fmt::Display;

#[derive(Eq, PartialEq, Clone)]
enum List<T> {
    Val(T),
    Vec(Vec<List<T>>),
}

impl<T> Ord for List<T>
where
    T: Copy + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (List::Val(x), List::Val(y)) => x.cmp(y),
            (List::Val(x), _) => List::Vec(vec![List::Val(*x)]).cmp(other),
            (_, List::Val(y)) => self.cmp(&List::Vec(vec![List::Val(*y)])),
            (List::Vec(x), List::Vec(y)) => x.cmp(y),
        }
    }
}

impl<T> PartialOrd for List<T>
where
    T: Copy + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Aoc for Day13 {
    fn part1(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.byte_lines;
        let pairs = parse(lines)?;
        let res: usize = pairs
            .into_iter()
            .enumerate()
            .map(|(i, (x, y))| (i + 1) * usize::from(x <= y))
            .sum();
        result!(res)
    }

    fn part2(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.byte_lines;
        let pairs = parse(lines)?;
        let mut lists = pairs
            .into_iter()
            .flat_map(|(x, y)| [x, y])
            .collect::<Vec<_>>();
        let d2 = parse_line(b"[[2]]").unwrap();
        let d6 = parse_line(b"[[6]]").unwrap();
        lists.push(d2.clone());
        lists.push(d6.clone());
        lists.sort_unstable();
        let mut res = 1;
        for (i, x) in lists.iter().enumerate() {
            if *x == d2 || *x == d6 {
                res *= i + 1;
            }
        }
        result!(res)
    }
}

fn parse(lines: &[&[u8]]) -> Result<Vec<(List<u8>, List<u8>)>> {
    lines
        .split(|&l| l.is_empty())
        .map(|ll| {
            let [l0,l1] = ll[..] else {
                bail!("parse error");
            };
            Ok((parse_line(l0)?, parse_line(l1)?))
        })
        .collect()
}

fn parse_line(line: &[u8]) -> Result<List<u8>> {
    let mut stack: Vec<Vec<List<u8>>> = vec![];
    stack.push(vec![]);
    let mut i: usize = 0;
    while i < line.len() {
        if line[i] == b',' {
        } else if line[i] == b'[' {
            stack.push(vec![]);
        } else if line[i] == b']' {
            let v = stack.pop().ok_or_else(|| eyre!("parse error"))?;
            let p = stack.last_mut().ok_or_else(|| eyre!("parse error"))?;
            p.push(List::Vec(v));
        } else {
            let mut j = i;
            while j < line.len() && b'0' <= line[j] && line[j] <= b'9' {
                j += 1;
            }
            let x = std::str::from_utf8(&line[i..j])?.parse::<u8>()?;
            let p = stack.last_mut().ok_or_else(|| eyre!("parse error"))?;
            p.push(List::Val(x));
            i = j - 1;
        }
        i += 1;
    }
    stack
        .pop()
        .map(List::Vec)
        .ok_or_else(|| eyre!("parse error"))
}
