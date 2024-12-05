#![allow(dead_code)]

use std::fs;

fn parse_input(test: bool) -> Vec<Vec<char>> {
    let path = if test {
        "input/year2024/test4.txt".to_string()
    } else {
        "input/year2024/day4.txt".to_string()
    };

    let input = fs::read_to_string(path).unwrap();

    input.lines().into_iter().map(|line| line.chars().collect()).collect()
}

fn check_for_xmas(input: &Vec<Vec<char>>, indices: &Vec<(i32, i32)>) -> bool {
    if indices.len() != 4 {
        return false;
    }
    for (row, column) in indices {
        if *row < 0 || *row >= input.len() as i32 || *column < 0 || *column >= input[*row as usize].len() as i32 {
            return false;
        }
    }

    // Now we know that all indices are in-bounds
    let indices: Vec<(usize, usize)> = indices.into_iter().map(|(a, b)| (*a as usize, *b as usize)).collect();
    if input[indices[0].0][indices[0].1] != 'X' {
        return false;
    }
    if input[indices[1].0][indices[1].1] != 'M' {
        return false;
    }
    if input[indices[2].0][indices[2].1] != 'A' {
        return false;
    }
    if input[indices[3].0][indices[3].1] != 'S' {
        return false;
    }

    true
}

fn check_for_x_mas(input: &Vec<Vec<char>>, indices: &Vec<(i32, i32)>) -> bool {
    if indices.len() != 5 {
        return false;
    }
    for (row, column) in indices {
        if *row < 0 || *row >= input.len() as i32 || *column < 0 || *column >= input[*row as usize].len() as i32 {
            return false;
        }
    }

    // Now we know that all indices are in-bounds
    let indices: Vec<(usize, usize)> = indices.into_iter().map(|(a, b)| (*a as usize, *b as usize)).collect();

    // Check down-right
    let word = vec![
        input[indices[1].0][indices[1].1],
        input[indices[0].0][indices[0].1],
        input[indices[2].0][indices[2].1]
    ].into_iter().collect::<String>();
    if word != "MAS" && word != "SAM" {
        return false;
    }

    // Check down-left
    let word = vec![
        input[indices[3].0][indices[3].1],
        input[indices[0].0][indices[0].1],
        input[indices[4].0][indices[4].1]
    ].into_iter().collect::<String>();
    if word != "MAS" && word != "SAM" {
        return false;
    }

    true
}


pub fn part_one(_test: bool) {
    let input = parse_input(false);

    let shift_indices = vec![
        // Horizontals
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, -1), (0, -2), (0, -3)],
        // Verticals
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
        // Diagonals (down-right)
        vec![(0, 0), (1, 1), (2, 2), (3, 3)],
        vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
        // Diagonals (down-left)
        vec![(0, 0), (1, -1), (2, -2), (3, -3)],
        vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
    ];

    let mut res = 0;
    for row in 0..input.len() {
        for column in 0..input[row].len() {
            for shift in &shift_indices {
                if check_for_xmas(&input, &shift.iter().map(|(r, c)| (row as i32 + r, column as i32 + c)).collect()) {
                    res += 1;
                }
            }
        }
    }

    println!("Result: {}", res);
}

pub fn part_two(_test: bool) {
    let input = parse_input(false);

    let shift_indices = vec![
        (0, 0), (-1, -1), (1, 1), (-1, 1), (1, -1),
    ];

    let mut res = 0;
    for row in 0..input.len() {
        for column in 0..input[row].len() {
            if check_for_x_mas(&input, &shift_indices.iter().map(|(r, c)| (row as i32 + r, column as i32 + c)).collect()) {
                res += 1;
            }
        }
    }

    println!("Result: {}", res);
}