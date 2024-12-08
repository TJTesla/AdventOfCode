#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::iproduct;

fn parse_input(test: bool) -> (HashMap<char, HashSet<(i32, i32)>>, usize, usize) {
    let path = if test {
        "input/year2024/test8.txt"
    } else {
        "input/year2024/day8.txt"
    };
    let input = fs::read_to_string(path).unwrap();

    let mut res = HashMap::new();

    let mut row = 0;
    for line in input.lines() {
        let mut col = 0;
        for c in line.chars() {
            if c != '.' && !c.is_whitespace() {
                res.entry(c).or_insert(HashSet::new()).insert((row, col));
            }

            col += 1;
        }

        row += 1;
    }


    (res, input.lines().count(), input.lines().next().unwrap().len())
}

fn is_correct_coord(coord: (i32, i32), length: usize, width: usize) -> bool {
    if coord.0 < 0 || coord.1 < 0 {
        return false;
    }

    if coord.0 >= length as i32 || coord.1 >= width as i32 {
        return false;
    }

    true
}

pub fn part_one(_test: bool) {
    let (layout, length, width) = parse_input(false);

    let mut locations: HashSet<(i32, i32)> = HashSet::new();

    for (_, antennas) in layout {
        for (p1, p2) in iproduct!(antennas.iter(), antennas.iter()) {
            if p1 == p2 { continue; }
            let delta = (p2.0 - p1.0, p2.1 - p1.1);

            let new_coord = (p2.0 + delta.0, p2.1 + delta.1);
            if is_correct_coord(new_coord, length, width) {
                locations.insert(new_coord);
            }
        }
    }

    println!("Result: {}", locations.len());
}

pub fn part_two(_test: bool) {
    let (layout, length, width) = parse_input(false);
    let mut locations: HashSet<(i32, i32)> = HashSet::new();

    for (_, antennas) in layout {
        for (p1, p2) in iproduct!(antennas.iter(), antennas.iter()) {
            if p1 == p2 { continue; }
            let delta = (p2.0 - p1.0, p2.1 - p1.1);

            let mut new_coord = *p2;
            while is_correct_coord(new_coord, length, width) {
                locations.insert(new_coord);
                new_coord = (new_coord.0 + delta.0, new_coord.1 + delta.1);
            }
        }
    }

    println!("Result: {}", locations.len());
}