use crate::{bail, eyre, Aoc, Day02, Display, Result};

impl Aoc for Day02 {
    fn part1(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        result!(score(parse_part1(lines)?))
    }
    fn part2(&self, lines: &[&[u8]]) -> Result<Box<dyn Display>> {
        result!(score(parse_part2(lines)?))
    }
}

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

fn score(games: Vec<(Move, Move)>) -> u32 {
    let mut score = 0;
    for g in games {
        score += match g {
            (_, Move::Rock) => 1,
            (_, Move::Paper) => 2,
            (_, Move::Scissor) => 3,
        };
        score += match g {
            (Move::Rock, Move::Rock)
            | (Move::Paper, Move::Paper)
            | (Move::Scissor, Move::Scissor) => 3,
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissor)
            | (Move::Scissor, Move::Rock) => 6,
            _ => 0,
        };
    }

    score
}

fn parse_part1(lines: &[&[u8]]) -> Result<Vec<(Move, Move)>> {
    let mut v = Vec::new();
    for l in lines {
        let them = parse_move(b'A', *l.first().ok_or_else(|| eyre!("parse error"))?)?;
        let me = parse_move(b'X', *l.get(2).ok_or_else(|| eyre!("parse error"))?)?;

        v.push((them, me));
    }

    Ok(v)
}

fn beats(m: Move) -> Move {
    match m {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissor,
        Move::Scissor => Move::Rock,
    }
}

fn loses(m: Move) -> Move {
    match m {
        Move::Rock => Move::Scissor,
        Move::Paper => Move::Rock,
        Move::Scissor => Move::Paper,
    }
}

fn parse_move(basis: u8, m: u8) -> Result<Move> {
    let res = match m {
        x if x == basis => Move::Rock,
        x if x == basis + 1 => Move::Paper,
        x if x == basis + 2 => Move::Scissor,
        _ => bail!("parse error"),
    };
    Ok(res)
}

fn parse_part2(lines: &[&[u8]]) -> Result<Vec<(Move, Move)>> {
    let mut v = Vec::new();
    for l in lines {
        let them = parse_move(b'A', *l.first().ok_or_else(|| eyre!("parse error"))?)?;
        let me = match l.get(2).ok_or_else(|| eyre!("parse error"))? {
            b'X' => loses(them),
            b'Y' => them,
            b'Z' => beats(them),
            _ => bail!("parse error"),
        };

        v.push((them, me));
    }

    Ok(v)
}
