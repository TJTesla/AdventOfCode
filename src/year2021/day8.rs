use std::{fs, collections::HashSet};

/// Store a single digit of the 7-segment display
#[derive(Debug, Default, Clone)]
struct Digit {
    segments: Vec<char>,
    number: Option<i32>,
}

impl Digit {
    fn new(segments: &str) -> Digit {
        let number = match segments.len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None
        };

        let mut segments: Vec<char> = segments.chars().collect();
        segments.sort();

        Digit { segments, number }
    }

    fn new_char(c: char) -> Digit {
        Digit { segments: vec![c], number: None }
    }

    fn fits_on(&self, other: &Digit) -> bool {
        for c in &self.segments {
            if !other.segments.contains(c) {
                return false;
            }
        }

        return true;
    }
}

impl PartialEq for Digit {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}

/// One line, thus one puzzle from the input file
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
        output.reserve(4);

        for word in parts[0].split_ascii_whitespace() {
            patterns.push(Digit::new(word));
        }

        for word in parts[1].split_ascii_whitespace() {
            output.push(Digit::new(word));
        }

        Line {
            patterns,
            output,
        }
    }
}

/// Transform the content of the input file into a Vec, where every element represents one line/puzzle
fn get_input() -> Vec<Line> {
    let input_string = fs::read_to_string("input/year2021/day8.txt").expect("The path to the input file is incorrect");

    let mut input: Vec<Line> = Vec::new();
    for line in input_string.lines() {
        input.push(Line::new(line));
    }

    input
}

/// Public API for the second part of the puzzle
pub fn part_two() {
    let mut total = 0;
    for line in get_input() {
        total += deduce_line(line);
    }
    println!("The total is {}", total);
}

/// Private method that hosts the algorithm
/// # Method for deducing the numbers
/// 1. Check whether the output consists of unique numbers
/// 2. Usage of congruency algorithm (not really congruency, but close enough)
/// 
/// ## The congruency algorithm
/// - Function that takes a template and a pool of digits
/// - Calls fits_on method on template, with iterating elements from pool as parameter
/// ### The fits_on method
/// - Returns true, if every segment of the instance called on is present in the segments of the parameter
///  
/// 3. Divide remaining digits into digits with 5 or with 6 digits (both have 3)
/// 4. Check for 6-segmenters:
///     1. 9 is only digit that returns true with congruency algorithm calles on 4
///     2. 0 returns true when called on 1
///     3. Remining digit can only be 6
/// 5. Check for 5-segmenters:
///     1. 3 is only digit that returns truw when called on 1
///     2. Get segment that is not in 6 (upper right)
///     3. Digit from remainders that includes that on is 5, the other is 2
/// 6. Go through every output digit
/// 7. Compare with every solved pattern and store correct number
fn deduce_line(mut line: Line) -> i32 {
    let mut result = line.patterns.clone();

    // Check whether the output is already solved (unique numbers only)
    if solved(&line.output) {
        return calculate_line_total(&line.output);
    }

    // Reference vecs for 5-segmenters and 6-segmenters
    let (mut seg5, mut seg6) = (Vec::new(), Vec::new());
    for digit in &line.patterns {
        if digit.segments.len() == 5 {
            seg5.push(digit);
        }
        if digit.segments.len() == 6 {
            seg6.push(digit);
        }
    }

    // Find 4
    let four: &Digit = &line.patterns.iter().find(|d| d.number == Some(4)).unwrap();
    // Find 1
    let one: &Digit = &line.patterns.iter().find(|d| d.number == Some(1)).unwrap();

    // 6-segmenters first:
    // Digit where everything from 4 is in -> 9
    let nine = find_congruent(four, &mut seg6).unwrap();
    set_number(nine, &mut result, 9);
    
    // Digit where everything from 1 is in -> 0
    let zero = find_congruent(one, &mut seg6).unwrap();
    set_number(zero, &mut result, 0);

    // Rest -> 6
    let six = seg6[0];
    set_number(six, &mut result, 6);

    // 5-segmenters
    // Digit where everything from 1 is in -> 3
    let three = find_congruent(one, &mut seg5).unwrap();
    set_number(three, &mut result, 3);

    // Get only segment that is not on for 6
    let mut all_segments: HashSet<char> = ('a'..='g').collect();
    for segment in &six.segments {
        all_segments.remove(segment);
    }
    let decider_for_2_vs_5: char = *all_segments.iter().next().unwrap();

    // Digit where that is present -> 2
    let two = find_congruent(
        &Digit::new_char(decider_for_2_vs_5), 
        &mut seg5
    ).unwrap();
    set_number(two, &mut result, 2);

    // Rest -> 5
    let five = seg5[0];
    set_number(five, &mut result, 5);

    // Go with every not-deduced output digit through solved patterns
    // When match (sorted so normal comparison for equality should suffice) -> Copy number and continue with next output
    for out in &mut line.output {
        for digit in &result {
            if out == digit {
                out.number = digit.number;
                break;
            }
        }
    }

    // Return total
    calculate_line_total(&line.output)
}

/// Helper method to save the decoded number in a vector of digits
fn set_number(set_digit: &Digit, pool: &mut Vec<Digit>, number: i32) {
    for digit in pool {
        if digit == set_digit {
            digit.number = Some(number);
            return;
        }
    }
}

/// Congruency algorithm
fn find_congruent<'a>(template: &Digit, pool: &mut Vec<&'a Digit>) -> Option<&'a Digit> {
    for i in 0..pool.len() {
        if template.fits_on(pool[i]) {
            let result = pool[i];
            pool.remove(i);
            return Some(result);
        }
    }

    None
}

/// Calculate total for one line
/// 
/// for loop from 3 to 0 (inclusive)
/// number * 10.pow(loop var) + total
fn calculate_line_total(output: &Vec<Digit>) -> i32 {
    let (mut total, mut counter) = (0, 0);
    for i in (0..4).rev() {
        total += 10_i32.pow(i as u32) * output.get(counter)
                                                .expect(&format!("The given output for calculation was of the wrong size (expected: 4; given: {}", output.len()))
                                                .number.expect("The line was not solved first");
        counter += 1;
    }
    total
}

/// Determine, whether line is solved
/// 
/// Check for all output digits, if all have a Some(i32) value for number, they are all solved
fn solved(digits: &Vec<Digit>) -> bool {
    digits.iter().all(|digit| digit.number != None)
}

/// Public API for the first part
/// 
/// Since it's not needed anymore, but is part of the puzzle, its non-usage is ignored by the compiler
pub fn _part_one() {
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