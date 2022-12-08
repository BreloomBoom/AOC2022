use std::fs;

struct FS {
    root: FSNode,
}

impl FS {
    fn new() -> Self {
        Self {
            root: FSNode::mkdir("/"),
        }
    }

    fn parse(input: String) -> Self {
        let mut fs = FS::new();

        let mut path: Vec<&str> = vec![];
        for command in input.split('\n') {
            let args = command.split(' ').collect::<Vec<&str>>();
            if args[1] == "ls" || args[0] == "dir" || command.contains("/") {
                continue;
            } else if command.contains("..") {
                let _ = path.pop();
                continue;
            }

            if args[1] == "cd" {
                let dir = FSNode::mkdir(args[2]);
                fs.root.find_dir(path.clone()).add_child(dir);
                path.push(args[2]);
            } else {
                let file = FSNode::touch(args[1], args[0]);
                fs.root.find_dir(path.clone()).add_child(file);
            }
        }

        fs.root.add_sizes();
        fs
    }
}

struct FSNode {
    name: String,
    is_file: bool,
    size: usize,
    children: Vec<FSNode>,
}

impl FSNode {
    fn touch(name: &str, size: &str) -> Self {
        Self {
            name: name.to_string(),
            size: size.parse::<usize>().unwrap(),
            is_file: true,
            children: vec![],
        }
    }

    fn mkdir(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: 0,
            is_file: false,
            children: vec![],
        }
    }

    fn add_sizes(&mut self) {
        for child in self.children.iter_mut() {
            child.add_sizes();
            self.size += child.size;
        }
    }

    fn get_child(&mut self, name: &str) -> Option<&mut Self> {
        self.children.iter_mut().find(|child| child.name == name)
    }

    fn add_child(&mut self, child: FSNode) {
        if !self.is_file {
            self.children.push(child);
        }
    }

    fn find_dir(&mut self, path: Vec<&str>) -> &mut Self {
        let mut curr = self;
        for dir in path.into_iter() {
            curr = curr.get_child(dir).unwrap();
        }
        curr
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The sum of all directories less than 100000 is {}", part1(input.clone()));
    println!("The smallest dir with at least 30000000 has a size of {}", part2(input));
}

fn part1(input: String) -> usize {
    let file_sys = FS::parse(input);
    do_part1(file_sys.root)
}

fn do_part1(root: FSNode) -> usize {
    if root.is_file {
        return 0;
    }

    let mut size = 0;
    if root.size <= 100000 {
        size =  root.size;
    }
    size + root.children.into_iter().fold(0, |a, b| a + do_part1(b))
}

fn part2(input: String) -> usize {
    let file_sys = FS::parse(input);
    let mut smallest: usize = file_sys.root.size;
    let needed = file_sys.root.size - 40000000;
    do_part2(file_sys.root, &mut smallest, needed);
    smallest
}

fn do_part2(root: FSNode, smallest: &mut usize, needed: usize) {
    if root.size < *smallest && root.size >= needed {
        *smallest = root.size;
    }

    for dir in root.children.into_iter() {
        if !dir.is_file {
            do_part2(dir, smallest, needed);
        }
    }
}