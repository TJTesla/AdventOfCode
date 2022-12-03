use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Rucksack {
    bag_one: HashSet<char>,
    bag_two: HashSet<char>
}

// a b c d e f g h [Length: 8]; 8/2 = 4
impl Rucksack {
    fn new(line: &str) -> Rucksack {
        let (first, last) = line.split_at(line.len()/2);

        let mut bag_one = HashSet::new();
        for c in first.chars() {
            bag_one.insert(c);
        }

        let mut bag_two = HashSet::new();
        for c in last.chars() {
            bag_two.insert(c);
        }

        Rucksack { bag_one, bag_two }
    }
}

fn parse_input(test: bool) -> Vec<Rucksack> {
    let path = if test {
        "input/year2022/test3.txt".to_string()
    } else {
        "input/year2022/day3.txt".to_string()
    };

    let mut res = Vec::new();

    let input = read_to_string(path).unwrap();
    for line in input.lines() {
        res.push(Rucksack::new(line));
    }

    return res;
}

fn line_to_hashset(line: &str) -> HashSet<char> {
    let mut res = HashSet::new();
    for c in line.chars() {
        res.insert(c);
    }
    return res;
}

fn parse_input_part_two(test: bool) -> Vec<(HashSet<char>, HashSet<char>, HashSet<char>)> {
    let path = if test {
        "input/year2022/test3.txt".to_string()
    } else {
        "input/year2022/day3.txt".to_string()
    };

    let mut res = Vec::new();
    let mut buffer = Vec::new();

    let input = read_to_string(path).unwrap();
    for line in input.lines() {
        buffer.push(line_to_hashset(line));
        if buffer.len() == 3 {
            res.push(
                (buffer.pop().unwrap(), buffer.pop().unwrap(), buffer.pop().unwrap())
            );
        }
    }

    return res;
}

fn find_duplicate(rucksack: Rucksack) -> char {
    for c in rucksack.bag_one.iter() {
        if rucksack.bag_two.get(c) != None {
            return *c;
        }
    }
    panic!("There wasn't a duplicate char found in Rucksack: {:?}", rucksack);
}

fn find_badge(group: (HashSet<char>, HashSet<char>, HashSet<char>)) -> char {
    for c in group.0 {
        if group.1.get(&c) != None {
            if group.2.get(&c) != None {
                return c;
            }
        }
    }
    panic!("There was no badge found");
}

fn char_to_score(c: char) -> u32 {
    let difference = if c.is_ascii_lowercase() {
        // Difference to get from ascii value of lower case to range 1..=26
        96
    } else {
        // Difference to get from ascii value of upper case to range 27..=52
        38
    };

    c as u32 - difference
}

pub fn part_two() {
    let input = parse_input_part_two(false);

    let mut score = 0;
    for group in input {
        score += char_to_score(find_badge(group));
    }

    println!("The score is {}", score);
}

pub fn part_one() {
    let input = parse_input(false);

    let mut score = 0;
    for r in input {
        score += char_to_score(find_duplicate(r));
    }

    println!("The score is {}", score);
}