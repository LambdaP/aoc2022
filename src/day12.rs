use crate::{eyre, Aoc, Day12, Result};
use std::collections::hash_set::HashSet as Set;
use std::collections::BinaryHeap as Heap;
use std::collections::HashMap as Map;
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    p: Point,
    el: u8,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.p.cmp(&other.p))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Aoc for Day12 {
    fn part1(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let (start, end) = locate_start_end(lines).unwrap();

        let paths = dijkstra(lines, start, |cur_el, el| el <= cur_el + 1);
        let (_, res) = paths
            .get(&end)
            .ok_or_else(|| eyre!("no path to end point"))?;

        result!(*res)
    }

    fn part2(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        let (_, end) = locate_start_end(lines).unwrap();

        let paths = dijkstra(lines, end, |cur_el, el| cur_el <= el + 1);

        let res = paths
            .into_values()
            .filter_map(|(e, c)| (e == b'a').then_some(c))
            .min()
            .ok_or_else(|| eyre!("no path from lowest elevation"));

        result!(res?)
    }
}

fn dijkstra(
    grid: &[&[u8]],
    source: Point,
    can_reach: fn(u8, u8) -> bool,
) -> Map<Point, (u8, usize)> {
    let mut res: Map<Point, (u8, usize)> = Map::new();
    let mut visited: Set<Point> = Set::new();
    let mut to_process: Heap<State> = Heap::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, b) in row.iter().enumerate() {
            let el = match *b {
                b'S' => b'a',
                b'E' => b'z',
                _ => *b,
            };
            res.insert(Point { x, y }, (el, usize::MAX));
        }
    }

    let &(source_el, _) = res.get(&source).unwrap();
    res.insert(source, (source_el, 0));

    to_process.push(State {
        p: source,
        el: source_el,
        cost: 0,
    });

    while let Some(State {
        p: cur,
        el: cur_el,
        cost: cur_cost,
    }) = to_process.pop()
    {
        for p in von_neumann(cur) {
            let Some(&(el, cost)) = res.get(&p) else {
                continue;
            };

            if !can_reach(cur_el, el) || visited.contains(&p) || cost <= cur_cost + 1 {
                continue;
            }

            res.insert(p, (el, cur_cost + 1));
            to_process.push(State {
                p,
                el,
                cost: cur_cost + 1,
            });
        }

        visited.insert(cur);
    }

    res
}

fn von_neumann(point: Point) -> Vec<Point> {
    let (x, y) = (point.x, point.y);

    [
        x.checked_add(1).map(|x| Point { x, y }),
        y.checked_add(1).map(|y| Point { x, y }),
        x.checked_sub(1).map(|x| Point { x, y }),
        y.checked_sub(1).map(|y| Point { x, y }),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn locate_start_end(lines: &[&[u8]]) -> Option<(Point, Point)> {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for (x, line) in lines.iter().enumerate() {
        for (y, b) in line.iter().enumerate() {
            start = start.or_else(|| (*b == b'S').then_some(Point { x, y }));
            end = end.or_else(|| (*b == b'E').then_some(Point { x, y }));
        }
    }

    Some((start?, end?))
}
