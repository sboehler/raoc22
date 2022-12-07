use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use Cmd::*;

/**
Part 1:
*/
pub fn compute1(p: &Path) -> Result<usize> {
    let f = File::open(p)?;
    let commands: Vec<Cmd> = BufReader::new(f)
        .lines()
        .map(|res| res.map_err(dyn_err).and_then(|s| Cmd::parse(&s)))
        .collect::<Result<_>>()?;
    let mut cmds = commands.iter();
    let mut sizes = HashMap::new();
    let mut path = Vec::new();
    traverse(&mut cmds, &mut path, &mut sizes);
    let mut total = 0;
    for (_, v) in sizes {
        if v <= 100000 {
            total += v
        }
    }
    Ok(total)
}

/**
Part 2:
*/
pub fn compute2(p: &Path) -> Result<usize> {
    let commands: Vec<Cmd> = BufReader::new(File::open(p)?)
        .lines()
        .map(|res| res.map_err(dyn_err).and_then(|s| Cmd::parse(&s)))
        .collect::<Result<_>>()?;
    let mut path = Vec::new();
    let mut sizes = HashMap::new();
    let space_used = traverse(&mut commands.iter(), &mut path, &mut sizes);
    let capacity = 70000000;
    let reserved = 30000000;
    let required = space_used + reserved - capacity;
    let total = *sizes
        .values()
        .filter(|v| **v >= required)
        .min()
        .ok_or_else(|| "no directory found")?;
    Ok(total)
}

fn dyn_err(err: io::Error) -> Box<dyn Error> {
    let dyn_err: Box<dyn Error> = Box::new(err);
    dyn_err
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
        let cmd = match tokens.as_slice() {
            &["$", "cd", ".."] => CdUp,
            &["$", "cd", dir] => Cd(dir.into()),
            &["$", "ls"] => Ls,
            &["dir", name] => DirInfo(name.into()),
            &[size, name] => FileInfo(name.into(), size.parse::<usize>()?),
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
    fn test_day7_part1_example() {
        let s = Path::new("src/day7/example.txt");
        assert_eq!(compute1(&s).unwrap(), 95437)
    }

    #[test]
    fn test_day7_part1_input() {
        let s = Path::new("src/day7/input.txt");
        assert_eq!(compute1(&s).unwrap(), 1743217)
    }

    #[test]
    fn test_day7_part2_example() {
        let s = Path::new("src/day7/example.txt");
        assert_eq!(compute2(&s).unwrap(), 24933642)
    }

    #[test]
    fn test_day7_part2_input() {
        let s = Path::new("src/day7/input.txt");
        assert_eq!(compute2(&s).unwrap(), 8319096)
    }
}
