#[allow(dead_code)]
use std::fs;

fn get_input(test: bool) -> Vec<Vec<Cell>> {
    let input = match test {
        false => fs::read_to_string("input/year2021/day9.txt").expect("The path to the input file was incorrect"),
        true => fs::read_to_string("input/year2021/test9.txt").expect("The path to the input file was incorrect")
    };

    let mut result = Vec::new();
    for line in input.lines() {
        let sub_vec: Vec<Cell> = line.chars().into_iter().map(
            |x| Cell::new(x.to_string().parse().unwrap())
        ).collect();
        result.push(sub_vec);
    }
    result
}

struct Cell {
    value: i32,
    status: Count,
}

impl Cell {
    fn new(value: i32) -> Cell {
        Cell { value: value, status: Count::Uncounted }
    }

    fn set_counted(&mut self) {
        self.status = Count::Counted;
    }
}

#[derive(PartialEq)]
enum Count {
    Counted,
    Uncounted,
}

pub fn part_two() {
    let mut input = get_input(false);

    let mut basins = Vec::new();

    for i in 0..input.len() {
        for k in 0..input[i].len() {
            let adjacent = get_possible_neighbours(i, k, input.len(), input[i].len());

            let mut one_adjacent_lower = false;
            for (x, y) in adjacent {
                if input[x][y].value <= input[i][k].value {
                    one_adjacent_lower = true;
                    break;
                }
            }

            if !one_adjacent_lower && input[i][k].status == Count::Uncounted && input[i][k].value != 9 {
                basins.push(deep_search(i, k, &mut input));
            }
        }
    }

    basins.sort();
    basins.reverse();
    let result = basins[0] * basins[1] * basins[2];

    println!("The product of the three biggest basins is {}", result);
}

fn get_possible_neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();

    if x != 0 {
        adjacent.push((x-1, y));
    }
    if x != width-1 {
        adjacent.push((x+1, y));
    }

    if y != 0 {
        adjacent.push((x, y-1));
    }
    if y != height-1 {
        adjacent.push((x, y+1));
    }

    adjacent
}

fn deep_search(x: usize, y: usize, input: &mut Vec<Vec<Cell>>) -> i32 {
    input[x][y].set_counted();
    let mut size = 1;

    let adjacent = get_possible_neighbours(x, y, input.len(), input[x].len());
    for (next_x, next_y) in adjacent {
        if input[next_x][next_y].status == Count::Counted || input[next_x][next_y].value == 9 {
            continue;
        }

        size += deep_search(next_x, next_y, input);
    }

    size
}

pub fn _part_one() {
    let input = get_input(false);

    let mut sum_of_risks = 0;

    for i in 0..input.len() {
        for k in 0..input[i].len() {
            let mut adjacent = Vec::new();

            if i != 0 {
                adjacent.push((i-1, k));
            }
            if i != input.len()-1 {
                adjacent.push((i+1, k));
            }

            if k != 0 {
                adjacent.push((i, k-1));
            }
            if k != input[i].len()-1 {
                adjacent.push((i, k+1));
            }

            let mut one_adjacent_lower = false;
            for (x, y) in adjacent {
                if input[x][y].value <= input[i][k].value {
                    one_adjacent_lower = true;
                    break;
                }
            }

            if !one_adjacent_lower {
                sum_of_risks += input[i][k].value + 1;
            }
        }
    }

    println!("The sum of the risk levels of all low points is {}", sum_of_risks);
}