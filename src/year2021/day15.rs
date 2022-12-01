#![allow(dead_code)]

use std::cmp::Ordering;
use std::fs;
use priority_queue;
use priority_queue::PriorityQueue;

#[derive(Eq, PartialEq, Hash)]
struct Path {
    previous_fields: Vec<(u8, u8)>,
    weights: u32,
}

impl Path {
    fn new() -> Path {
        Path {
            previous_fields: vec![],
            weights: 0
        }
    }

    fn new_from_field(f: (u8, u8)) -> Path {
        Path {
            previous_fields: vec![f],
            weights: 0
        }
    }

    fn add_field(&mut self, field: (u8, u8), weight: u32) {
        self.previous_fields.push(field);
        self.weights += weight;
    }

    fn visited(&self, list: &Vec<Path>) -> bool {
        for p in list {
            if p == self {
                return true;
            }
        }
        false
    }
}

impl PartialOrd<Self> for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.weights > other.weights {
            return Some(Ordering::Greater)
        }
        if self.weights < other.weights {
            return Some(Ordering::Less)
        }
        if self.weights == other.weights {
            return Some(Ordering::Equal)
        }
        None
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.weights > other.weights {
            return Ordering::Greater
        }
        if self.weights < other.weights {
            return Ordering::Less
        }
        Ordering::Equal
    }
}

fn parse_input(test: bool) -> Vec<Vec<u8>> {
    let file_content = if test {
        fs::read_to_string("input/year2021/test15.txt").unwrap()
    } else {
        fs::read_to_string("input/year2021/test16.txt").unwrap()
    };

    let mut result = Vec::new();
    for line in file_content.lines() {
        let mut v: Vec<u8> = Vec::new();
        for c in line.chars() {
            v.push(c.to_string().parse().unwrap());
        }
        result.push(v);
    }

    return result;
}

fn dijkstra(net: Vec<Vec<(u8, u8)>>) -> Path {
    let mut prio_q: PriorityQueue<Path, u32> = priority_queue::PriorityQueue::new();
    prio_q.push(Path::new_from_field(net[0][0]), 0);

    loop {
        break;
    }

    return Path::new();
}

pub fn part_one() {

}