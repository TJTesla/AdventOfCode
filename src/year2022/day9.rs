#![allow(dead_code)]
use std::{fs::read_to_string, collections::{HashSet}};

#[derive(Debug)]
enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Not accepted char \"{}\"", c)
        }
    }

    fn change_coords(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (x, y-1),
            Direction::Down => (x, y+1),
            Direction::Left => (x-1, y),
            Direction::Right => (x+1, y),
        }
    }
}

fn parse_input(test: bool) -> Vec<(Direction, i32)> {
    let path = if test {
        "input/year2022/test9.txt"
    } else {
        "input/year2022/day9.txt"
    }.to_string();
    let input = read_to_string(path).unwrap();

    let mut result = Vec::new();
    for line in input.lines() {
        let line_no_space = line.replace(" ", "");
        let mut char_iter = line_no_space.chars();
        let direction = Direction::from_char(char_iter.next().unwrap());
        let num = char_iter.next().unwrap().to_string().parse().unwrap();
        result.push((direction, num));
    }

    result
}

fn simulate_step(
    mut head: (i32, i32), mut tail: (i32, i32), 
    step: (Direction, i32), visited: &mut HashSet<(i32, i32)>
) -> ((i32, i32), (i32, i32)) {
    for _ in 0..step.1 {
        visited.insert(tail);
        let new_head = step.0.change_coords(head);
        if (tail.0 - new_head.0).abs() >= 2 || (tail.1 - new_head.1).abs() >= 2 {
            tail = head;
        }
        head = new_head;
        //println!("Head: {:?}; Tail: {:?}", head, tail);
    }
    visited.insert(tail);

    (head, tail)
}

pub fn part_one(test: bool) {
    let input = parse_input(test);
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();
    
    for line in input {
        (head, tail) = simulate_step(head, tail, line, &mut visited);
    }

    //println!("{:?}", visited);
    println!("The result is {}", visited.len());
}