#![allow(dead_code)]
use std::fs::read_to_string;

#[derive(Debug)]
struct Tree {
    size: i32,
    visible: bool,
}

impl Tree {
    fn new(size: i32) -> Tree {
        Tree {
            size,
            visible: false,
        }
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

fn parse_input(test: bool) -> Vec<Vec<Tree>> {
    let path = if test {
        "input/year2022/test8.txt"
    } else {
        "input/year2022/day8.txt"
    }.to_string();
    let input = read_to_string(path).unwrap();

    let mut result = Vec::new();
    for line in input.lines() {
        let mut l: Vec<Tree> = Vec::new();
        for c in line.chars() {
            l.push(Tree::new(c.to_string().parse().expect(
                &format!("The character {} couldn't be parsed", c)
            )));
        }
        result.push(l);
    }

    result
}

fn check_visibility(mut trees: Vec<Vec<Tree>>) -> i32 {
    // Check lines (top to bottom)
    for line in &mut trees {
        // Left to right iteration
        let mut max = -1;
        for i in 0..line.len() {
            if line[i].size > max {
                max = line[i].size;
                line[i].set_visible(true);
            }
        }

        // Right to left iteration
        let mut max = -1;
        for i in (0..line.len()).rev() {
            if line[i].size > max {
                max = line[i].size;
                line[i].set_visible(true);
            }
        }
    }

    // Check columns
    for i in 0..trees[0].len() {
        // Check top to bottom
        let mut max = -1;
        for j in 0..trees.len() {
            if trees[j][i].size > max {
                max = trees[j][i].size;
                trees[j][i].set_visible(true);
            }
        }

        // Check bottom to top
        let mut max = -1;
        for j in (0..trees.len()).rev() {
            if trees[j][i].size > max {
                max = trees[j][i].size;
                trees[j][i].set_visible(true);
            }
        }
    }

    count_visible_trees(trees)
}

struct View {
    up: i32,
    down: i32,
    left: i32,
    right: i32,
}

impl View {
    fn new_default() -> View {
        View { up: 0, down: 0, left: 0, right: 0 }
    }

    fn scenic_score(&self) -> i32 {
        self.up * self.down * self.left * self.right
    }
}

fn check_view(trees: Vec<Vec<Tree>>) -> i32 {
    let mut views = Vec::new();
    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            if determine_edge_tree((i, j), (trees.len(), trees[i].len())) {
                continue;
            }
            let mut view = View::new_default();
            // Go up
            let mut counter = i as i32 -1;
            loop {
                view.up += 1;
                if trees[counter as usize][j].size >= trees[i][j].size {
                    break;
                }
                counter -= 1;
                if counter < 0 {
                    break;
                }
            }
            // Go down
            let mut counter = i +1;
            loop {
                view.down += 1;
                if trees[counter][j].size >= trees[i][j].size {
                    break;
                }
                counter += 1;
                if counter >= trees.len() {
                    break;
                }
            }
            // Go left
            let mut counter = j as i32 -1;
            loop {
                view.left += 1;
                if trees[i][counter as usize].size >= trees[i][j].size {
                    break;
                }
                counter -= 1;
                if counter < 0 {
                    break;
                }
            }
            // Go right
            let mut counter = j +1;
            loop {
                view.right += 1;
                if trees[i][counter].size >= trees[i][j].size {
                    break;
                }
                counter += 1;
                if counter >= trees[i].len() {
                    break;
                }
            }

            views.push(view.scenic_score());
        }
    }

    *views.iter().max().unwrap()
}

fn determine_edge_tree((x, y): (usize, usize), (width, height): (usize, usize)) -> bool {
    x == 0 || x == width-1 || y == 0 || y == height-1
}

fn count_visible_trees(trees: Vec<Vec<Tree>>) -> i32 {
    trees.iter().flatten().filter(|elem| elem.visible).count() as i32
}

pub fn part_two(test: bool) {
    let input = parse_input(test);

    let result = check_view(input);
    println!("The result is {}", result);
}

pub fn part_one(test: bool) {
    let input = parse_input(test);

    let result = check_visibility(input);
    println!("The result is {}", result);
}