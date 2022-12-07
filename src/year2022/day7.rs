use std::{fs::read_to_string, collections::HashMap};

use slab_tree::*;

struct File {
    size: Option<u32>,
    _name: String
}

impl File {
    fn new(name: String, size: Option<u32>) -> File {
        File {
            size,
            _name: name,
        }
    }
}

enum Command {
    CD(String), LS
}

enum FileObject {
    File(File), Dir(String)
}

fn parse_input(test: bool) -> String {
    let path = if test {
        "input/year2022/test7.txt"
    } else {
        "input/year2022/day7.txt"
    }.to_string();
    read_to_string(path).unwrap()
}

fn create_dir_tree(input: String) -> Tree<(String, Vec<File>)> {
    let mut t = Tree::new();
    let mut id_table: HashMap<String, slab_tree::NodeId> = HashMap::new();

    id_table.insert(
        "/".to_string(), 
        t.set_root(("/".to_string(), vec![]))
    );
    
    let mut current_dir = "/".to_string();
    for line in input.lines() {
        if line.starts_with('$') {
            match parse_command(line) {
                Command::CD(cmd) => {
                    if cmd == ".." {
                        current_dir = move_up_one_dir(current_dir);
                    } else if cmd == "/" {
                        current_dir = "/".to_string();
                    } else {
                        current_dir.push_str(&cmd);
                        current_dir.push('/');
                    }
                },
                Command::LS => continue,
            }
        } else {
            match parse_non_command(line) {
                FileObject::File(f) => {
                    t.get_mut(*id_table.get(&current_dir).unwrap()).unwrap().data().1.push(f);
                },
                FileObject::Dir(name) => {
                    let mut new_dir = current_dir.clone();
                    new_dir.push_str(&name);
                    new_dir.push('/');

                    let mut current_tree = t.get_mut(*id_table.get(&current_dir).unwrap()).unwrap();
                    id_table.insert(
                        new_dir.clone(), 
                        current_tree.append((new_dir, vec![])).node_id()
                    );
                },
            }
        }
    }

    t
}

fn move_up_one_dir(path: String) -> String {
    let array: Vec<char> = path.chars().collect();
    let mut saw_slash = false;
    for i in (0..array.len()).rev() {
        if array[i] == '/' && saw_slash {
            return path[0..=i].to_string();
        } else if array[i] == '/' {
            saw_slash = true;
        }
    }
    return String::new();
}

fn parse_command(cmd: &str) -> Command {
    if cmd.starts_with("$ cd") {
        return Command::CD(cmd[5..].to_string());
    } else if cmd.starts_with("$ ls") {
        return Command::LS;
    }

    panic!("No correct command given: {}", cmd);
}

fn parse_non_command(cmd: &str) -> FileObject {
    if cmd.starts_with("dir ") {
        return FileObject::Dir(cmd[4..].to_string());
    }
    let split: Vec<&str> = cmd.split_ascii_whitespace().collect();
    let size: u32 = split[0].parse().unwrap();
    return FileObject::File(File::new(split[1].to_string(), Some(size)))
}

fn calc_size(index: &mut HashMap<String, i32>, tree: NodeRef<(String, Vec<File>)>) -> i32 {
    let mut sum: i32 = tree.data().1.iter().fold(0, |accu, elem| accu + elem.size.unwrap() as i32);
    for child in tree.children() {
        sum += calc_size(index, child);
    }

    index.insert(tree.data().0.clone(), sum);
    return sum;
}

fn sum_big_dirs(map: HashMap<String, i32>) -> i32 {
    map.iter().filter_map(|elem| {
        if *elem.1 <= 100000 {
            // print!("{}; " , elem.1);
            Some(*elem.1)
        } else {
            None
        }
    }).sum()
}

fn find_smallest_dir_to_delete(map: HashMap<String, i32>) -> i32 {
    const DISK_SIZE: i32 = 70000000;
    const NEEDED_SPACE: i32 = 30000000;
    let used_space: i32 = *map.get("/").unwrap();
    map.iter().map(|e| *e.1).fold(i32::MAX, |mut accu, elem| {
        if used_space-elem <= DISK_SIZE-NEEDED_SPACE && elem < accu {
            accu = elem;
        }
        accu
    })
}

pub fn part_two(test: bool) {
    let input = create_dir_tree(parse_input(test));
    let mut map: HashMap<String, i32> = HashMap::new();
    calc_size(&mut map, input.root().unwrap());

    println!("The dir to delete is {} big", find_smallest_dir_to_delete(map));
}

pub fn part_one(test: bool) {
    let input = create_dir_tree(parse_input(test));
    let mut map: HashMap<String, i32> = HashMap::new();
    calc_size(&mut map, input.root().unwrap());

    println!("The size is {}", sum_big_dirs(map));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_moving_up_dirs() {
        assert_eq!("/Users/theodor/rust/", move_up_one_dir("/Users/theodor/rust/aoc/".to_string()));
        assert_eq!(String::new(), move_up_one_dir("/".to_string()));
    }
}