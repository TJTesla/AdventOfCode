mod year2021;
mod year2022;

use std::env;

fn string_to_bool(s: &str) -> bool {
    match s {
        "true" => true,
        "false" => false,
        _ => panic!("The given argument: '{}' wasn't 'true' or 'false'", s)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _test: bool = if args.len() > 1 {
        string_to_bool(&args[1])
    } else {
        true
    };

    year2022::day8::part_one(_test);
}
