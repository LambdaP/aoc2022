use crate::{bail, Aoc, Day06, Result};

impl Aoc<u32> for Day06 {
    fn part1(&self, lines: &[&[u8]]) -> Result<u32> {
        let l = lines[0];
        let mut count = 4;
        for w in l.windows(4) {
            if all_different(w) {
                return Ok(count);
            }
            count += 1;
        }
        bail!("No result could be found");
    }
    fn part2(&self, lines: &[&[u8]]) -> Result<u32> {
        let l = lines[0];
        let mut count = 14;
        for w in l.windows(14) {
            if all_different(w) {
                return Ok(count);
            }
            count += 1;
        }
        bail!("No result could be found");
    }
}

fn all_different(bytes: &[u8]) -> bool {
    let mut counter = [0_u8; 256];
    for &b in bytes {
        counter[b as usize] += 1;
    }
    counter[b'a' as usize..=b'z' as usize]
        .iter()
        .all(|c| *c <= 1)
}
