#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up, Right, Down, Left
}

impl Direction {
    fn rotate(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        };
    }
}

enum Cell {
    Empty,
    Obstacle,
}

impl Cell {
    fn new_from_char(c: char) -> Cell {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Obstacle,
            '^' => Cell::Empty,
            _ => panic!("Creating Cell from unsupported char")
        }
    }
}

fn parse_input(test: bool) -> (Vec<Vec<char>>, (usize, usize)) {
    let path = if test {
        "input/year2024/test6.txt"
    } else {
        "input/year2024/day6.txt"
    };
    let input = fs::read_to_string(path).unwrap();

    let mut field: Vec<Vec<char>> = input.lines().into_iter()
        .map(|line|
            line.chars().collect()
        ).collect();

    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == '^' {
                field[row][col] = '.';
                return (field, (row, col))
            }
        }
    }

    (Vec::new(), (0, 0))

    /*
    // Lines
    let mut lines = Vec::new();
    for row in 0..field.len() {
        let mut obstacles = Vec::new();

        for column in 0..field[row].len() {
            if field[row][column] == '#' {
                obstacles.push(column as i32);
            }
        }

        lines.push(obstacles);
    }

    // Columns
    let mut columns = Vec::new();
    for column in 0..field[0].len() {
        let mut obstacles = Vec::new();

        for row in 0..field.len() {
            if field[row][column] == '#' {
                obstacles.push(row as i32);
            }
        }

        columns.push(obstacles);
    }


    (lines, columns)

     */
}

fn get_next_guard_spot(guard_pos: (i32, i32), dir: &Direction, field_len: usize) -> Option<(i32, i32)> {
    let new_spot = match dir {
        Direction::Up => (guard_pos.0-1, guard_pos.1),
        Direction::Down => (guard_pos.0+1, guard_pos.1),
        Direction::Right => (guard_pos.0, guard_pos.1+1),
        Direction::Left => (guard_pos.0, guard_pos.1-1)
    };

    if new_spot.0 < 0 || new_spot.1 < 0 || new_spot.0 as usize >= field_len || new_spot.1 as usize >= field_len {
        return None;
    };

    Some(new_spot)
}

pub fn part_one(_test: bool) {
    let (field, (guard_row, guard_col)) = parse_input(false);
    let (mut guard_row, mut guard_col) = (guard_row as i32, guard_col as i32);

    let mut visited: HashSet<(i32, i32)> = vec![(guard_row, guard_col)].into_iter().collect();
    let mut guard_dir = Direction::Up;
    while guard_row >= 0 && (guard_row as usize) < field.len() && guard_col >= 0 && (guard_col as usize) < field[guard_col as usize].len() {
        let next_step = get_next_guard_spot((guard_row, guard_col), &guard_dir, field.len());

        match next_step {
            None => break,
            Some((x, y)) => {
                if field[x as usize][y as usize] == '#' {
                    guard_dir.rotate();
                    continue;
                }

                guard_row = x;
                guard_col = y;
                visited.insert((x, y));
            }
        };
    }

    println!("Result: {}", visited.len());
}





fn new_field_is_cyclic(field: &Vec<Vec<char>>, new_obstacle: (i32, i32), guard: (i32, i32)) -> bool {
    let (mut guard_row, mut guard_col) = (guard.0, guard.1);

    let mut visited: HashSet<((i32, i32), Direction)> = vec![((guard_row, guard_col), Direction::Up)].into_iter().collect();
    let mut guard_dir = Direction::Up;
    while guard_row >= 0 && (guard_row as usize) < field.len() && guard_col >= 0 && (guard_col as usize) < field[guard_col as usize].len() {
        let next_step = get_next_guard_spot((guard_row, guard_col), &guard_dir, field.len());

        match next_step {
            None => break,
            Some((x, y)) => {
                if field[x as usize][y as usize] == '#' || (x, y) == new_obstacle {
                    guard_dir.rotate();
                    continue;
                }

                guard_row = x;
                guard_col = y;
                if !visited.insert(((x, y), guard_dir)) {
                    return true
                }
            }
        };
    }

    false
}

fn print_filled_field(field: &Vec<Vec<char>>, blocks: &HashSet<(i32, i32)>) {
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if blocks.contains(&(row as i32, col as i32)) {
                print!("O");
            } else {
                print!("{}", field[row][col]);
            }
        }

        print!("\t\t");

        for col in 0..field[row].len() {
            print!("{}", field[row][col]);
        }

        print!("\n");
    }
}

// > 1935
pub fn part_two(_test: bool) {
    let (field, (guard_row, guard_col)) = parse_input(false);
    let guard_init = (guard_row, guard_col);
    let (mut guard_row, mut guard_col) = (guard_row as i32, guard_col as i32);

    // Get default path
    let mut path: Vec<((i32, i32), Direction)> = vec![((guard_row, guard_col), Direction::Up)];
    let mut guard_dir = Direction::Up;
    while guard_row >= 0 && (guard_row as usize) < field.len() && guard_col >= 0 && (guard_col as usize) < field[guard_col as usize].len() {
        let next_step = get_next_guard_spot((guard_row, guard_col), &guard_dir, field.len());

        match next_step {
            None => break,
            Some((x, y)) => {
                if field[x as usize][y as usize] == '#' {
                    guard_dir.rotate();
                    path.push(((guard_row, guard_col), guard_dir.clone()));
                    continue;
                }

                guard_row = x;
                guard_col = y;
                path.push(((x, y), guard_dir.clone()));
            }
        };
    }

    let blocks = path.into_iter().map(|t| t.0).collect::<HashSet<(i32, i32)>>();

    let mut res = 0;
    // print_filled_field(&field, &blocks);

    for block in blocks {
        if new_field_is_cyclic(&field, block, (guard_init.0 as i32, guard_init.1 as i32)) {
            res += 1;

            // println!();
            // print_filled_field(&field, &vec![block].into_iter().collect());
        }
    }


    println!("Result: {}", res);
}