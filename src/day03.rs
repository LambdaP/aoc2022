use crate::{Aoc, Day03, Result};

impl Aoc<u32> for Day03 {
    fn part1(&self, lines: &[&[u8]]) -> Result<u32> {
        let mut res = 0;
        for l in lines {
            let mut count: Vec<u32> = vec![0; 128];
            let (left, right) = l.split_at(l.len() / 2);
            for x in left {
                count[*x as usize] += 1;
            }
            for y in right {
                if count[*y as usize] > 0 {
                    let priority = if *y < b'a' {
                        *y - b'A' + 27
                    } else {
                        *y - b'a' + 1
                    };
                    res += priority as u32;
                    break;
                }
            }
        }
        Ok(res)
    }

    fn part2(&self, lines: &[&[u8]]) -> Result<u32> {
        let mut res = 0;
        for ll in lines.chunks(3) {
            let mut counts: Vec<Vec<u32>> = Vec::new();
            for &l in ll {
                let mut count = vec![0; 128];
                for x in l {
                    count[*x as usize] += 1;
                }
                counts.push(count);
            }
            for i in 0..128_u8 {
                if counts.iter().all(|c| c[i as usize] > 0) {
                    let priority = if i < b'a' {
                        i - b'A' + 27
                    } else {
                        i - b'a' + 1
                    };
                    res += priority as u32;
                    break;
                }
            }
        }
        Ok(res)
    }
}
