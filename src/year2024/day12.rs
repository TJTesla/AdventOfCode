#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(test: bool) -> Vec<Vec<char>> {
    let path = if test {
        "input/year2024/test12.2.txt"
    } else {
        "input/year2024/day12.txt"
    };
    let input = fs::read_to_string(path).unwrap();

    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_all_possible_neighbours(input: &Vec<Vec<char>>, pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let signed_pos = (pos.0 as i32, pos.1 as i32);

    let shifts = vec![
        (-1, 0),
        (0, -1), (0, 1),
        (1, 0),
    ];

    let mut res = HashSet::new();

    let current_plant = input[pos.0][pos.1];
    for shift in shifts {
        let new_pos = (signed_pos.0+shift.0, signed_pos.1+shift.1);
        if new_pos.0 < 0 || new_pos.1 < 0 ||
            new_pos.0 >= input.len() as i32 || new_pos.1 >= input[new_pos.0 as usize].len() as i32 {
            continue;
        }

        // Is valid coord
        if input[new_pos.0 as usize][new_pos.1 as usize] == current_plant {
            res.insert((new_pos.0 as usize, new_pos.1 as usize));
        }
    }

    res
}


fn find_region_and_peri(land: &Vec<Vec<char>>, location: (usize, usize)) -> (HashSet<(usize, usize)>, u64) {
    let mut stack = vec![location];
    let mut visited = vec![location].into_iter().collect::<HashSet<(usize, usize)>>();
    let mut perimeter: u64 = 0;

    while !stack.is_empty() {
        let cur_vertex = stack[stack.len()-1];
        stack.remove(stack.len()-1);


        let neighbours = get_all_possible_neighbours(land, cur_vertex);
        perimeter += 4 - neighbours.len() as u64;
        for neighbour in neighbours {
            if !visited.contains(&neighbour) {
                stack.push(neighbour);
                visited.insert(neighbour);
            }
        }
    }

    (visited, perimeter)
}

pub fn part_one(_test: bool) {
    let land = parse_input(false);

    let mut counted = HashSet::new();
    let mut res: u64 = 0;
    for (row, line) in land.iter().enumerate() {
        for (col, _char) in line.iter().enumerate() {
            // If (row, col) is already counted continue
            if counted.contains(&(row, col)) {
                continue;
            }

            // Run dfs to find region (store plots in set)
            let (region, perimeter) = find_region_and_peri(&land, (row, col));

            res += region.len() as u64 * perimeter;

            counted.extend(region);
        }
    }

    println!("Result: {}", res);
}

struct Sides {
    up: bool,
    left: bool,
    right: bool,
    down: bool
}

impl Sides {
    fn new() -> Sides {
        Sides {
            up: false, left: false, right: false, down: false
        }
    }

    fn set(&mut self, index: usize) {
        match index {
            0 => self.up = true,
            1 => self.left = true,
            2 => self.right = true,
            3 => self.down = true,
            _ => panic!("Cannot set side {}", index)
        };
    }

    fn to_tuple(&self, pos: (usize, usize)) -> HashSet<((usize, usize), i32)> {
        let mut res = HashSet::new();
        if self.up { res.insert((pos, 0)); }
        if self.left { res.insert((pos, 1)); }
        if self.right { res.insert((pos, 2)); }
        if self.down { res.insert((pos, 3)); }
        res
    }
}

fn get_possible_sides(land: &Vec<Vec<char>>, pos: (usize, usize)) -> Sides {
    let signed_pos = (pos.0 as i32, pos.1 as i32);

    let shifts = vec![
        (-1, 0),
        (0, -1), (0, 1),
        (1, 0),
    ];

    let mut sides = Sides::new();

    let current_plant = land[pos.0][pos.1];
    for (i, shift) in shifts.iter().enumerate() {
        let new_pos = (signed_pos.0+shift.0, signed_pos.1+shift.1);
        if new_pos.0 < 0 || new_pos.1 < 0 ||
            new_pos.0 >= land.len() as i32 || new_pos.1 >= land[new_pos.0 as usize].len() as i32 {
            sides.set(i);
            continue;
        }

        // Is valid coord
        if land[new_pos.0 as usize][new_pos.1 as usize] != current_plant {
            sides.set(i);
        }
    }

    sides
}

fn get_next_iter(possible_sides: &HashMap<(usize, usize), Sides>, cur: ((usize, usize), i32)) -> ((usize, usize), i32) {
    match cur.1 {
        0 => {
            // Up
            // If has no left side -> direct left neighbour -> if neighbour has up side -> go to that
            //  else left neighbour must have up neighbour
            // Otherwise same plot but left side
            return if !possible_sides.get(&cur.0).unwrap().left {
                let neighbour: (usize, usize) = (cur.0.0, cur.0.1 - 1);
                if possible_sides.get(&neighbour).unwrap().up {
                    (neighbour, 0)
                } else {
                    ((cur.0.0 - 1, cur.0.1 - 1), 2)
                }
            } else {
                (cur.0, 1)
            }
        },
        1 => {
            // Left
            // If has no down side -> direct down neighbour -> if neighbour has left side -> go to that
            //  else down neighbour must have left neighbour
            // Otherwise same plot but down side
            return if !possible_sides.get(&cur.0).unwrap().down {
                let neighbour: (usize, usize) = (cur.0.0 + 1, cur.0.1);
                if possible_sides.get(&neighbour).unwrap().left {
                    (neighbour, 1)
                } else {
                    ((cur.0.0 + 1, cur.0.1 - 1), 0)
                }
            } else {
                (cur.0, 3)
            }
        },
        2 => {
            // Right
            // If has no up side -> direct up neighbour -> if neighbour has right side -> go to that
            //  else up neighbour must have right neighbour
            // Otherwise same plot but up side
            return if !possible_sides.get(&cur.0).unwrap().up {
                let neighbour: (usize, usize) = (cur.0.0 - 1, cur.0.1);
                if possible_sides.get(&neighbour).unwrap().right {
                    (neighbour, 2)
                } else {
                    ((cur.0.0 - 1, cur.0.1 + 1), 3)
                }
            } else {
                (cur.0, 0)
            }
        },
        3 => {
            // Down
            // If has no right side -> direct right neighbour -> if neighbour has down side -> go to that
            //  else right neighbour must have down neighbour
            // Otherwise same plot but right side
            return if !possible_sides.get(&cur.0).unwrap().right {
                let neighbour: (usize, usize) = (cur.0.0, cur.0.1 + 1);
                if possible_sides.get(&neighbour).unwrap().down {
                    (neighbour, 3)
                } else {
                    ((cur.0.0 + 1, cur.0.1 + 1), 1)
                }
            } else {
                (cur.0, 2)
            }
        }
        _ => panic!("Unknown side {}", cur.1)
    }
}

fn calculate_sides(land: &Vec<Vec<char>>, region: &HashSet<(usize, usize)>) -> u64 {
    let possible_sides: HashMap<(usize, usize), Sides> = region.iter().map(|loc| (*loc, get_possible_sides(land, *loc))).collect();
    let mut all_sides: HashSet<((usize, usize), i32)> = region.iter().map(|loc| get_possible_sides(land, *loc).to_tuple(*loc)).flatten().collect();

    let mut fences = 0;

    while !all_sides.is_empty() {
        let start = *all_sides.iter().next().unwrap();
        let mut cur = start;
        loop {
            all_sides.remove(&cur);
            let new = get_next_iter(&possible_sides, cur);
            if cur.1 != new.1 {
                // Side is over
                fences += 1;
            }
            cur = new;

            if start == cur {
                break;
            }
        }
    }


    fences
}

// > 768658
pub fn part_two(_test: bool) {
    let land = parse_input(true);

    let mut counted = HashSet::new();
    let mut res: u64 = 0;
    for (row, line) in land.iter().enumerate() {
        for (col, _char) in line.iter().enumerate() {
            // If (row, col) is already counted continue
            if counted.contains(&(row, col)) {
                continue;
            }

            // Run dfs to find region (store plots in set)
            let (region, _) = find_region_and_peri(&land, (row, col));

            let sides = calculate_sides(&land, &region);
            res += region.len() as u64 * sides;

            counted.extend(region);
        }
    }

    println!("Result: {}", res);
}