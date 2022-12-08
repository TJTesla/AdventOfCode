use std::fs::read_to_string;

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
        let mut max = 0;
        for i in 0..line.len() {
            if line[i].size > max {
                max = line[i].size;
                line[i].set_visible(true);
            }
        }

        // Right to left iteration
        let mut max = 0;
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
        let mut max = 0;
        for j in 0..trees.len() {
            if trees[j][i].size > max {
                max = trees[j][i].size;
                trees[j][i].set_visible(true);
            }
        }

        // Check bottom to top
        let mut max = 0;
        for j in (0..trees.len()).rev() {
            if trees[j][i].size > max {
                max = trees[j][i].size;
                trees[j][i].set_visible(true);
            }
        }
    }

    count_visible_trees(trees)
}

fn count_visible_trees(trees: Vec<Vec<Tree>>) -> i32 {
    trees.iter().flatten().filter(|elem| elem.visible).count() as i32
}

pub fn part_one(test: bool) {
    let input = parse_input(test);

    let result = check_visibility(input);
    println!("The result is {}", result);
}