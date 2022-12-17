const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[derive(Debug)]
enum Line<'a> {
    CdEnter(&'a str),
    CdExit,
    Ls,
    File(usize, &'a str),
    Directory(&'a str),
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(l: &'a str) -> Self {
        if l.starts_with("$ cd ..") {
            Line::CdExit
        } else if let Some(ll) = l.strip_prefix("$ cd ") {
            Line::CdEnter(ll)
        } else if l.starts_with("$ ls") {
            Line::Ls
        } else if let Some(ll) = l.strip_prefix("dir ") {
            Line::Directory(ll)
        } else {
            let mut m = l.split(' ');
            let size = m.next().unwrap();
            let name = m.next().unwrap();
            Line::File(size.parse().unwrap(), name)
        }
    }
}

fn parse(cmds: &str) -> impl Iterator<Item = Line> {
    cmds.lines().skip(1).map(Line::from)
}

fn visit<'a, F>(it: &mut impl Iterator<Item = Line<'a>>, visitor: &mut F) -> usize
where
    F: FnMut(usize),
{
    let mut dsize = 0usize;

    loop {
        match it.next() {
            Some(Line::CdEnter(_)) => dsize += visit(it, visitor),
            Some(Line::CdExit) | None => break,
            Some(Line::Ls) => (),
            Some(Line::File(size, _)) => dsize += size,
            Some(Line::Directory(_)) => (),
        }
    }

    visitor(dsize);
    dsize
}

fn run1(input: &str) -> usize {
    let mut cumulative_size = 0usize;
    visit(&mut parse(input), &mut |u| {
        if u < 100000 {
            cumulative_size += u;
        }
    });
    cumulative_size
}

fn run2(input: &str) -> usize {
    const TOTAL_SPACE: usize = 70000000;
    const NEEDED_SPACE: usize = 30000000;

    let used_space = visit(&mut parse(input), &mut |_| {});

    let avail_space = TOTAL_SPACE - used_space;
    let mut deleted_size = usize::MAX;

    visit(&mut parse(input), &mut |u| {
        if u < deleted_size && (avail_space + u) > NEEDED_SPACE {
            deleted_size = u
        }
    });

    deleted_size
}

fn main() {
    dbg!("{}", run1(INPUT.trim()));
    dbg!("{}", run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), 95437);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE01), 24933642);
    }
}
