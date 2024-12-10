#![allow(dead_code)]

use std::collections::{HashSet};
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Cell {
    pub id: i32,
    pub length: i32,
    is_block: bool
}

impl Cell {
    fn new_block(id: i32, length: i32) -> Cell {
        Cell {
            id, length, is_block: true
        }
    }

    fn new_free(length: i32) -> Cell {
        Cell {
            id: -1, length, is_block: false
        }
    }

    fn is_free(&self) -> bool {
        !self.is_block
    }
}

fn parse_input(test: bool) -> Vec<Cell> {
    let path = if test {
        "input/year2024/test9.txt"
    } else {
        "input/year2024/day9.txt"
    };
    let input = fs::read_to_string(path).unwrap();

    let mut res = Vec::new();

    let mut free = false;
    let mut id_counter = 0;
    for num in input.chars().map(|c| c.to_digit(10).unwrap() as i32) {
        if free {
            res.push(Cell::new_free(num));
        } else {
            res.push(Cell::new_block(id_counter, num));
            id_counter += 1;
        }
        free = !free;
    }

    res
}

fn parse_for_part_two(test: bool) -> Vec<(Cell, i32)> {
    let path = if test {
        "input/year2024/test9.txt"
    } else {
        "input/year2024/day9.txt"
    };
    let input = fs::read_to_string(path).unwrap();

    // let input = "14113".to_string();

    let mut res = Vec::new();

    let mut free = false;
    let mut id_counter = 0;
    for num in input.chars().map(|c| c.to_digit(10).unwrap() as i32) {
        if !free {
            res.push((Cell::new_block(id_counter as i32, num), 0));
            id_counter += 1;
        } else {
            res[id_counter-1].1 = num;
        }
        free = !free;
    }

    res
}

fn calculate_checksum(input: &Vec<Cell>) -> i64 {
    let mut res = 0;

    let mut pos = 0;
    for c in input {
        if c.id == -1 { continue; }

        for i in 0..c.length {
            res += c.id as i64 * (pos+i) as i64;
        }
        pos += c.length;
    }

    res
}

fn print(input: &Vec<Cell>) {
    println!("{:?}", input.iter().map(|c| {
        match c.id {
            -1 => ".".repeat(c.length as usize),
            a => a.to_string().repeat(c.length as usize)
        }
    }).collect::<String>());
}

fn print_2(input: &Vec<(Cell, i32)>) {
    println!("{}", input.iter().map(|(c, n)| {
        let mut str = c.id.to_string().repeat(c.length as usize);
        str.push_str(&".".repeat(*n as usize));
        str
    }).collect::<String>());
}

pub fn part_one(_test: bool) {
    let mut input = parse_input(false);

    let mut first_free = 1_i64;
    let mut last_block = if input[input.len()-1].is_free() {
        input.len()-2
    } else {
        input.len()-1
    } as i64;

    while first_free < last_block {
        let free_length = input[first_free as usize].length;
        let block_length = input[last_block as usize].length;
        let block_id = input[last_block as usize].id;

        if free_length <= block_length {
            input[first_free as usize] = Cell::new_block(block_id, free_length);
            if free_length < block_length {
                input[last_block as usize] = Cell::new_block(block_id, block_length - free_length);
            } else {
                input.remove(last_block as usize);
            }
        } else if free_length > block_length {
            input[first_free as usize] = Cell::new_block(block_id, block_length);
            input.insert(first_free as usize + 1, Cell::new_free(free_length - block_length));
            input.remove(last_block as usize + 1);
        }

        // Find next free
        while first_free < input.len() as i64 && !input[first_free as usize].is_free() {
            first_free += 1;
        }

        // Find next block
        while last_block >= 0 && input[last_block as usize].is_free() {
            last_block -= 1;
        }
    }

    println!("{:?}", input.iter().map(|c| c.id.to_string().repeat(c.length as usize)).collect::<String>());
    let res: i64 = calculate_checksum(&input);
    println!("Result: {}", res);
}

fn find_fitting_slot(input: &Vec<(Cell, i32)>, index: usize) -> usize {
    for i in 0..index {
        if input[i].1 >= input[index].0.length {
            return i;
        }
    }

    index
}

fn calculate_checksum_2(input: &Vec<(Cell, i32)>) -> i64 {
    let mut res = 0;

    let mut pos = 0;
    for c in input {
        for i in 0..c.0.length {
            res += c.0.id as i64 * (pos+i) as i64;
        }
        pos += c.0.length + c.1;
    }

    res
}

fn sanity_check(calculated: &Vec<(Cell, i32)>, test: bool) -> bool {
    let input = parse_for_part_two(test);

    // Check that amount of blocks is the same
    let calculated_amount = calculated.iter().fold(0, |acc, (c, n)| {
        acc + c.length + *n
    });
    let correct_amount = input.iter().fold(0, |acc, (c, n)| {
        acc + c.length  + *n
    });
    if calculated_amount != correct_amount {
        println!("{} vs {}: Difference: {}", calculated_amount, correct_amount, calculated_amount -correct_amount);
        return false;
    }

    true
}

// > 6307231144130
// > 6307223171462
//   6307279963620
pub fn part_two(_test: bool) {
    let test = false;
    let mut input = parse_for_part_two(test);

    let mut moved: HashSet<i32> = HashSet::new();

    let mut i = input.len() as i64 - 1;
    loop {
        if test { print_2(&input); }
        if i == 0 {
            break;
        }
        let next_block = input[i as usize];
        let new_slot = find_fitting_slot(&input, i as usize);
        if moved.contains(&next_block.0.id) || new_slot >= i as usize {
            i -= 1;
            continue;
        }
        moved.insert(next_block.0.id);

        if new_slot == i as usize -1 {
            let empty = input[(i-1) as usize].1 + next_block.1;
            input[(i-1) as usize].1 = 0;
            input[i as usize].1 = empty;

            i -= 1;
            continue;
        }


        let remaining = input[new_slot].1 - next_block.0.length;
        input[(i-1) as usize].1 += next_block.0.length + next_block.1;

        // Remove space of/after new_slot
        input[new_slot].1 = 0;
        // Insert block with remaining space
        input.remove(i as usize);
        input.insert(new_slot+1, (next_block.0, remaining));
    }

    if test { print_2(&input); }
    println!("Sanity Check: {}", sanity_check(&input, test));
    println!("Result: {}", calculate_checksum_2(&input));
}
