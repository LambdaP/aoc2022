use crate::{Aoc, Day11, Result, bail};
use std::collections::VecDeque;

// shape of input: list of
// Monkey 0:
//   Starting items: 89, 84, 88, 78, 70
//   Operation: new = old * 5
//   Test: divisible by 7
//     If true: throw to monkey 6
//     If false: throw to monkey 7

struct Monkey {
    items: VecDeque<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    divisor: usize,
    monkey_true: usize,
    monkey_false: usize
}

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Times
}

impl Aoc<usize> for Day11 {
    fn part1(&self, lines: &[&[u8]]) -> Result<usize> {
        let mut monkeys: Vec<Monkey> = parse(lines)?;
        let mut counts: Vec<usize> = vec![0;monkeys.len()];
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                while let Some(item) = monkeys[i].items.pop_front() {
                    let item = (monkeys[i].op)(item)/3;
                    let next_monkey = if (item % monkeys[i].divisor) == 0 { monkeys[i].monkey_true } else { monkeys[i].monkey_false };
                    monkeys[next_monkey as usize].items.push_front(item);
                    counts[i] += 1;
                }
            }
        }

        counts.sort_unstable();
        let [x,y] = counts.windows(2).last().unwrap()[..] else {
            panic!();
        };
        Ok(x * y)
    }
    fn part2(&self, _lines: &[&[u8]]) -> Result<usize> {
        Ok(0)
    }
}

fn parse(lines: &[&[u8]]) -> Result<Vec<Monkey>> {
    lines
        .split(|&l| l.is_empty())
        .map(parse_monkey)
        .collect()
}

fn parse_monkey(lines: &[&[u8]]) -> Result<Monkey> {
    let items: VecDeque<usize> = std::str::from_utf8(&lines[1][18..])?
        .split(", ")
        .map(|w| w.parse())
        .collect::<Result<_,_>>()?;

    let operator = match lines[2][23] {
        b'+' => Operator::Plus,
        b'*' => Operator::Times,
        _ => bail!("parse error")
    };

    let operand = match std::str::from_utf8(&lines[2][25..])? {
        "old" => None,
        w => Some(w.parse::<usize>()?)
    };

    let op = move |x: usize| match (operator,operand) {
        (Operator::Plus,Some(y)) => { x + y },
        (Operator::Plus,_) => { x + x },
        (Operator::Times,Some(y)) => { x * y },
        (Operator::Times,_) => { x * x }
    };

    let op = Box::new(op);

    let divisor: usize = std::str::from_utf8(&lines[3][21..])?.parse()?;
    let monkey_true: usize = std::str::from_utf8(&lines[4][29..])?.parse()?;
    let monkey_false: usize = std::str::from_utf8(&lines[5][30..])?.parse()?;

    Ok(Monkey{items, op, divisor, monkey_true, monkey_false})
}
