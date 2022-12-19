use crate::{bail, Aoc, Day06, Display, FileRep, Result};

impl Aoc for Day06 {
    fn part1(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.byte_lines;
        let l = lines[0];
        let mut count = 4;
        for w in l.windows(4) {
            if all_different(w) {
                return result!(count);
            }
            count += 1;
        }
        bail!("No result could be found");
    }

    fn part2(&self, input: &FileRep) -> Result<Box<dyn Display>> {
        let lines = &input.byte_lines;
        let l = lines[0];
        let mut count = 14;
        for w in l.windows(14) {
            if all_different(w) {
                return result!(count);
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
