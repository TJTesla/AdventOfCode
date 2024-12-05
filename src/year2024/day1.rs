#![allow(dead_code)]

use std::fs;
use itertools::Itertools;

fn parse_input(test: bool) -> (Vec<i32>, Vec<i32>) {
    let path = if test {
        "input/year2024/test1.txt".to_string()
    } else {
        "input/year2024/day1.txt".to_string()
    };

    let input = fs::read_to_string(path).unwrap();

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in input.lines() {
        let chars = line.split_ascii_whitespace().collect::<Vec<&str>>();
        list1.push(chars[0].parse().unwrap());
        list2.push(chars[1].parse().unwrap());
    }

    (list1, list2)
}

pub fn part_one(test: bool) {
    let (list1, list2) = parse_input(test);

    let result = list1.iter().sorted().zip(list2.iter().sorted()).fold(0, |acc, (a, b)| acc + (a-b).abs());
    println!("Result: {}", result);
}

pub fn part_two(_test: bool) {
    let (list1, list2) = parse_input(false);
    let list1 = list1.iter().sorted().collect::<Vec<&i32>>();
    let list2 = list2.iter().sorted().collect::<Vec<&i32>>();

    let mut left = 0;
    let mut right = 0;

    let mut result = 0;
    while left < list1.len() {
        let mut counter = 0;
        while right < list2.len() && list2[right] <= list1[left] {
            if list2[right] == list1[left] {
                counter += 1;
            }
            right += 1;
        }

        let mut shift = 0;
        while left+shift < list1.len() && list1[left+shift] == list1[left] {
            shift += 1;
        }

        result += list1[left] * counter * shift as i32;
        left += shift;
    }

    println!("Result: {}", result);
}