use std::fs;

#[derive(Debug)]
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
}

enum FoldType {
    Hori,   // Fold up
    Verti,  // Fold left
}

impl FoldType {
    fn from(dir: &str) -> FoldType {
        if dir == "x" {
            return FoldType::Hori;
        } else if dir == "y" {
            return FoldType::Verti;
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

fn parse_input(test: bool) -> (Vec<Point>, Vec<Fold>) {
    let path = if test {
        "input/year2021/test13.txt"
    } else {
        "input/year2021/day13.txt"
    };

    let input = fs::read_to_string(path).unwrap();

    let mut result_points = Vec::new();

    let mut fold_instructions: String = String::new();
    let mut start_folds = false;
    for line in input.lines() {
        if line.len() == 0 {
            start_folds = true;
            continue;
        }
        if start_folds {
            fold_instructions.push_str(line);
            continue;
        }

        let coords: Vec<u32> = line.split(',').into_iter().map(|i| {println!("{i}"); i.parse::<u32>().unwrap()}).collect();
        result_points.push(Point::new(coords[0], coords[1]));
    }

    let mut result_folds = Vec::new();
    for line in fold_instructions.lines() {
        let command: Vec<&str> = line.split(' ').into_iter().collect::<Vec<&str>>().get(2)
            .expect("The fold instruction is not given correctly")
            .split('=').collect();

        result_folds.push(Fold::new(FoldType::from(command[0]), command[1].parse().unwrap()) );
    }

    (result_points, result_folds)
}

pub fn part_one() {
    let input = parse_input(true);

    println!("{:?}", input.0);
}