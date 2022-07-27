use std::fs;
use std::collections::HashMap;

fn get_input(test: bool) -> Vec<Vec<char>> {
    let input = match test {
        false => fs::read_to_string("input/year2021/day10.txt").unwrap(),
        true => fs::read_to_string("input/year2021/test10.txt").unwrap(),
    };

    let mut vec = Vec::new();
    for line in input.lines() {
        vec.push(line.chars().collect());
    }

    vec
}

enum Action {
    Push(char),
    Pop(char),
}

enum Reaction {
    Ok(char),
    Err(char),
    Empty,
}

trait Keller {
    type content;

    fn action(&mut self, action: Action) -> Option<Self::content>;
}

impl Keller for Vec<char> {
    type content = char;

    fn action(&mut self, action: Action) -> Option<Self::content> {
        match action {
            Action::Pop(c) => {
                if let Some(last) = self.last() {
                    if *last != c {
                        return None;
                    }
                }
                self.pop()
            }
            Action::Push(c) => {
                self.push(c);
                return Some(c)
            }
        }
    }
}

pub fn part_one() {
    let dict = HashMap::new();

    let input = get_input(true);
    let mut result_save: HashMap<char, i32> = HashMap::new();

    for chunk in input {
        let mut keller = Vec::new();
        for c in chunk {
            
        }
    }
}