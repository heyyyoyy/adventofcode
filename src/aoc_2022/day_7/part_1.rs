use std::iter::Peekable;

fn size_counter(lines: &mut Peekable<impl Iterator<Item = &'static str>>, sum: &mut usize) -> usize {
    let mut size = 0;
    while let Some(line) = lines.next() {
        match line {
            "$ cd .." => break,
            _ if line.starts_with("$ l") => {
                size = std::iter::from_fn(|| lines.next_if(|line| !line.starts_with('$')))
                    .filter(|line| !line.starts_with('d'))
                    .map(|line| line.split(' ').next().unwrap().parse::<usize>().unwrap())
                    .sum()
            }
            _ => size += size_counter(lines, sum),
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
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
        let (mut lines, mut sum) = (input_str.lines().peekable(), 0);
        size_counter(&mut lines, &mut sum);
        assert_eq!(95437, sum);
    }

    #[test]
    fn test_size_counter_from_file() {
        let input_str = include_str!(r"input.txt");
        let (mut lines, mut sum) = (input_str.lines().peekable(), 0);
        size_counter(&mut lines, &mut sum);
        assert_eq!(1391690, sum);
    }
}
