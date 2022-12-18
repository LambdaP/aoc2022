use crate::{bail, eyre, Aoc, Day16, FileRep, Result};
use std::collections::BinaryHeap as Heap;
use std::collections::VecDeque as Deque;
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
                    .map(|dst| Arc { src:i, dst, cost: 1 })
                    .collect();
                ls.sort_unstable_by_key(|&Arc {dst, ..}| dst);
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

        positive_valves.push(0);

        let mut contracted_arcs: Vec<Vec<Arc<usize>>> = vec![vec![];gr.arcs.len()];

        for a in positive_valves
            .iter()
            .flat_map(|&v| {
                let mut paths = paths_to(&gr, v, &positive_valves);
                // paths.dedup_by(|x, y| is_prefix(&y, &x));
                paths
            })
            .filter(|p| !p.is_empty())
            .map(|p| Arc {
                src: p.first().unwrap().src,
                dst: p.last().unwrap().dst,
                cost: p.len(),
            })
            {
                contracted_arcs[a.src].push(a);
            }

        for foo in &contracted_arcs {
            if !foo.is_empty() {
                println!("valve {:?}: pressure: {:?}, paths: {:?}", foo[0].src, gr.weights[foo[0].src], foo);
            }
        }

        let contracted = Graph {
            arcs: contracted_arcs,
            weights: gr.weights
        };

        let res = exhaustive_search::<4>(&contracted, 0);

        result!(res)



        // for v in paths_to(&gr, 0, &positive_valves) {
        //     let score = path_pressure(30, &gr.weights, &v);
        //     println!("Score {:?} for path {:?}", score, v);
        // }

        // let positive_valves = (0..).zip(rates.into_iter()).filter_map(|(i,r)| (r > 0).then_some(i));
        // todo!();

        // result!("todo")
    }

    fn part2(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.byte_lines;
        result!("todo")
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

fn path_pressure(nrounds: usize, rates: &[usize], path: &[Arc<usize>]) -> usize {
    let mut res = 0;
    let mut remaining = nrounds;
    for &Arc { dst, cost, .. } in path {
        if remaining < cost {
            break;
        }
        remaining -= cost;
        res += rates[dst] * remaining;
    }
    // path.into_iter().
    // (0..nrounds)
    //     .rev()
    //     .zip(path.into_iter())
    //     .map(|(i, &v)| i * rates[v] as usize)
    //     .sum()
    res
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

fn paths_to(gr: &Graph, source: usize, targets: &[usize]) -> Vec<Vec<Arc<usize>>> {
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

    let mut visited = vec![vec![]; gr.arcs.len()];
    // visited[source].push(source);
    let mut to_process: Heap<TS> = Heap::new();

    to_process.push(TS {
        vert: source,
        cost: 0,
        score: gr.weights[source],
    });

    let mut res = vec![];

    while let Some(TS {
        vert: v, cost: c, ..
    }) = to_process.pop()
    {
        if targets.iter().any(|&t| t == v) {
            res.push(visited[v].clone());
        }

        if res.len() == targets.len() {
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

    res.sort_unstable_by(|x, y| x.cmp(&y));
    res
}

fn is_prefix<T: Eq>(a: &[T], b: &[T]) -> bool {
    if b.len() < a.len() || a.len() == 0 {
        return false;
    }

    a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

fn exhaustive_search<const LIMIT: usize>(gr: &Graph, source: usize) -> usize {
    let mut res = 0;
    let mut stack = vec![gr.arcs[source].clone()];
    let mut path = vec![Arc { src: source, dst: source, cost: 0}];
    while let Some(mut arcs) = stack.pop() {
        let Some(arc) = arcs.pop() else {
            let pressure = path_pressure(LIMIT, &gr.weights, &path);
            res = if res < pressure { pressure } else { res };

            path.pop();

            continue;
        };

        stack.push(arcs);

        let path_cost: usize = path.iter().map(|&a| a.cost).sum();

        if path_cost + arc.cost < LIMIT {
            path.push(arc);
            stack.push(gr.arcs[arc.dst].clone());
        }
    }

    res
}

// fn dfs_all_covering_paths(gr: &Graph, source: usize, targets: &[usize], limit: usize) {
//     let mut head = source;
//     let mut res = vec![];
//
//
// }

// fn covering_paths<const LIMIT: usize>(arcs: Vec<Vec<Arc<usize>>>, source: usize) -> Vec<Vec<Arc<usize>>> {
//     let mut vert_stack = vec![source];
//     let mut res = vec![];
//
//     res
// }
