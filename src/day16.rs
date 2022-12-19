use crate::{bail, eyre, Aoc, Day16, FileRep, Result};
use std::collections::BinaryHeap as Heap;
use std::fmt::Display;

// dummy input:
//
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
// Valve BB has flow rate=13; tunnels lead to valves CC, AA
// Valve CC has flow rate=2; tunnels lead to valves DD, BB
// Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE

impl Aoc for Day16 {
    fn part1(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.string_lines;
        let mut data = lines
            .into_iter()
            .map(|l| parse_line(l))
            .collect::<Result<Vec<_>>>()?;
        data.sort_unstable_by(|x, y| x.0.cmp(&y.0));

        let mut valves = vec![];
        let mut rates: Vec<usize> = vec![];

        for &(v, (r, _)) in &data {
            valves.push(v);
            rates.push(r);
        }

        let arcs: Vec<Vec<Arc<usize>>> = data
            .into_iter()
            .enumerate()
            .map(|(i, (_, (_, vs)))| {
                let mut ls: Vec<Arc<usize>> = vs
                    .into_iter()
                    .map(|v| valves.binary_search(&v).unwrap())
                    .map(|dst| Arc {
                        src: i,
                        dst,
                        cost: 1,
                    })
                    .collect();
                ls.sort_unstable_by_key(|&Arc { dst, .. }| dst);
                ls
            })
            .collect();

        let gr = Graph {
            arcs,
            weights: rates,
        };

        let positive_valves: Vec<usize> = [0]
            .into_iter()
            .chain(
                (0..)
                    .zip(gr.weights.iter())
                    .filter_map(|(i, &r)| (r > 0).then_some(i)),
            )
            .collect::<Vec<_>>();

        let mut distances = vec![vec![0; gr.weights.len()]; gr.weights.len()];

        for Arc { src, dst, cost } in distance_clique(&gr, &positive_valves) {
            distances[src][dst] = cost;
            distances[dst][src] = cost;
        }

        let res = best_permutation(30, &distances, &gr.weights, &positive_valves);

        result!(res.0)
    }

    fn part2(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.string_lines;
        let mut data = lines
            .into_iter()
            .map(|l| parse_line(l))
            .collect::<Result<Vec<_>>>()?;
        data.sort_unstable_by(|x, y| x.0.cmp(&y.0));

        let mut valves = vec![];
        let mut rates: Vec<usize> = vec![];

        for &(v, (r, _)) in &data {
            valves.push(v);
            rates.push(r);
        }

        let arcs: Vec<Vec<Arc<usize>>> = data
            .into_iter()
            .enumerate()
            .map(|(i, (_, (_, vs)))| {
                let mut ls: Vec<Arc<usize>> = vs
                    .into_iter()
                    .map(|v| valves.binary_search(&v).unwrap())
                    .map(|dst| Arc {
                        src: i,
                        dst,
                        cost: 1,
                    })
                    .collect();
                ls.sort_unstable_by_key(|&Arc { dst, .. }| dst);
                ls
            })
            .collect();

        let gr = Graph {
            arcs,
            weights: rates,
        };

        let mut positive_valves: Vec<usize> = (0..)
            .zip(gr.weights.iter())
            .filter_map(|(i, &r)| (r > 0).then_some(i))
            .collect::<Vec<_>>();

        let mut distances = vec![vec![0; gr.weights.len()]; gr.weights.len()];

        positive_valves.push(0);

        for Arc { src, dst, cost } in distance_clique(&gr, &positive_valves) {
            distances[src][dst] = cost;
            distances[dst][src] = cost;
        }

        positive_valves.pop();

        let mut res = 0;

        for mut partition in 0..2_usize.pow(positive_valves.len() as u32 - 1) {
            let mut my_valves = vec![0];
            let mut el_valves = vec![0];

            for &valve in &positive_valves {
                if partition & 1 == 1 {
                    my_valves.push(valve);
                } else {
                    el_valves.push(valve);
                }
                partition >>= 1;
            }

            let mine = best_permutation(26, &distances, &gr.weights, &my_valves);
            let elephant = best_permutation(26, &distances, &gr.weights, &el_valves);
            let total = mine.0 + elephant.0;
            if res < total {
                res = total;
            }
        }

        result!(res)
    }
}

fn parse_line(line: &str) -> Result<(u16, (usize, Vec<u16>))> {
    fn valve(s: &str) -> u16 {
        let res = u16::from_ne_bytes(s.as_bytes()[0..2].try_into().unwrap());
        res
    }

    let (l, r) = line
        .split_once("; tunnels lead to valves ")
        .or_else(|| line.split_once("; tunnel leads to valve "))
        .ok_or_else(|| eyre!("parse error"))?;
    let (ll, lr) = &l[6..]
        .split_once(" has flow rate=")
        .ok_or_else(|| eyre!("parse error"))?;
    let r: Vec<u16> = r.split(", ").map(|v| valve(v)).collect();

    Ok((valve(ll), (lr.parse()?, r)))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Arc<T> {
    src: T,
    dst: T,
    cost: usize,
}

struct Graph {
    arcs: Vec<Vec<Arc<usize>>>,
    weights: Vec<usize>,
}

fn distance_clique(gr: &Graph, targets: &[usize]) -> Vec<Arc<usize>> {
    let mut targets = targets.to_vec();

    #[derive(Eq, PartialEq, Copy, Clone)]
    struct TS {
        vert: usize,
        cost: usize,
        score: usize,
    }

    impl Ord for TS {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.score.cmp(&other.score))
                .then_with(|| self.vert.cmp(&other.vert))
        }
    }

    impl PartialOrd for TS {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut res = vec![];

    while let Some(source) = targets.pop() {
        let mut visited = vec![vec![]; gr.arcs.len()];
        let mut to_process: Heap<TS> = Heap::new();

        to_process.push(TS {
            vert: source,
            cost: 0,
            score: gr.weights[source],
        });

        let mut paths = vec![];

        while let Some(TS {
            vert: v, cost: c, ..
        }) = to_process.pop()
        {
            if targets.iter().any(|&t| t == v) {
                paths.push(visited[v].clone());
            }

            if paths.len() == targets.len() {
                break;
            }

            for &a in &gr.arcs[v] {
                let Arc { dst, cost, .. } = a;
                if dst != source && visited[dst].is_empty() {
                    visited[dst] = visited[v].clone();
                    visited[dst].push(a);
                    to_process.push(TS {
                        vert: dst,
                        cost: c + cost,
                        score: gr.weights[dst],
                    });
                }
            }
        }

        for arc in paths.into_iter().map(|p| Arc {
            src: p.first().unwrap().src,
            dst: p.last().unwrap().dst,
            cost: p.len(),
        }) {
            res.push(arc);
        }
    }

    res
}

fn best_permutation(
    limit: usize,
    distances: &[Vec<usize>],
    weights: &[usize],
    v: &[usize],
) -> (usize, Vec<usize>) {
    fn permute(
        distances: &[Vec<usize>],
        weights: &[usize],
        v: &mut [usize],
        remaining: usize,
    ) -> (usize, Vec<usize>) {
        let mut score = 0;
        let mut path = vec![];
        if v.is_empty() {
            return (score, path);
        }

        let v0 = v[0];

        score += remaining * weights[v0];
        path.push(v0);

        if v.len() == 1 {
            return (score, path);
        }

        let mut subs = vec![];

        for i in 1..v.len() {
            let v1 = v[1];
            let vi = v[i];

            let dist = distances[v0][vi] + 1;

            if remaining <= dist {
                continue;
            }

            let remaining = remaining - dist;

            v[i] = v1;
            v[1] = vi;

            let sub = permute(distances, weights, &mut v[1..], remaining);
            subs.push(sub);

            v[i] = vi;
            v[1] = v1;
        }

        let Some((sub_score, mut sub_path)) = subs.into_iter().max_by_key(|x| x.0) else {
            return (score, path);
        };

        score += sub_score;
        path.append(&mut sub_path);

        (score, path)
    }

    permute(distances, weights, &mut v.to_vec(), limit)
}
