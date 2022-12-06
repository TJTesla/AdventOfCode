#![allow(dead_code)]
use std::{fs::read_to_string, collections::HashSet};

fn parse_input(test: bool) -> Vec<Vec<char>> {
    let path = if test {
        "input/year2022/test6.txt".to_string()
    } else {
        "input/year2022/day6.txt".to_string()
    };
    let input = read_to_string(path).unwrap();

    let mut result = Vec::new();
    for line in input.lines() {
        let mut temp = Vec::new();
        for c in line.chars() {
            temp.push(c);
        }
        result.push(temp);
    }

    result
}

fn find_token_end_index(signal: &Vec<char>) -> Option<usize> {
    let mut set = HashSet::new();
    for i in 0..signal.len()-4 {
        set.clear();
        for j in 0..4 {
            set.insert(signal.get(i+j)?);
        }
        if set.len() == 4 {
            return Some(i+4);
        }
    }

    Some(0)
}

fn find_msg_end_index(signal: &Vec<char>) -> Option<usize> {
    let mut set = HashSet::new();
    for i in 0..signal.len()-14 {
        set.clear();
        for j in 0..14 {
            set.insert(signal.get(i+j)?);
        }
        if set.len() == 14 {
            return Some(i+14);
        }
    }

    Some(0)
}

pub fn part_two(test: bool) {
    let input = parse_input(test);

    for line in input {
        println!("Signal: {}; First marker: {}", line.iter().collect::<String>(), find_msg_end_index(&line).unwrap());
    }
}

pub fn part_one() {
    let input = parse_input(false);

    for line in input {
        println!("Signal: {}; First marker: {}", line.iter().collect::<String>(), find_token_end_index(&line).unwrap());
    }
}