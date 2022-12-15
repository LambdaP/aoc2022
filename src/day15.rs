use crate::{bail, eyre, Aoc, Day15, Result};
use std::collections::BTreeSet as Set;
use std::fmt::Display;

// dummy input:
//
// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
// Sensor at x=9, y=16: closest beacon is at x=10, y=16
// Sensor at x=13, y=2: closest beacon is at x=15, y=3
// (...)

impl Aoc for Day15 {
    fn part1(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let data: Vec<[(i32, i32); 2]> = lines
            .iter()
            .map(|line| parse_line(std::str::from_utf8(line)?))
            .collect::<Result<_>>()?;

        let f10 = forbidden(10, &data);
        let f2000000 = forbidden(2000000, &data);

        result!(format!("\n10: {}\n2000000: {}", f10, f2000000))
    }

    fn part2(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let data: Vec<[(i32, i32); 2]> = lines
            .iter()
            .map(|line| parse_line(std::str::from_utf8(line)?))
            .collect::<Result<_>>()?;

        if let Some(res) = search_beacon::<4000000>(&data) {
            return result!(res);
        }

        bail!("search did not complete");
    }
}

fn parse_line(line: &str) -> Result<[(i32, i32); 2]> {
    fn parse_var(s: &str) -> Result<i32, std::num::ParseIntError> {
        s[2..].parse::<i32>()
    }
    fn parse_pair(s: &str) -> Result<(i32, i32)> {
        let (x, y) = s.split_once(", ").ok_or_else(|| eyre!("parse error"))?;
        Ok((parse_var(x)?, parse_var(y)?))
    }
    let (l, r) = line.split_once(": ").ok_or_else(|| eyre!("parse error"))?;
    Ok([parse_pair(&l[10..])?, parse_pair(&r[21..])?])
}

fn manhattan(p1: (i32, i32), p2: (i32, i32)) -> usize {
    (p1.0 - p2.0).unsigned_abs() as usize + (p1.1 - p2.1).unsigned_abs() as usize
}

fn ball_cut(row: i32, (sx, sy): (i32, i32), r: usize) -> Option<(i32, i32)> {
    r.checked_sub((sy - row).unsigned_abs() as usize)
        .map(|rx| (sx - rx as i32, sx + rx as i32))
}

// assumes a sorted, non-empty input
fn fuse_segments(segments: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut v: Vec<(i32,i32)> = vec![];

    let mut p = segments[0];

    for &q in &segments[1..] {
        if q.0 <= p.1 + 1 {
            p.1 = p.1.max(q.1);
        } else {
            println!("{:?}", p);
            v.push(p);
            p = q;
        }
    }

    // println!("{:?}", p);
    v.push(p);

    v
}

// assumes a sorted, non-empty input
fn first_segment(segments: &[(i32, i32)]) -> Option<(i32, i32)> {
    let mut p = segments[0];

    for &q in &segments[1..] {
        if q.0 <= p.1 + 1 {
            p.1 = p.1.max(q.1);
        } else {
            return Some(p);
        }
    }

    None
}

fn forbidden(y: i32, data: &[[(i32, i32); 2]]) -> usize {
    let mut segments: Vec<(i32, i32)> = vec![];
    let mut beacons_y: Set<i32> = Set::new();

    for &[s, b] in data {
        if b.1 == y {
            beacons_y.insert(b.0);
        }
        let r = manhattan(s, b);
        if let Some(seg) = ball_cut(y, s, r) {
            segments.push(seg);
        }
    }

    if segments.is_empty() { return 0; }

    segments.sort_unstable();
    let segments = fuse_segments(&segments);

    segments
        .iter()
        .map(|(l, r)| (r - l + 1) as usize)
        .sum::<usize>()
        - beacons_y
            .iter()
            .filter(|&x| segments.iter().any(|(l, r)| l <= x && x <= r))
            .count()
}

fn search_beacon<const MULT: usize>(data: &[[(i32, i32); 2]]) -> Option<usize> {
    for y in 0..=MULT {
        let mut segments = vec![];

        for &[s, b] in data {
            let rad = manhattan(s, b);
            if let Some((l, r)) = ball_cut(y as i32, s, rad) {
                let l = l.max(0);
                let r = r.min(MULT as i32);
                if l <= r {
                    segments.push((l, r));
                }
            };
        }

        segments.sort_unstable();

        if let Some((_,r)) = first_segment(&segments) {
            return Some(MULT * (r as usize +1) + y);
        }
    }

    None
}
