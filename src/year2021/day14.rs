use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fs;

#[derive(Eq, Debug, Copy, Clone)]
struct Rule {
    left: char,
    right: char,
    insert: char,
}

impl Rule {
    fn new_from_line(line: &str) -> Rule {
        let mut iter = line.split(" -> ");

        let origin = iter.next().unwrap();
        let mut origin_iter = origin.chars();
        let left = origin_iter.next().unwrap();
        let right = origin_iter.next().unwrap();

        let insert = iter.next().unwrap().chars().next().unwrap();

        Rule::new(left, right, insert)
    }

    fn new(left: char, right: char, insert: char) -> Rule {
        Rule {
            left,
            right,
            insert,
        }
    }

    fn new_search(left: char, right: char) -> Rule {
        Rule {
            left,
            right,
            insert: '\0'
        }
    }

    fn next_rules(&self) -> (Rule, Rule) {
        (Rule::new_search(self.left, self.insert), Rule::new_search(self.insert, self.right))
    }
}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.left.hash(state);
        self.right.hash(state);
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

fn initial_fill(line: String, rules: &mut HashMap<Rule, u128>) {
    let mut left_iter = line.chars();
    let mut right_iter = line.chars();
    right_iter.next();

    loop {
        let left = left_iter.next().unwrap();
        let right = match right_iter.next() {
            Some(val) => val,
            None => break
        };

        let num = rules.entry(Rule::new_search(left, right)).or_insert(0);
        *num += 1;
    }
}

fn calculate(steps: i8, line: String, mut rules: HashMap<Rule, u128>) -> u128 {
    println!("Before: {}", size_of_map(&rules));
    initial_fill(line, &mut rules);

    for _ in 0..steps {
        println!("Size: {}", size_of_map(&rules));
        let mut temp_map = rules.clone();
        for (_ , n) in &mut temp_map {
            *n = 0;
        }

        for (rule, current_amount) in &rules {
            let next_rules = rule.next_rules();

            let real_pair = *(rules.get_key_value(&(next_rules.0)).unwrap().0);
            let e = temp_map.entry(real_pair).or_insert(*current_amount);
            *e += current_amount;

            let real_pair = *(rules.get_key_value(&(next_rules.1)).unwrap().0);
            let e = temp_map.entry(real_pair).or_insert(*current_amount);
            *e += current_amount;
        }

        rules = temp_map;
    }

    return calculate_score(&rules);
}

fn size_of_map<T>(map: &HashMap<T, u128>) -> u128 {
    map.values().sum()
}

fn calculate_score(rules: &HashMap<Rule, u128>) -> u128 {
    if rules.len() == 0 {
        return 0;
    }
    let mut scores: HashMap<char, u128> = HashMap::new();

    let mut pair_iter = rules.iter();
    let first_pair = pair_iter.next().unwrap();

    let e = scores.entry(first_pair.0.left).or_insert(0);
    *e += rules.get(first_pair.0).unwrap();

    for pair in pair_iter {
        let e = scores.entry(pair.0.right).or_insert(0);
        *e += rules.get(pair.0).unwrap();
    }

    let mut max: (char, u128);
    let mut min: (char, u128);

    let mut score_iter = scores.into_iter();
    max = score_iter.next().unwrap();
    min = max;
    for (c, n) in score_iter {
        if n > max.1 {
            max = (c, n);
        }

        if n < min.1 {
            min = (c, n);
        }
    }

    return max.1 - min.1;
}


fn parse_input(test: bool) -> (String, HashMap<Rule, u128>) {
    let mut result = HashMap::new();

    let file_content = if test {
        fs::read_to_string("input/year2021/test14.txt").unwrap()
    } else {
        fs::read_to_string("input/year2021/day14.txt").unwrap()
    };

    let mut line_iter = file_content.lines();
    let poly: String = line_iter.next().unwrap().to_string();

    for line in line_iter {
        if line.len() == 0 {
            continue;
        }
        result.insert(Rule::new_from_line(line), 0);
    }

    return (poly, result);
}

pub fn part_one() {
    let (line, rules) = parse_input(false);

    let result = calculate(10, line, rules);

    println!("The result is {}", result);
}

pub fn part_two() {
    let (line, rules) = parse_input(false);

    let result = calculate(40, line, rules);

    println!("The result is {}", result);
}