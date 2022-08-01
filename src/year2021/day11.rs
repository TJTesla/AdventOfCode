#![allow(dead_code)]
use std::{fs, collections::HashSet};

fn get_input(test: bool) -> Vec<Vec<i32>> {
    let input = match test {
        false => fs::read_to_string("input/year2021/day11.txt").unwrap(),
        true => fs::read_to_string("input/year2021/test11.txt").unwrap(),
    };

    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.chars().map(|c| c.to_string().parse().unwrap()).collect());
    }

    result
}

pub fn part_two() {
    let mut input = get_input(false);

    let mut counter: u64 = 0;
    let step = loop {
        let resets = one_step(&mut input);
        counter += 1;

        if resets == input.len()*input[0].len() {
            break counter;
        }
    };

    println!("After {} steps, all octopuses flash simultaneously", step);
}

pub fn part_one() {
    let mut input = get_input(false);

    let mut total = 0;

    for _ in 0..100 {
        total += one_step(&mut input);
    }

    println!("After 100 steps, there have been a total of {} flashes", total);
}

fn one_step(arr: &mut Vec<Vec<i32>>) -> usize {
    for i in 0..arr.len() {
        for k in 0..arr[i].len() {
            arr[i][k] += 1;
        }
    }

    // let mut total = 0;
    let mut flashed = HashSet::new();
    for i in 0..arr.len() {
        for k in 0..arr[i].len() {
            //total += check_for_flash(arr, i, k, &mut flashed);
            check_for_flash(arr, i, k, &mut flashed);
        }
    }

    let mut total: usize = 0;
    for i in 0..arr.len() {
        for k in 0..arr[i].len() {
            if arr[i][k] > 9 {
                total += 1;
                arr[i][k] = 0;
            }
        }
    }

    total
}

fn check_for_flash(arr: &mut Vec<Vec<i32>>, x: usize, y: usize, flashed: &mut HashSet<(usize, usize)>) -> i32 {
    if let Some(_) = flashed.get(&(x, y)) {
        return 0;
    }
    if arr[x][y] >= 10 {
        let mut total = 1;
        arr[x][y] += 1;
        flashed.insert((x, y));

        let mut adjacents = Vec::new();
        for i in get_range(x, arr.len()) {
            for k in get_range(y, arr[i].len()) {
                if i == x && k == y {
                    continue;
                }

                adjacents.push((i, k));
            }
        }

        for pair in adjacents {
            arr[pair.0][pair.1] += 1;
            total += check_for_flash(arr, pair.0, pair.1, flashed);
        }

        return total;
    }

    0
}

fn get_range(num: usize, len: usize) -> std::ops::RangeInclusive<usize> {
    let num1: usize;
    if num == 0 {
        num1 = num;
    } else {
        num1 = num-1;
    }

    let num2: usize;
    if num == len-1 {
        num2 = num;
    } else {
        num2 = num+1;
    }

    num1..=num2
}