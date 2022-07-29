#![allow(dead_code)]
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
    Corrupt(char),
    Empty,
}

trait Keller {
    fn en_or_decode(c: char) -> char;

    fn action(&mut self, action: Action) -> Reaction;
}

impl Keller for Vec<char> {
    fn en_or_decode(c: char) -> char {
        match c {
            '{'..='}' => '{',
            '['..=']' => '[',
            '<'..='>' => '<',
            '('..=')' => '(',
            _ => '_',
        }
    }

    fn action(&mut self, action: Action) -> Reaction {
        match action {
            Action::Pop(c) => {
                if let Some(last) = self.pop() {
                    if last != Self::en_or_decode(c) {
                        return Reaction::Corrupt(c);
                    }
                    return Reaction::Ok(c);
                }
                Reaction::Empty
            }

            Action::Push(c) => {
                self.push(c);
                return Reaction::Ok(c)
            }
        }
    }
}

enum Brace {
    Open,
    Close,
}

pub fn part_two() {
    let dict = create_dict();
    let mut scores = Vec::new();

    let input = get_input(false);

    for chunk in input {
        let mut keller = Vec::new();
        let mut early_break = false;

        for c in chunk {
            let action_result = match dict.get(&c).unwrap() {
                Brace::Open => keller.action(Action::Push(c)),
                Brace::Close => keller.action(Action::Pop(c)),
            };

            match action_result {
                Reaction::Ok(_) => continue,
                Reaction::Corrupt(_) => {
                    early_break = true;
                    break;
                },
                Reaction::Empty => {
                    continue;
                }
            }
        }

        if !keller.is_empty() && !early_break {
            scores.push( calculate_score( completion_string(keller) ) );
        }
    }

    scores.sort();
    let middle = scores[scores.len()/2];

    println!("The middle score is {}", middle);
}

fn completion_string(mut keller: Vec<char>) -> String {
    let mut completed = String::new();
    let dict = get_closing_map();

    keller.reverse();
    for c in keller {
        completed.push(*dict.get(&c).unwrap());
    }

    completed
}

fn calculate_score(score_string: String) -> u64 {
    let score_table = get_score_table_part2();
    let mut val: u64 = 0;

    for c in score_string.chars() {
        val *= 5;
        val += *score_table.get(&c).unwrap() as u64;
    }

    val
}

fn get_closing_map() -> HashMap<char, char> {
    let mut dict = HashMap::new();

    dict.insert('(', ')');
    dict.insert('[', ']');
    dict.insert('{', '}');
    dict.insert('<', '>');

    dict
}

pub fn part_one() {
    let dict = create_dict();
    let score_table = get_score_table_part1();
    let mut score = 0;

    let input = get_input(false);

    for chunk in input {
        let mut keller = Vec::new();
        for c in chunk {
            let action_result = match dict.get(&c).unwrap() {
                Brace::Open => keller.action(Action::Push(c)),
                Brace::Close => keller.action(Action::Pop(c)),
            };

            match action_result {
                Reaction::Ok(_) => continue,
                Reaction::Corrupt(c) => {
                    score += score_table.get(&c).unwrap();
                    break;
                },
                Reaction::Empty => {
                    // Incomplete
                    break;
                }
            }
        }
    }

    println!("The total syntax error score is {}", score);
}

fn get_score_table_part1() -> HashMap<char, i32> {
    let mut dict = HashMap::new();

    dict.insert(')', 3);
    dict.insert(']', 57);
    dict.insert('}', 1197);
    dict.insert('>', 25137);

    dict
}

fn get_score_table_part2() -> HashMap<char, i32> {
    let mut dict = HashMap::new();

    dict.insert(')', 1);
    dict.insert(']', 2);
    dict.insert('}', 3);
    dict.insert('>', 4);

    dict
}

fn create_dict() -> HashMap<char, Brace> {
    let mut dict = HashMap::new();

    let open = "{[<(";
    for c in open.chars() {
        dict.insert(c, Brace::Open);
    }
    
    let close = "}]>)";
    for c in close.chars() {
        dict.insert(c, Brace::Close);
    }

    dict
}