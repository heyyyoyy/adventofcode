use std::iter::Peekable;

struct Dir(Vec<Dir>, usize);


fn size_counter(lines: &mut Peekable<impl Iterator<Item = &'static str>>) -> Dir {
    let (mut dirs, mut size) = (vec![], 0);
    while let Some(line) = lines.next() {
        match line {
            "$ cd .." => break,
            _ if line.starts_with("$ l") => {
                size = std::iter::from_fn(|| lines.next_if(|line| !line.starts_with('$')))
                    .filter(|line| !line.starts_with('d'))
                    .map(|line| line.split(' ').next().unwrap().parse::<usize>().unwrap())
                    .sum()
            }
            _ => dirs.push(size_counter(lines)),
        }
    }
    size += dirs.iter().map(|d| d.1).sum::<usize>();
    Dir(dirs, size)
}


fn search(d: &Dir, min: usize) -> Option<usize> {
    d.0.iter()
        .filter(|d| d.1 >= min)
        .flat_map(|d| [Some(d.1), search(d, min)])
        .flatten()
        .min()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_counter() {
        let input_str = "$ cd /
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
7214296 k";
        let root = size_counter(&mut input_str.lines().peekable());
        let res = search(&root, root.1 - 40_000_000).unwrap();
        assert_eq!(24933642, res);
    }

    #[test]
    fn test_size_counter_from_file() {
        let root = size_counter(&mut include_str!("input.txt").lines().peekable());
        let res = search(&root, root.1 - 40_000_000).unwrap();
        assert_eq!(5469168, res);
    }
}
