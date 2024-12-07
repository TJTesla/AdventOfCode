#![allow(dead_code)]

use std::fs;

fn parse_input(test: bool) -> Vec<(i64, Vec<i64>)> {
    let path = if test {
        "input/year2024/test7.txt"
    } else {
        "input/year2024/day7.txt"
    };
    let input = fs::read_to_string(path).unwrap();

    let mut res = Vec::new();
    for line in input.lines() {
        let mut first = 0;
        let mut operands = Vec::new();
        for num in line.split_ascii_whitespace() {
            if num.contains(':') {
                first = num.replace(":", "").parse().expect(&format!("Could not parse {}   ", num));
                continue;
            }
            operands.push(num.parse().unwrap());
        }

        res.push((first, operands));
    }


    res
}

fn may_be_correct(result: i64, operands: Vec<i64>) -> bool {
    let mut stack: Vec<(usize, i64)> = vec![(1, operands[0])];

    while !stack.is_empty() {
        let i = stack[stack.len()-1].0;
        let intermediate = stack[stack.len()-1].1;
        stack.remove(stack.len()-1);

        if i == operands.len() {
            if intermediate == result {
                return true;
            }
            continue;
        }

        stack.push((i+1, intermediate * operands[i]));
        stack.push((i+1, intermediate + operands[i]));
    }

    false
}

// > 1598073054248
// > 1598376236117
// > 5030527854106
//   5030892084481
pub fn part_one(_test: bool) {
    let input = parse_input(false);

    let mut res = 0;
    for equation in input {
        if may_be_correct(equation.0, equation.1) {
            res += equation.0;
        }
    }

    println!("{}", res);
}

fn concat_op(a: i64, b: i64) -> i64 {
    let mut a_str = a.to_string();
    let b_str = b.to_string();

    a_str.push_str(&b_str);
    a_str.parse().unwrap()
}


fn may_be_correct_with_three(result: i64, operands: Vec<i64>) -> bool {
    let mut stack: Vec<(usize, i64)> = vec![(1, operands[0])];

    while !stack.is_empty() {
        let i = stack[stack.len()-1].0;
        let intermediate = stack[stack.len()-1].1;
        stack.remove(stack.len()-1);

        if i == operands.len() {
            if intermediate == result {
                return true;
            }
            continue;
        }

        stack.push((i+1, intermediate * operands[i]));
        stack.push((i+1, intermediate + operands[i]));
        stack.push((i+1, concat_op(intermediate, operands[i])));
    }

    false
}


pub fn part_two(_test: bool) {
    let input = parse_input(false);

    let mut res = 0;
    for equation in input {
        if may_be_correct_with_three(equation.0, equation.1) {
            res += equation.0;
        }
    }

    println!("{}", res);
}