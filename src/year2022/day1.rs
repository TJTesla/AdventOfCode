#![allow(dead_code)]
use std::fs;

struct Elf {
    food: Vec<i32>,
    total: i32
}

impl Elf {
    fn new() -> Elf {
        Elf {
            food: Vec::new(),
            total: 0,
        }
    }

    fn add_food(&mut self, food: i32) {
        self.food.push(food);
        self.total += food;
    }
}

fn parse_input(test: bool) -> Vec<Elf> {
    let path = if test {
        "input/year2022/test1.txt".to_string()
    } else {
        "input/year2022/day1.txt".to_string()
    };

    let input = fs::read_to_string(path).unwrap();

    let mut result = Vec::new();
    for package in input.split("\n\n") {
        let mut e = Elf::new();
        for line in package.lines() {
            let num: i32 = line.parse().unwrap();
            e.add_food(num);
        }
        result.push(e);
    }

    return result;
}

fn find_max(vec: &Vec<Elf>) -> (usize, &Elf) {
    let mut max_index = 0;

    for (index, val) in vec.iter().enumerate() {
        if val.total > vec[max_index].total {
            max_index = index;
        }
    }

    return (max_index, &vec[max_index]);
}

fn find_max_three_sum(mut vec: Vec<Elf>) -> i32 {
    let mut top_three = Vec::new();

    for _ in 0..3 {
        let (index, _) = find_max(&vec);
        top_three.push(vec.remove(index));
    }

    if top_three.len() != 3 {
        panic!("{}", format!("There should have been 3 elements in top_three. There were {} elements", top_three.len()));
    }

    let mut res = 0;
    for e in top_three {
        res += e.total;
    }

    res
}

pub fn part_two() {
    let res = parse_input(false);

    println!("The calories of th three maxed out elves is {}", find_max_three_sum(res));
}

pub fn part_one() {
    let res = parse_input(false);
    
    println!("The maximum calories are {}", find_max(&res).1.total);
}