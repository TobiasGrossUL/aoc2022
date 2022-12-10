use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct DirRepr {
    parent: Weak<RefCell<DirRepr>>,
    files: HashMap<String, u64>,
    dirs: HashMap<String, Rc<RefCell<DirRepr>>>,
}

impl DirRepr {
    fn new(parent: Weak<RefCell<DirRepr>>) -> DirRepr {
        return DirRepr { parent, files: HashMap::new(), dirs: HashMap::new() }
    }

    fn new_rc(parent: Weak<RefCell<DirRepr>>) -> Rc<RefCell<DirRepr>> {
        return Rc::new(RefCell::new(DirRepr::new(parent)));
    }

    fn add_dir(&mut self, name: String, parent: Weak<RefCell<DirRepr>>) -> Rc<RefCell<DirRepr>> {
        let new_dir = DirRepr::new_rc(parent);
        self.dirs.insert(name, new_dir.clone());
        return new_dir;
    }

    fn sum_of_files(&self) -> u64 {
        let mut sum = 0;
        sum += self.dirs.values().map(|f| f.borrow().sum_of_files()).sum::<u64>();
        sum += self.files.values().sum::<u64>();
        return sum;
    }

    fn sum_smaller(&self, limit: u64) -> u64 {
        let child_sum = self.dirs.values().map(|f| f.borrow().sum_smaller(limit)).sum::<u64>();
        let own_sum = self.sum_of_files();
        if own_sum <= limit {
            return child_sum + own_sum;
        } else {
            return child_sum;
        }
    }

    fn bigger(&self, limit: u64) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        for childf in self.dirs.values() {
            let mut sizes = childf.borrow().bigger(limit);
            result.append(&mut sizes);
        }
        let own_size = self.sum_of_files();
        if own_size >= limit {
            result.push(own_size);
        }
        return result;
    }
}

fn part_one(root: &DirRepr) -> u64 {
    let sum = root.sum_smaller(100000);
    println!("Solution part 1: {}", sum);
    return sum;
}

fn part_two(root: &DirRepr) -> u64 {
    let used_space = root.sum_of_files();
    let total_space = 70000000;
    let free_space = total_space - used_space;
    let required_space = 30000000 - free_space;

    let mut sizes = root.bigger(required_space);
    sizes.sort();
    println!("Solution part 2: {} ", sizes[0]);
    return sizes[0];
}

fn main() {
    let root = build_fs_tree();
    part_one(&root.borrow());
    part_two(&root.borrow());
}

fn to_root(cwd: Weak<RefCell<DirRepr>>) -> Weak<RefCell<DirRepr>> {
    let cwd_p = cwd.upgrade().unwrap();
    let cwd_r = cwd_p.borrow();
    let parent = cwd_r.parent.clone();
    if parent.upgrade().is_none() {
        return cwd;
    } else {
        return to_root(parent);
    }
}

fn handle_cd(cwd: Weak<RefCell<DirRepr>>, dst: &str) -> Weak<RefCell<DirRepr>> {
    match dst {
        "/" => {
            return to_root(cwd);
        },
        ".." => {
            return cwd.upgrade().unwrap().borrow().parent.clone();
        }
        other => {
            return Rc::downgrade(cwd.upgrade().unwrap().borrow().dirs.get(&String::from(other)).unwrap());
        }
    }
}



fn handle_ls(cwd: Weak<RefCell<DirRepr>>, result: &[String]) {
    for res in result {
        let mut tokens = res.split(" ");
        let size = tokens.nth(0).expect(res);
        let name = tokens.nth(0).unwrap();
        match size {
            "dir" => {
                let cwd_p = cwd.upgrade().expect(res);
                let mut current_dir = cwd_p.borrow_mut();
                current_dir.add_dir(name.to_string(), Rc::downgrade(&cwd_p));
            },
            _ => {
                let size = size.parse::<u64>().unwrap();
                cwd.upgrade().unwrap().borrow_mut().files.insert(name.to_string(), size);
            }
        }
    }
}

fn handle_command(command_and_result: &CommandAndResult, cwd: Weak<RefCell<DirRepr>>) -> Weak<RefCell<DirRepr>> {
    let command = &command_and_result[0];
    let result = &command_and_result[1..];

    let mut tokens = command.split_whitespace();
    let command = tokens.nth(1).unwrap();
    let argument = tokens.nth(0);
    match command {
        "cd" => {
            return handle_cd(cwd.clone(), &argument.unwrap().to_string());
        },
        "ls" => {handle_ls(cwd.clone(), result); return cwd;},
        _ => panic!("Unknown command")
    }
}


fn build_fs_tree() -> Rc<RefCell<DirRepr>> {
    let lines = parse_input();
    let root = DirRepr::new_rc(Weak::new());
    let mut cwd = Rc::downgrade(&root);
    for command_and_result in lines {
        cwd = handle_command(&command_and_result, cwd.clone());
    }
    return root;
}

type CommandAndResult = Vec<String>;

fn parse_input() -> Vec<CommandAndResult> {
    let mut result = Vec::new();
    let mut command = Vec::new();
    for line in read_lines("input").unwrap() {
        if let Ok(linedata) = line {
            if linedata.chars().nth(0).unwrap() == '$' {
                result.push(command);
                command = Vec::new();
            }
            command.push(linedata);
        }
    }
    result.remove(0);
    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let root = build_fs_tree();
        let solution = part_one(&root.borrow());
        assert_eq!(solution, 1232307);
    }

    #[test]
    fn part2_output() {
        let root = build_fs_tree();
        let solution = part_two(&root.borrow());
        assert_eq!(solution, 7268994);
    }

}
