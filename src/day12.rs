use crate::{eyre, Aoc, Day12, Result};
use std::collections::HashMap as Map;
use std::collections::hash_set::HashSet as Set;
use std::collections::BinaryHeap as Heap;

#[derive(Copy, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
struct Point { x: usize, y: usize }

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    p: Point,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.p.cmp(&other.p))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Aoc<usize> for Day12 {
    fn part1(&self, lines: &[&[u8]]) -> Result<usize> {
        let (start,end) = locate_start_end(lines).unwrap();

        let paths = dijkstra(lines, start, true);
        let (_, res) = paths.get(&end).ok_or_else(|| eyre!("no path to end point"))?;

        Ok(*res)
    }
    fn part2(&self, lines: &[&[u8]]) -> Result<usize> {
        let (_,end) = locate_start_end(lines).unwrap();

        let paths = dijkstra(lines, end, false);

        paths
            .into_values()
            .filter_map(|(e,c)| (e == b'a').then_some(c))
            .min()
            .ok_or_else(|| eyre!("error"))
    }
}

fn dijkstra(grid: &[&[u8]], source: Point, asc: bool) -> Map<Point,(u8, usize)> {
    let mut res: Map<Point,(u8,usize)> = Map::new();
    let mut visited: Set<Point> = Set::new();
    let mut to_process: Heap<State> = Heap::new();


    let &start_el = get_point(grid, source).unwrap();

    res.insert(source,(start_el,0));
    to_process.push(State { p: source, cost: 0 });

    while let Some(state) = to_process.pop() {
        let point = state.p;
        let current_cost = state.cost;

        for p in neighbors(grid, point, asc) {
            if visited.contains(&p) {
                continue;
            } else if let Some((_,cost)) = res.get(&p) {
                if *cost <= current_cost + 1 {
                    continue;
                }
            }
            let el = elevation(*get_point(grid, p).unwrap());
            res.insert(p, (el, current_cost+1));
            to_process.push(State { p, cost: current_cost + 1 });
        }

        visited.insert(point);
    }

    res
}

fn get_point<'a, T>(grid: &'a[&'a[T]], point: Point) -> Option<&'a T> {
    grid.get(point.x)?.get(point.y)
}

fn von_neumann(point: Point) -> Vec<Point> {
    let (x,y) = (point.x, point.y);

    [x.checked_add(1).and_then(|x| Some(Point {x, y})),
    y.checked_add(1).and_then(|y| Some(Point {x, y})),
    x.checked_sub(1).and_then(|x| Some(Point {x, y})),
    y.checked_sub(1).and_then(|y| Some(Point {x, y}))]
        .into_iter().flatten().collect()
}

fn elevation(x: u8) -> u8 {
    match x {
        b'S' => b'a',
        b'E' => b'z',
        _ => x
    }
}

fn neighbors(grid: &[&[u8]], point: Point, asc: bool) -> Vec<Point> {
    let cur = elevation(*get_point(grid, point).unwrap());

    von_neumann(point).into_iter().filter(|p| {
        if let Some(x) = get_point(grid, *p) {
            if asc {
                elevation(*x) <= cur + 1
            } else {
                cur <= elevation(*x) + 1
            }
        } else {
            false
        }
    })
    .collect()
}

fn locate_start_end(lines: &[&[u8]]) -> Option<(Point,Point)> {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == b'S' {
                start = Some(Point{x:i, y:j});
            } else if lines[i][j] == b'E' {
                end = Some(Point{x:i, y:j});
            }
        }
    }

    Some((start?,end?))
}
