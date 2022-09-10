use std::fs;
use std::collections::HashSet;

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

    fn set_important_coordinate(&mut self, dir: FoldType, new_value: u32) {
        if dir == FoldType::Hori {
            self.y = new_value;
        } else {
            self.x = new_value;
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
            FoldType::Hori
        } else if dir == "y" {
            FoldType::Verti
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

    let mut result_folds = Vec::new();
    for line in fold_instructions.lines() {
        let command: Vec<&str> = line.split(' ').into_iter().collect::<Vec<&str>>().get(2)
            .expect("The fold instruction is not given correctly")
            .split('=').collect();

        result_folds.push(Fold::new(FoldType::from(command[0]), command[1].parse().unwrap()) );
    }

    (result_points, result_folds)
}

pub fn find_size_of_paper() {
    let input = parse_input(false);

    let mut width = (u32::MAX, u32::MIN);
    let mut height = (u32::MAX, u32::MIN);

    for point in input.0 {
        if point.x > width.1 {
            width.1 = point.x;
        } else if point.x < width.0 {
            width.0 = point.x;
        }

        if point.y > height.1 {
            height.1 = point.y;
        } else if point.y < height.0 {
            height.0 = point.y;
        }
    }

    println!("---{}---", height.0);
    println!("{}-------{}", width.0, width.1);
    println!("---{}---", height.1);

}

pub fn part_one() {
    let mut input = parse_input(false);

    let first = input.1.remove(0);

    let folded = fold(first, input.0);

    println!("There are {} dots", folded.len());
}

fn calculate_new_coordinate(fold: u32, coordinate: u32) -> u32 {
    (fold * 2) - coordinate
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

        new_paper.insert(point.transform(fold.direction, coord));
    }

    new_paper
}