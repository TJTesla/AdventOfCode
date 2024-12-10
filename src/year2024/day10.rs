#![allow(dead_code)]

use std::collections::{HashSet};
use std::fs;

fn parse_input(test: bool) -> Vec<Vec<i32>> {
    let path = if test {
        "input/year2024/test10.txt"
    } else {
        "input/year2024/day10.txt"
    };
    let input = fs::read_to_string(path).unwrap();

    input.lines().map(|line|
        line.chars().map(|c|
            c.to_digit(10).unwrap() as i32).collect()).collect()
}

fn get_all_possible_neighbours(input: &Vec<Vec<i32>>, pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let signed_pos = (pos.0 as i32, pos.1 as i32);

    let shifts = vec![
        (-1, 0),
        (0, -1), (0, 1),
        (1, 0),
    ];

    let mut res = HashSet::new();

    let current_height = input[pos.0][pos.1];
    for shift in shifts {
        let new_pos = (signed_pos.0+shift.0, signed_pos.1+shift.1);
        if new_pos.0 < 0 || new_pos.1 < 0 ||
                new_pos.0 >= input.len() as i32 || new_pos.1 >= input[new_pos.0 as usize].len() as i32 {
            continue;
        }
        // Is valid coord
        if input[new_pos.0 as usize][new_pos.1 as usize] == current_height+1 {
            res.insert((new_pos.0 as usize, new_pos.1 as usize));
        }
    }

    res
}

fn build_graph(input: &Vec<Vec<i32>>, trailhead: (usize, usize)) -> i32 {
    let mut stack = vec![trailhead];
    let mut visited = vec![trailhead].into_iter().collect::<HashSet<(usize, usize)>>();

    let mut reachable_tops = HashSet::new();

    while !stack.is_empty() {
        let cur_vertex = stack[stack.len()-1];
        stack.remove(stack.len()-1);

        if input[cur_vertex.0][cur_vertex.1] == 9 {
            reachable_tops.insert(cur_vertex);
        }

        let neighbours = get_all_possible_neighbours(input, cur_vertex);
        for neighbour in neighbours {
            if !visited.contains(&neighbour) {
                stack.push(neighbour);
                visited.insert(neighbour);
            }
        }
    }

    reachable_tops.len() as i32
}

pub fn part_one(_test: bool) {
    let input = parse_input(false);

    // Find all trailheads
    let mut trailheads = Vec::new();
    for (row, line) in input.iter().enumerate() {
        for (col, num) in line.iter().enumerate() {
            if *num == 0 {
                trailheads.push((row, col));
            }
        }
    }

    // For all trailheads create directed graph
    let mut res = 0;
    for trailhead in &trailheads {
        let score = build_graph(&input, *trailhead);
        res += score;
    }

    println!("Result: {}", res);
}


fn calculate_rating(input: &Vec<Vec<i32>>, trailhead: (usize, usize)) -> i32 {
    let mut stack = vec![trailhead];

    let mut rating = 0;

    while !stack.is_empty() {
        let cur_vertex = stack[stack.len()-1];
        stack.remove(stack.len()-1);

        if input[cur_vertex.0][cur_vertex.1] == 9 {
            rating += 1;
        }

        let neighbours = get_all_possible_neighbours(input, cur_vertex);
        for neighbour in neighbours {
            stack.push(neighbour);
        }
    }

    rating
}


pub fn part_two(_test: bool) {
    let input = parse_input(false);

    // Find all trailheads
    let mut trailheads = Vec::new();
    for (row, line) in input.iter().enumerate() {
        for (col, num) in line.iter().enumerate() {
            if *num == 0 {
                trailheads.push((row, col));
            }
        }
    }

    // For all trailheads create directed graph
    let mut res = 0;
    for trailhead in &trailheads {
        let score = calculate_rating(&input, *trailhead);
        res += score;
    }

    println!("Result: {}", res);
}



