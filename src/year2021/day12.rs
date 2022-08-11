use std::{fs};
use std::collections::HashSet;

enum CaveType {
    Start,
    End,
    Big,
    Small(bool),
}

struct Cave<'a> {
    id: String,
    cave_type: CaveType,
    connections: HashSet<&'a Cave<'a>>
}

impl Cave<'_> {
    fn new(id: &str, cave_type: CaveType, connections: HashSet<&Cave<'_>>) -> Cave<'static> {
        Cave {
            id: "a".to_string(), 
            cave_type: CaveType::Start, 
            connections: HashSet::new() 
        }
    }
}

fn get_input(test: bool) -> Cave<'static> {
    let input = match test {
        false => fs::read_to_string("input/year2021/day12.txt").unwrap(),
        true => fs::read_to_string("input/year2021/test12.txt").unwrap(),
    };

    let start_cave: Cave;
    let mut caves = HashSet::new();

    for line in input.lines() {
        let mut iter = line.split('-');
        let left_str = iter.next().unwrap();
        let right_str = iter.next().unwrap();

        
    }

    Cave::new("a", CaveType::Big, HashSet::new())
}

fn get_cave_type(label: &'static str) -> CaveType {
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
    let input = get_input(true);
    
}