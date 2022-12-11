use std::{collections::HashMap, rc::Rc, cell::RefCell};


#[derive(Default)]
struct Dir {
    name: String,
    size: RefCell<u64>,
    parent: Option<Rc<Dir>>,
    childrens: RefCell<HashMap<String, Rc<Dir>>>
}

impl Dir {
    fn parse_input(input_str: &str) -> Rc<Dir> {
        let root = Rc::new(Dir {
             name: "/".to_string(), 
             size: RefCell::new(0), 
             parent: None, 
             childrens: RefCell::new(HashMap::new()) 
            });
        let mut cwd = Rc::new(Dir::default());
        for line in input_str.lines() {
            let command = line.split_whitespace().collect::<Vec<&str>>();
            match (command[0], command[1]) {
                ("$", "cd") => {
                    match command[2] {
                        "/" => cwd = Rc::clone(&root),
                        ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
                        dir => {
                            let new_dir = cwd.childrens.borrow().get(dir).unwrap().clone();
                            cwd = new_dir;
                        },
                    }
                },
                ("$", "ls") => {},
                ("dir", dir) => {
                    cwd.childrens.borrow_mut().insert(
                        dir.to_string(), 
                        Rc::new(Dir {
                             name: dir.to_string(), 
                             size: RefCell::new(0), 
                             parent: Some(Rc::clone(&cwd)), 
                             childrens: RefCell::new(HashMap::new()) 
                            })
                    );
                },
    
                (size, _) => {
                    *cwd.size.borrow_mut() += size.parse::<u64>().unwrap();
                },
            }
        }
        root
    }

    fn size(&self) -> u64 {
        *self.size.borrow() + self.childrens.borrow().values().fold(0, |x, y| x + y.size())
    }
}


fn size_counter(input_str: &str) -> u64 {
    let root = Dir::parse_input(input_str);
    let total_size = root.size();
    let free_space = 70000000 - total_size;
    let space_needed = 30000000 - free_space;

    let mut arr = vec![Rc::clone(&root)];
    let mut min_space = u64::MAX;

    while let Some(dir) = arr.pop() {
        arr.extend(dir.childrens.borrow().values().map(Rc::clone));
        let size = dir.size();
        if size >= space_needed {
            min_space = min_space.min(size);
        }
    }
    min_space
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
        assert_eq!(24933642, size_counter(input_str));
    }

    #[test]
    fn test_size_counter_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(5469168, size_counter(input_str));
    }
}
