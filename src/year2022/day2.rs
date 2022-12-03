#![allow(dead_code)]
use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3
}

impl Hand {
    fn fight(&self, other: Hand) -> Outcome {
        let own_stats = Hand::get_relation(*self);
        if other == own_stats.0 {
            return Outcome::Win;
        }
        if other == own_stats.1 {
            return Outcome::Lose;
        }
        if other == own_stats.2 {
            return Outcome::Draw;
        }

        return Outcome::Draw;
    }

    fn get_hand_for_outcome(oponent: Hand, outcome: Outcome) -> Hand {
        let stats = Hand::get_relation(oponent);
        match outcome {
            Outcome::Win => stats.1,
            Outcome::Lose => stats.0,
            Outcome::Draw => stats.2
        }
    }

    fn get_relation(hand: Hand) -> (Hand, Hand, Hand) {
        match hand {
            Hand::Rock => (Hand::Scissors, Hand::Paper, Hand::Rock),
            Hand::Paper => (Hand::Rock, Hand::Scissors, Hand::Paper),
            Hand::Scissors => (Hand::Paper, Hand::Rock, Hand::Scissors)
        }
    }
}

fn create_code_to_hand() -> HashMap<char, Hand> {
    // A -> Rock
    // B -> Paper
    // C -> Scissors
    let mut code_to_hand: HashMap<char, Hand> = HashMap::new();
    code_to_hand.insert('A', Hand::Rock);
    code_to_hand.insert('B', Hand::Paper);
    code_to_hand.insert('C', Hand::Scissors);

    code_to_hand
}

fn create_code_to_outcome() -> HashMap<char, Outcome> {
    // X -> Lose
    // Y -> Draw
    // Z -> Win
    let mut code_to_outcome: HashMap<char, Outcome> = HashMap::new();
    code_to_outcome.insert('X', Outcome::Lose);
    code_to_outcome.insert('Y', Outcome::Draw);
    code_to_outcome.insert('Z', Outcome::Win);

    code_to_outcome
}

fn parse_input_part_two(test: bool) -> Vec<(Hand, Outcome)> {
    let path = if test {
        "input/year2022/test2.txt".to_string()
    } else {
        "input/year2022/day2.txt".to_string()
    };

    let code_to_hand = create_code_to_hand();
    let code_to_outcome = create_code_to_outcome();
    let input = read_to_string(path).unwrap();
    let mut result = Vec::new();

    for line in input.lines() {
        let mut line_iter = line.split_ascii_whitespace();
        let first_char = line_iter.next().unwrap().chars().next().unwrap();
        let second_char = line_iter.next().unwrap().chars().next().unwrap();

        result.push((
            *code_to_hand.get(&first_char).expect(&format!("The char {} wasn't found in code_to_hand", first_char)),
            *code_to_outcome.get(&second_char).expect(&format!("The char {} wasn't found in code_to_outcome", second_char))
        ));
    }

    result
}

fn round_outcome_points(oponent: Hand, own: Hand) -> i32 {
    (own as isize + own.fight(oponent) as isize) as i32
}

pub fn part_two() {
    let input = parse_input_part_two(false);

    let mut score = 0;
    for round in input {
        score += round_outcome_points(round.0, Hand::get_hand_for_outcome(round.0, round.1))
    }

    println!("The score is {}", score);
}

pub fn part_one() {
    // The method was changed for the second part 😬
    // let input = parse_input(false);
    let input: Vec<(Hand, Hand)> = Vec::new();
    let mut score = 0;
    for round in input {
        score += round_outcome_points(round.0, round.1);
    }
    println!("The score is {}", score);
}