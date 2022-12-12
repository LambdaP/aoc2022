use crate::{bail, Aoc, Day7, Result};

use std::collections::hash_map::HashMap;

type Fid = usize;

#[derive(Clone, PartialEq)]
struct File {
    id: Fid,
    parent_id: Fid,
    children: Option<HashMap<String, Fid>>,
    name: String,
    is_dir: bool,
    size: Option<usize>,
}

impl File {
    fn dir(name: &str, id: Fid, parent_id: Fid) -> File {
        File {
            id,
            parent_id,
            name: name.to_string(),
            is_dir: true,
            children: Some(HashMap::new()),
            size: None,
        }
    }

    fn file(name: &str, id: Fid, parent_id: Fid, size: usize) -> File {
        File {
            id,
            parent_id,
            name: name.to_string(),
            children: None,
            is_dir: false,
            size: Some(size),
        }
    }
}

impl Aoc<usize> for Day7 {
    fn part1(&self, lines: &[&[u8]]) -> Result<usize> {
        let mut files = parse(lines)?;
        compute_size(&mut files, 0)?;
        let res = files
            .iter()
            .filter_map(|f| if f.is_dir { f.size } else { None })
            .filter(|&u| u <= 100000)
            .sum::<usize>();
        Ok(res)
    }
    fn part2(&self, lines: &[&[u8]]) -> Result<usize> {
        let mut files = parse(lines)?;
        compute_size(&mut files, 0)?;
        let root_size = files[0].size.unwrap();
        let free_space = 70000000 - root_size;
        let needed = 30000000 - free_space;
        let res = files
            .iter()
            .filter_map(|f| if f.is_dir { f.size } else { None })
            .filter(|&u| u >= needed)
            .min()
            .unwrap();
        Ok(res)
    }
}

fn parse(lines: &[&[u8]]) -> Result<Vec<File>> {
    let mut files: Vec<File> = vec![];
    let mut pwd = 0;
    files.push(File::dir("/", 0, 0));

    for &line in lines {
        let line = std::str::from_utf8(line)?;
        let words = line.split_ascii_whitespace().collect::<Vec<&str>>();
        if words[0] == "$" {
            match words[1] {
                "cd" => match words[2] {
                    "/" => pwd = 0,
                    ".." => pwd = files[pwd].parent_id,
                    s => {
                        pwd = *(files[pwd].children.as_ref().unwrap().get(s).unwrap());
                    }
                },
                "ls" => (),
                _ => bail!("invalid command"),
            };
            continue;
        } else if words[0] == "dir" {
            let fid = files.len();
            let children = files[pwd].children.as_mut().unwrap();
            if !children.contains_key(words[1]) {
                children.insert(words[1].to_string(), fid);
                files.push(File::dir(words[1], fid, pwd));
            }
        } else {
            let size = words[0].parse::<usize>()?;
            let fid = files.len();
            let children = files[pwd].children.as_mut().unwrap();
            if !children.contains_key(words[1]) {
                children.insert(words[1].to_string(), fid);
                files.push(File::file(words[1], fid, pwd, size));
            }
        }
    }

    Ok(files)
}

fn compute_size(files: &mut [File], fid: Fid) -> Result<usize> {
    if let Some(size) = files[fid].size {
        return Ok(size);
    }
    let children = files[fid].children.clone().unwrap();
    let size = children
        .values()
        .map(|&v| compute_size(files, v))
        .sum::<Result<usize>>()?;
    files[fid].size = Some(size);
    Ok(size)
}
