#![allow(dead_code)]

use std::fs;
use std::collections::HashSet;

struct Length {
    min: u32,
    max: u32,
}

struct Dimensions {
    width: Length,
    height: Length,
}

impl Dimensions {
    fn new(width: (u32, u32), height: (u32, u32)) -> Dimensions {
        Dimensions{
            width: Length { min: width.0, max: width.1, },
            height: Length { min: height.0, max: height.1, },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point {
            x,
            y,
        }
    }

    fn from_pair(pair: (u32, u32)) -> Point {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }

    fn get_important_coordinate(&self, dir: FoldType) -> u32 {
        if dir == FoldType::Hori {
            self.y
        } else {
            self.x
        }
    }

    fn transform(self, dir: FoldType, new_value: u32) -> Point {
        let pair = if dir == FoldType::Hori {
            (self.x, new_value)
        } else {
            (new_value, self.y)
        };

        Point::from_pair(pair)
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum FoldType {
    Hori,   // Fold up
    Verti,  // Fold left
}

impl FoldType {
    fn from(dir: &str) -> FoldType {
        if dir == "x" {
            FoldType::Verti
        } else if dir == "y" {
            FoldType::Hori
        } else {
            panic!("The given fold instruction is not readable");
        }
    }
}

struct Fold {
    direction: FoldType,
    row: u32,
}

impl Fold {
    fn new(direction: FoldType, row: u32) -> Fold {
        Fold {
            direction,
            row,
        }
    }
}

fn parse_input(test: bool) -> (HashSet<Point>, Vec<Fold>) {
    let path = if test {
        "input/year2021/test13.txt"
    } else {
        "input/year2021/day13.txt"
    };

    let input = fs::read_to_string(path).unwrap();

    let mut result_points = HashSet::new();

    // Points
    let mut fold_instructions: String = String::new();
    let mut start_folds = false;
    for line in input.lines() {
        if line.is_empty() {
            start_folds = true;
            continue;
        }
        if start_folds {
            fold_instructions.push_str(line);
            fold_instructions.push('\n');
            continue;
        }

        let coords: Vec<u32> = line.split(',').into_iter().map(|i| i.parse::<u32>().unwrap()).collect();
        result_points.insert(Point::new(coords[0], coords[1]));
    }

    // Folds
    let mut result_folds = Vec::new();
    for line in fold_instructions.lines() {
        let command: Vec<&str> = line.split(' ').into_iter().collect::<Vec<&str>>().get(2)
            .expect("The fold instruction is not given correctly")
            .split('=').collect();

        result_folds.push(Fold::new(FoldType::from(command[0]), command[1].parse().unwrap()) );
    }

    (result_points, result_folds)
}

pub fn part_two() {
    let input = parse_input(false);

    let mut folded = input.0;
    for instruction in input.1 {
        folded = fold(instruction, folded);
    }

    print(folded);
}

fn print(points: HashSet<Point>) {
    let dim = get_dimensions(&points);
    let mut output: String = String::new();

    for y in dim.height.min..=dim.height.max {
        for x in dim.width.min..=dim.width.max {
            if let Some(_) = points.get(&Point::new(x, y)) {
                output.push('█');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    println!("{}", output);
}

fn get_dimensions(points: &HashSet<Point>) -> Dimensions {
    let mut width = (u32::MAX, u32::MIN);
    let mut height = (u32::MAX, u32::MIN);

    for p in points.iter() {
        if p.x > width.1 {
            width.1 = p.x;
        } else if p.x < width.0 {
            width.0 = p.x;
        }

        if p.y > height.1 {
            height.1 = p.y;
        } else if p.y < height.0 {
            height.0 = p.y;
        }
    }

    Dimensions::new(width, height)
}

pub fn part_one() {
    let mut input = parse_input(false);

    let first = input.1.remove(0);

    let folded = fold(first, input.0);

    println!("There are {} dots", folded.len());
}

fn calculate_new_coordinate(fold: u32, coordinate: u32) -> i32 {
    ((fold * 2) - coordinate) as i32
}

fn fold(fold: Fold, paper: HashSet<Point>) -> HashSet<Point> {
    let mut new_paper = HashSet::new();
    for point in paper {
        let coord = point.get_important_coordinate(fold.direction);
        if coord < fold.row {
            new_paper.insert(point);
            continue;
        }

        let coord = calculate_new_coordinate(fold.row, coord);
        if coord < 0 {
            continue;
        }

        new_paper.insert(point.transform(fold.direction, coord as u32));
    }

    new_paper
}