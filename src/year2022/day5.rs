use std::fs::read_to_string;

fn parse_input(test: bool) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let path = if test {
        "input/year2022/test5.txt".to_string()
    } else {
        "input/year2022/day5.txt".to_string()
    };
    let input = read_to_string(path).unwrap();
    
    let mut stack_input: Vec<Vec<char>> = Vec::new();
    let mut divider = input.split("\n\n");

    for line in divider.next().unwrap().lines() {
        let line_string = line.replace("[", "");
        let line_string = line_string.replace("] ", "");
        let line_string = line_string.replace(']', "");
        let mut line_string = line_string.replace("   ", "");

        while (!test && line_string.len() < 9)
            || (test && line_string.len() < 3)
        {
            line_string.push(' ');
        }
        stack_input.push(line_string.chars().collect());
    }

    stack_input.pop();
    let mut result: Vec<Vec<char>> = Vec::new();
    for _ in &stack_input[0] {
        result.push(Vec::new());
    }
    for line in stack_input {
        for (index, c) in line.into_iter().enumerate() {
            if c == ' ' {
                continue;
            }
            result.get_mut(index).expect(&format!("ERROR: Index: {}", index)).insert(0, c);
        }
    }

    let mut moves = Vec::new();
    for line in divider.next().unwrap().lines() {
        let mut line_string = line.replace("move ", "");
        line_string = line_string.replace(" from ", " ");
        line_string = line_string.replace(" to ", " ");
        let l: Vec<usize> = line_string.split_ascii_whitespace().map(|c| c.parse().unwrap()).collect();

        for _ in 0..l[0] {
            moves.push((l[1], l[2]));
        }
    }

    (result, moves)
}

fn parse_input_part_two(test: bool) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let path = if test {
        "input/year2022/test5.txt".to_string()
    } else {
        "input/year2022/day5.txt".to_string()
    };
    let input = read_to_string(path).unwrap();
    
    let mut stack_input: Vec<Vec<char>> = Vec::new();
    let mut divider = input.split("\n\n");

    for line in divider.next().unwrap().lines() {
        let line_string = line.replace("[", "");
        let line_string = line_string.replace("] ", "");
        let line_string = line_string.replace(']', "");
        let mut line_string = line_string.replace("   ", "");

        while (!test && line_string.len() < 9)
            || (test && line_string.len() < 3)
        {
            line_string.push(' ');
        }
        stack_input.push(line_string.chars().collect());
    }

    stack_input.pop();
    let mut result: Vec<Vec<char>> = Vec::new();
    for _ in &stack_input[0] {
        result.push(Vec::new());
    }
    for line in stack_input {
        for (index, c) in line.into_iter().enumerate() {
            if c == ' ' {
                continue;
            }
            result.get_mut(index).expect(&format!("ERROR: Index: {}", index)).insert(0, c);
        }
    }

    let mut moves = Vec::new();
    for line in divider.next().unwrap().lines() {
        let mut line_string = line.replace("move ", "");
        line_string = line_string.replace(" from ", " ");
        line_string = line_string.replace(" to ", " ");
        let l: Vec<usize> = line_string.split_ascii_whitespace().map(|c| c.parse().unwrap()).collect();

        moves.push((l[0], l[1], l[2]));
    }

    (result, moves)
}

fn move_crate(stacks: &mut Vec<Vec<char>>, movement: (usize, usize)) {
    if let Some(c) = stacks[movement.0-1].pop() {
        stacks[movement.1-1].push(c);
    }
}

fn move_crate_9001(stacks: &mut Vec<Vec<char>>, movement: (usize, usize, usize)) {
    let range = (stacks[movement.1-1].len()-movement.0)..stacks[movement.1-1].len();
    let mut crates: Vec<char> = stacks[movement.1-1].drain(range).collect();

    stacks[movement.2-1].append(&mut crates);
}

fn get_result(stacks: Vec<Vec<char>>) -> String {
    let mut result = String::new();

    for s in stacks {
        result.push(*s.last().unwrap());
    }

    result
}

pub fn part_two() {
    let mut input = parse_input_part_two(false);
    for m in input.1 {
        move_crate_9001(&mut input.0, m);
    }

    println!("The result is {}", get_result(input.0));
}

pub fn part_one() {
    let mut input = parse_input(false);
    for m in input.1 {
        move_crate(&mut input.0, m);
    }

    println!("The result is {}", get_result(input.0));
}