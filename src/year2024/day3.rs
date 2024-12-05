#![allow(dead_code)]

use std::fs;

use std::collections::HashMap;

fn parse_input(test: bool, part_one: bool) -> Vec<char> {
    let path = if test {
        if part_one {
            "input/year2024/test3.txt".to_string()
        } else {
            "input/year2024/test3.2.txt".to_string()
        }
    } else {
        "input/year2024/day3.txt".to_string()
    };

    let input = fs::read_to_string(path).unwrap();

    input.chars().collect()
}

pub(crate) fn get_next_state(dfa: &HashMap<(i32, char), i32>, state: i32, c: char) -> i32 {
    match dfa.get(&(state, c)) {
        Some(&next_state) => next_state,
        None => 0
    }
}

fn build_mul_dfa() -> HashMap<(i32, char), i32> {
    let mut dfa: HashMap<(i32, char), i32> = [
        ((0, 'm'), 1),
        ((1, 'u'), 2),
        ((2, 'l'), 3),
        ((3, '('), 4),
        ((5, ','), 8),
        ((6, ','), 8),
        ((7, ','), 8),
        ((9, ')'), 12),
        ((10, ')'), 12),
        ((11, ')'), 12),
    ].iter().cloned().collect();
    for i in 0..=9 {
        dfa.insert((4, i.to_string().chars().next().unwrap()), 5);
        dfa.insert((5, i.to_string().chars().next().unwrap()), 6);
        dfa.insert((6, i.to_string().chars().next().unwrap()), 7);
        dfa.insert((8, i.to_string().chars().next().unwrap()), 9);
        dfa.insert((9, i.to_string().chars().next().unwrap()), 10);
        dfa.insert((10, i.to_string().chars().next().unwrap()), 11);
    }
    dfa
}

fn build_do_dfa() -> HashMap<(i32, char), i32> {
    [
        ((0, 'd'), 1),
        ((1, 'o'), 2),
        ((2, '('), 3),
        ((3, ')'), 4),
    ].iter().cloned().collect()
}

fn build_dont_dfa() -> HashMap<(i32, char), i32> {
    [
        ((0, 'd'), 1),
        ((1, 'o'), 2),
        ((2, 'n'), 3),
        ((3, '\''), 4),
        ((4, 't'), 5),
        ((5, '('), 6),
        ((6, ')'), 7),
    ].iter().cloned().collect()
}

pub fn part_one(_test: bool) {
    let input = parse_input(false, true);

    let mul_dfa = build_mul_dfa();

    let mut cur_state = 0;
    let mut result = 0;

    let mut first_num: i32 = 0;
    let mut second_num: i32 = 0;
    for c in input {
        let new_state = get_next_state(&mul_dfa, cur_state, c);
        if new_state == 0 {
            cur_state = 0;
            first_num = 0;
            second_num = 0;
        }

        if c.is_ascii_digit() {
            let num = c.to_digit(10).unwrap() as i32;
            if cur_state == 4 || cur_state == 5 || cur_state == 6 {
                first_num = first_num *10 + num;
            } else if cur_state == 8 || cur_state == 9 || cur_state == 10 {
                second_num = second_num *10 + num;
            }
        }

        if new_state == 12 {
            result += first_num * second_num;
            cur_state = 0;
            first_num = 0;
            second_num = 0;
            continue;
        }
        cur_state = new_state;
    }

    println!("Result: {}", result);
}


pub fn part_two(_test: bool) {
    let input = parse_input(false, false);

    let mul_dfa = build_mul_dfa();
    let do_dfa = build_do_dfa();
    let dont_dfa = build_dont_dfa();

    let mut mul_state = 0;
    let mut do_state = 0;
    let mut dont_state = 0;
    let mut result = 0;

    let mut first_num: i32 = 0;
    let mut second_num: i32 = 0;

    let mut enabled: bool = true;
    for c in input {
        if !enabled {
            let new_state = get_next_state(&do_dfa, do_state, c);
            if new_state == 4 {
                enabled = true;
                do_state = 0;
            } else {
                do_state = new_state;
            }
            continue;
        }

        let new_state = get_next_state(&dont_dfa, dont_state, c);
        if new_state == 7 {
            enabled = false;
            dont_state = 0;
            mul_state = 0;
            first_num = 0;
            second_num = 0;
            continue;
        }
        dont_state = new_state;

        let new_state = get_next_state(&mul_dfa, mul_state, c);
        if new_state == 0 {
            mul_state = 0;
            first_num = 0;
            second_num = 0;
            continue;
        }

        if c.is_ascii_digit() {
            let num = c.to_digit(10).unwrap() as i32;
            if mul_state == 4 || mul_state == 5 || mul_state == 6 {
                first_num = first_num *10 + num;
            } else if mul_state == 8 || mul_state == 9 || mul_state == 10 {
                second_num = second_num *10 + num;
            }
        }

        if new_state == 12 {
            result += first_num * second_num;
            mul_state = 0;
            first_num = 0;
            second_num = 0;
            continue;
        }
        mul_state = new_state;
    }

    println!("Result: {}", result);
}
