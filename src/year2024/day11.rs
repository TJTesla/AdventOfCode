#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;

fn parse_input(test: bool) -> Vec<u64> {
    let path = if test {
        "input/year2024/test11.txt"
    } else {
        "input/year2024/day11.txt"
    };
    let input = fs::read_to_string(path).unwrap();

    input.split_ascii_whitespace().map(|str| str.parse().unwrap()).collect()
}

fn divide_num(num: u64) -> Option<(u64, u64)> {
    let num_str = num.to_string();
    if num_str.len() % 2 != 0 {
        return None;
    }

    let stone1 = num_str[0..(num_str.len()/2)].to_string().parse().unwrap();
    let stone2 = num_str[(num_str.len()/2)..].to_string().parse().unwrap();

    Some((stone1, stone2))
}

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();

    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else if let Some((stone1, stone2)) = divide_num(stone) {
            new_stones.push(stone1);
            new_stones.push(stone2);
        } else {
            new_stones.push(stone*2024);
        }
    }

    new_stones
}

fn blink2(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();

    for (stone, amount) in stones {
        if stone == 0 {
            *new_stones.entry(1).or_insert(0) += amount;
        } else if let Some((stone1, stone2)) = divide_num(stone) {
            *new_stones.entry(stone1).or_insert(0) += amount;
            *new_stones.entry(stone2).or_insert(0) += amount;
        } else {
            *new_stones.entry(stone*2024).or_insert(0) += amount;
        }
    }

    new_stones
}

pub fn part_one(_test: bool) {
    let mut input = parse_input(false);

    // let blink_amount = 25;
    // Part two:
    let blink_amount = 75;

    for i in 0..blink_amount {
        input = blink(input);
        println!("{}: {}", i, input.len());
    }

    println!("Result: {}", input.len());
}

pub fn part_two(_test: bool) {
    let input = parse_input(false);
    let mut stones: HashMap<u64, u64> = input.into_iter().fold(HashMap::new(), |mut acc, s| {
        *acc.entry(s).or_insert(0) += 1; acc
    });

    let blink_amount = 75;

    for _ in 0..blink_amount {
        stones = blink2(stones);
    }

    println!("Result: {}", stones.into_iter().map(|(_, v)| v).sum::<u64>());
}