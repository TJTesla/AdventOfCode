#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(test: bool) -> (HashMap<i32, HashSet<i32>>, HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let path = if test {
        "input/year2024/test5.txt"
    } else {
        "input/year2024/day5.txt"
    };
    let input = fs::read_to_string(path).unwrap();

    let mut smaller = HashMap::new();
    let mut bigger = HashMap::new();
    let first_part = input.split("\n\n").next().unwrap().to_string();
    for line in first_part.lines() {
        let first: i32 = line.split('|').next().unwrap().parse().unwrap();
        let second: i32 = line.split('|').nth(1).unwrap().parse().unwrap();

        smaller.entry(second).or_insert(HashSet::new()).insert(first);
        bigger.entry(first).or_insert(HashSet::new()).insert(second);
    }

    let mut updates = Vec::new();
    let second_part = input.split("\n\n").nth(1).unwrap().to_string();
    for line in second_part.lines() {
        updates.push(line.split(',').map(|s| s.parse().unwrap()).collect());
    }

    (smaller, bigger, updates)
}

fn fix_update(update: &mut Vec<i32>, error_index: usize, bigger: &HashMap<i32, HashSet<i32>>) {
    let elem = update[error_index];
    let bigger_pages = match bigger.get(&elem) {
        Some(set) => set.clone(),
        None => HashSet::new(),
    };

    for i in 0..error_index {
        if !bigger_pages.contains(&update[i]) {
            continue;
        }


        update.remove(error_index);
        update.insert(i, elem);
        break;
    }
}

pub fn part_one(_test: bool) {
    let (rules, _, updates) = parse_input(false);

    let mut res: i32 = 0;
    for update in updates {
        let mut forbidden_pages: HashSet<i32> = HashSet::new();
        let mut correct: bool = true;

        for page in &update {
            if forbidden_pages.contains(page) {
                correct = false;
                break;
            }

            if let Some(new_forbidden) = rules.get(page) {
                forbidden_pages.extend(new_forbidden);
            }
        }

        if !correct { continue; }
        res += update[update.len()/2];
    }

    println!("Result: {}", res);
}

pub fn part_two(_test: bool) {
    let (
        smaller,
        bigger,
        updates
    ) = parse_input(false);

    let mut res: i32 = 0;
    for mut update in updates {
        let mut forbidden_pages: HashSet<i32> = HashSet::new();
        let mut correct: bool = true;

        let mut page_index = 0;
        while page_index < update.len() {
            let page = update[page_index];
            if forbidden_pages.contains(&page) {
                correct = false;
                if let Some(new_forbidden) = smaller.get(&page) {
                    forbidden_pages.extend(new_forbidden);
                }

                fix_update(&mut update, page_index, &bigger);
                continue;
            }

            if let Some(new_forbidden) = smaller.get(&page) {
                forbidden_pages.extend(new_forbidden);
            }
            page_index += 1;
        }

        if correct { continue; }
        res += update[update.len()/2];
    }

    println!("Result: {}", res);
}