#![allow(dead_code)]
use std::fs::read_to_string;

fn parse_input(test: bool) -> Vec<((u32, u32), (u32, u32))> {
    let path = if test {
        "input/year2022/test4.txt".to_string()
    } else {
        "input/year2022/day4.txt".to_string()
    };

    let input = read_to_string(path).unwrap();

    let mut result = Vec::new();
    for line in input.lines() {
        let mut elf_iter = line.split(',');
        let left = elf_iter.next().unwrap();
        let right = elf_iter.next().unwrap();

        result.push(
            (parse_group(left), parse_group(right))
        );
    }

    result
}

fn parse_group(g: &str) -> (u32, u32) {
    let mut iter = g.split('-');
    let left_num = iter.next().unwrap();
    let right_num = iter.next().unwrap();

    (left_num.parse().unwrap(), right_num.parse().unwrap())
}

fn get_overlap(left: (u32, u32), right: (u32, u32)) -> bool {
    if
        right.0 >= left.0 && right.1 <= left.1 
        || left.0 >= right.0 && left.1 <= right.1
    {
        return true;
    }
    false
}

fn get_overlap_part_two(left: (u32, u32), right: (u32, u32)) -> bool {
    if left.0 >= right.0 && left.0 <= right.1
        || right.0 >= left.0 && right.0 <= left.1 
    {
        return true;
    }
    false
}

pub fn part_two() {
    let input = parse_input(false);

    let mut counter = 0;
    for i in input {
        if get_overlap_part_two(i.0, i.1) {
            counter += 1;
        }
    }

    println!("There are {} overlaps", counter);
}

pub fn part_one() {
    let input = parse_input(false);
    
    let mut counter = 0;
    for i in input {
        if get_overlap(i.0, i.1) {
            counter += 1;
        }
    }

    println!("There are {} overlaps", counter);
}