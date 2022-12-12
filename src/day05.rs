use crate::{eyre, Aoc, Day05, Result};

impl Aoc<String> for Day05 {
    fn part1(&self, lines: &[&[u8]]) -> Result<String> {
        return part1(lines);
    }
    fn part2(&self, lines: &[&[u8]]) -> Result<String> {
        return part2(lines);
    }
}

pub fn part1(lines: &[&[u8]]) -> Result<String> {
    let mut parsed = None;
    for i in 0..lines.len() {
        if lines[i].is_empty() {
            parsed = Some((&lines[0..i], &lines[i + 1..]));
        }
    }
    let (crates, moves) = parsed.ok_or_else(|| eyre!("parse error"))?;
    let mut crates = parse_crates(crates)?;
    let moves = parse_moves(moves)?;

    for (m, s, t) in moves {
        for _ in 0..m {
            let c = crates[s - 1].pop().ok_or_else(|| eyre!("error"))?;
            crates[t - 1].push(c);
        }
    }

    let mut res = String::new();

    for mut cc in crates {
        if let Some(top) = cc.pop() {
            res.push(top as char);
        }
    }

    Ok(res)
}

pub fn part2(lines: &[&[u8]]) -> Result<String> {
    let mut parsed = None;
    for i in 0..lines.len() {
        if lines[i].is_empty() {
            parsed = Some((&lines[0..i], &lines[i + 1..]));
        }
    }
    let (crates, moves) = parsed.ok_or_else(|| eyre!("parse error"))?;
    let mut crates = parse_crates(crates)?;
    let moves = parse_moves(moves)?;

    for (m, s, t) in moves {
        let len = crates[s - 1].len();
        let mut top = crates[s - 1].split_off(len - m);
        crates[t - 1].append(&mut top);
        // crates[t-1].drain(&crates[s-1][len-m..len]);
        // let top = crates[s-1].rchunks(m).next().ok_or_else(|| eyre!("error"))?;
        // crates[t-1].drain(top).collect();
    }

    let mut res = String::new();

    for mut cc in crates {
        if let Some(top) = cc.pop() {
            res.push(top as char);
        }
    }

    Ok(res)
}

fn parse_crates(lines: &[&[u8]]) -> Result<Vec<Vec<u8>>> {
    let mut stacks = lines.iter().rev();
    let ncols: usize = (2 + stacks.next().ok_or_else(|| eyre!("parse error"))?.len()) / 4;
    let mut res = vec![Vec::new(); ncols];

    for row in stacks {
        for i in 0..ncols {
            let c = row[4 * i + 1];
            if c != b' ' {
                res[i].push(c);
            }
        }
    }

    Ok(res)
}

fn parse_moves(lines: &[&[u8]]) -> Result<Vec<(usize, usize, usize)>> {
    lines
        .iter()
        .map::<Result<_>, _>(|l| {
            let words = l.split(|c| *c == b' ').collect::<Vec<&[u8]>>();
            let mut v = vec![];
            for i in [1, 3, 5] {
                let w = words[i];
                let s = std::str::from_utf8(w)?;
                v.push(s.parse()?);
            }
            Ok((v[0], v[1], v[2]))
        })
        .collect::<Result<Vec<_>>>()
}
