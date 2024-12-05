#![allow(dead_code)]

use std::fs;

fn parse_input(test: bool) -> Vec<Vec<i32>> {
    let path = if test {
        "input/year2024/test2.txt".to_string()
    } else {
        "input/year2024/day2.txt".to_string()
    };

    let input = fs::read_to_string(path).unwrap();

    let mut list: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let chars = line.split_ascii_whitespace().map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        list.push(chars);
    }

    list
}

pub fn part_one(_test: bool) {
    let input = parse_input(false);

    let is_decreasing = |list : &Vec<i32>| -> bool {
        for i in 0..list.len()-1 {
            if list[i] < list[i+1] || (list[i]-list[i+1]).abs() > 3 || (list[i]-list[i+1]).abs() < 1 {
                return false;
            }
        }
        true
    };

    let is_increasing = |list : &Vec<i32>| -> bool {
        for i in 0..list.len()-1 {
            if list[i] > list[i+1] || (list[i]-list[i+1]).abs() > 3 || (list[i]-list[i+1]).abs() < 1 {
                return false;
            }
        }
        true
    };


    let result = input.into_iter().filter(|list| is_decreasing(list) || is_increasing(list)).count();
    println!("Result: {}", result);
}

// > 710
// nicht 739
pub fn part_two(_test: bool) {
    let input = parse_input(false);

    let is_decreasing_no_remove = |list : &Vec<i32>| -> bool {
        for i in 0..list.len()-1 {
            if list[i] < list[i+1] || (list[i]-list[i+1]).abs() > 3 || (list[i]-list[i+1]).abs() < 1 {
                return false;
            }
        }
        true
    };

    let is_increasing_no_remove = |list : &Vec<i32>| -> bool {
        for i in 0..list.len()-1 {
            if list[i] > list[i+1] || (list[i]-list[i+1]).abs() > 3 || (list[i]-list[i+1]).abs() < 1 {
                return false;
            }
        }
        true
    };


    let is_decreasing = |list : &Vec<i32>| -> bool {
        let mut copy = list.clone();

        let mut removed = false;
        let mut i = 0;
        while i < copy.len()-1 {
            if copy[i] < copy[i+1] || (copy[i]-copy[i+1]).abs() > 3 || (copy[i]-copy[i+1]).abs() < 1 {
                if removed { return false; }

                removed = true;
                copy.remove(i+1);
                continue;
            }
            i += 1;
        }
        true
    };

    let is_increasing = |list : &Vec<i32>| -> bool {
        let mut copy = list.clone();

        let mut removed = false;
        let mut i = 0;
        while i < copy.len()-1 {
            if copy[i] > copy[i+1] || (copy[i]-copy[i+1]).abs() > 3 || (copy[i]-copy[i+1]).abs() < 1 {
                if removed { return false; }

                removed = true;
                copy.remove(i+1);
                continue;
            }
            i += 1;
        }
        true
    };


    let result = input.into_iter().filter(|list| {
        let copy_wo_first = list.clone().into_iter().skip(1).collect::<Vec<i32>>();
        is_decreasing(list) || is_increasing(list) || is_decreasing_no_remove(&copy_wo_first) || is_increasing_no_remove(&copy_wo_first)
    }).count();
    println!("Result: {}", result);
}

