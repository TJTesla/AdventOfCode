use std::fs;

#[derive(Debug)]
struct Digit {
    segments: Vec<char>,
}

impl Digit {
    fn new(segments: &str) -> Digit {
        Digit { segments: segments.chars().collect() }
    }
}

#[derive(Debug)]
struct Line {
    patterns: Vec<Digit>,
    output: Vec<Digit>,
}

impl Line {
    fn new(line: &str) -> Line {
        let parts: Vec<&str> = line.split("|").collect();

        let mut patterns = Vec::new();
        patterns.reserve(10);
        let mut output = Vec::new();
        patterns.reserve(4);

        for word in parts[0].split_ascii_whitespace() {
            patterns.push(Digit::new(word));
        }

        for word in parts[1].split_ascii_whitespace() {
            output.push(Digit::new(word));
        }

        Line {
            patterns,
            output
        }
    }
}

fn get_input() -> Vec<Line> {
    let input_string = fs::read_to_string("input/year2021/day8.txt").expect("The path to the input file is incorrect");

    let mut input: Vec<Line> = Vec::new();
    for line in input_string.lines() {
        input.push(Line::new(line));
    }

    input
}

pub fn part_one() {
    let input = get_input();

    let mut result = 0;
    for line in input {
        for digit in line.output {
            if digit.segments.len() == 2 ||  // 1
              digit.segments.len() == 4 ||  // 4
              digit.segments.len() == 3 ||  // 7
              digit.segments.len() == 7 {  // 9
                result += 1;
            }
        }
    }

    println!("The digits 1,  4, 7 and 8 appear {} time", result);
}