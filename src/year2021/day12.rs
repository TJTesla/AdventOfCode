#![allow(dead_code)]

/// Look at the code of the C++ Project in the root directory of this cargo project
pub fn  part_one() {
    println!("Look at the code of the C++ Project in the root directory of this cargo project");
}

/*
use std::{fs};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(PartialEq, Eq)]
enum CaveType {
    Start,
    End,
    Big,
    Small(bool),
}

#[derive(PartialEq, Eq)]
struct Cave<'a> {
    id: String,
    cave_type: CaveType,
    connections: HashSet<&'a Cave<'a>>
}

impl<'a> Cave<'a> {
    fn new(id: &str, cave_type: CaveType) -> Cave<'static> {
        Cave {
            id: id.clone().to_string(), 
            cave_type,
            connections: HashSet::new(),
        }
    }

    fn add_connection(&mut self, cave: &'a Cave) {
        self.connections.insert(cave);
    }
}

impl Hash for Cave<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn get_input(path: &str) -> HashMap<String, Cave> {
    let input = fs::read_to_string(path).expect("The given path isn't correct");

    let mut caves: HashMap<String, Cave> = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split('-');
        let left_str = iter.next().unwrap();
        let right_str = iter.next().unwrap();

        if !caves.contains_key(left_str) {
            caves.insert(
                left_str.to_string(),
                Cave::new(left_str, get_cave_type(left_str))
            );
        }
        if !caves.contains_key(right_str) {
            caves.insert(
                right_str.to_string(),
                Cave::new(right_str, get_cave_type(right_str))
            );
        }
        
        caves.entry(left_str.to_string()).and_modify(
            |c| {c.add_connection(caves.get(right_str).unwrap());}
        );
    }

    caves
}

fn get_cave_type(label: &str) -> CaveType {
    if label == "start" {
        return CaveType::Start;
    }

    if label == "end" {
        return CaveType::End;
    }

    let mut lowercase = true;
    for c in label.chars() {
        if c.is_ascii_uppercase() {
            lowercase = false;
        }
    }

    if lowercase {
        return CaveType::Small(false);
    } else {
        return CaveType::Big;
    }
}

pub fn part_one() {
    let input = get_input("input/year2021/day12.txt");
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn create() {

    }

   #[test]
    fn test1() {

    }
}
*/
