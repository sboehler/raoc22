use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use Cmd::*;

/**
Part 1:
*/
pub fn compute1(p: &Path) -> Result<usize> {
    let cmds: Vec<Cmd> = File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|lines| {
            lines
                .map(|res| res.map_err(io::Error::into).and_then(|s| Cmd::parse(&s)))
                .collect()
        })?;
    let mut sizes = HashMap::new();
    let mut path = Vec::new();
    traverse(&mut cmds.iter(), &mut path, &mut sizes);
    Ok(sizes.values().filter(|v| **v <= 100000).sum())
}

/**
Part 2:
*/
pub fn compute2(p: &Path) -> Result<usize> {
    let cmds: Vec<Cmd> = File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|lines| {
            lines
                .map(|res| res.map_err(io::Error::into).and_then(|s| Cmd::parse(&s)))
                .collect()
        })?;
    let mut path = Vec::new();
    let mut sizes = HashMap::new();
    let space_used = traverse(&mut cmds.iter(), &mut path, &mut sizes);
    let capacity = 70000000;
    let reserved = 30000000;
    let required = space_used + reserved - capacity;
    Ok(*sizes
        .values()
        .filter(|v| **v >= required)
        .min()
        .ok_or("no directory found")?)
}

fn traverse<'a, I>(
    cmds: &mut I,
    path: &mut Vec<String>,
    sizes: &mut HashMap<String, usize>,
) -> usize
where
    I: Iterator<Item = &'a Cmd>,
{
    let mut size = 0;
    loop {
        let cmd = cmds.next();
        match cmd {
            Some(Cd(name)) => {
                path.push(name.into());
                let s = traverse(cmds, path, sizes);
                sizes.insert(path.join("/"), s);
                path.pop();
                size += s;
            }
            Some(FileInfo(_, s)) => size += s,
            Some(DirInfo(_)) | Some(Ls) => continue,
            None | Some(CdUp) => return size,
        };
    }
}

enum Cmd {
    Cd(String),
    CdUp,
    Ls,
    DirInfo(String),
    FileInfo(String, usize),
}

impl Cmd {
    pub fn parse(s: &str) -> Result<Self> {
        let tokens = s.split_whitespace().into_iter().collect::<Vec<&str>>();
        let cmd = match *tokens.as_slice() {
            ["$", "cd", ".."] => CdUp,
            ["$", "cd", dir] => Cd(dir.into()),
            ["$", "ls"] => Ls,
            ["dir", name] => DirInfo(name.into()),
            [size, name] => FileInfo(name.into(), size.parse::<usize>()?),
            _ => return Err(format!("invalid command: {}", s).into()),
        };
        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day07_part1_example() {
        let s = Path::new("src/inputs/day07_example.txt");
        assert_eq!(compute1(&s).unwrap(), 95437)
    }

    #[test]
    fn day07_part1_input() {
        let s = Path::new("src/inputs/day07_input.txt");
        assert_eq!(compute1(&s).unwrap(), 1743217)
    }

    #[test]
    fn day07_part2_example() {
        let s = Path::new("src/inputs/day07_example.txt");
        assert_eq!(compute2(&s).unwrap(), 24933642)
    }

    #[test]
    fn day07_part2_input() {
        let s = Path::new("src/inputs/day07_input.txt");
        assert_eq!(compute2(&s).unwrap(), 8319096)
    }
}
