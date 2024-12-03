use regex::Regex;
use std::fs::read_to_string;

pub fn get_lines_1() -> Vec<(i32, i32)> {
    let file = read_to_string("input/day3.txt").expect("File doesn't exist");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut vec: Vec<(i32, i32)> = Vec::new();

    file.lines().for_each(|line| {
        for (_, [a, b]) in re.captures_iter(line).map(|x| x.extract()) {
            vec.push((a.parse().unwrap(), b.parse().unwrap()));
        }
    });

    return vec;
}

pub fn part_one() {
    let lines = get_lines_1();
    let mut sum = 0;

    for (a, b) in lines {
        sum += a * b;
    }

    println!("{sum:?}");
}

#[derive(Debug, PartialEq, Eq)]
enum MulState {
    Enabled,
    Disabled,
}

pub fn get_lines_2() -> Vec<(i32, i32)> {
    let file = read_to_string("input/day3.txt").expect("File doesn't exist");
    let re = Regex::new(r"(do)\(\)|(don)\'t\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut input: Vec<(i32, i32)> = Vec::new();

    let mut state = MulState::Enabled;
    file.lines().for_each(|line| {
        for capture in re.captures_iter(line) {
            if capture.get(1).is_some() {
                state = MulState::Enabled;
            } else if capture.get(2).is_some() {
                state = MulState::Disabled;
            } else if state == MulState::Enabled {
                let a = capture.get(3).unwrap().as_str();
                let b = capture.get(4).unwrap().as_str();
                input.push((a.parse().unwrap(), b.parse().unwrap()));
            }
        }
    });

    return input;
}

pub fn part_two() {
    let lines = get_lines_2();
    let mut sum = 0;

    for (a, b) in lines {
        sum += a * b;
    }

    println!("PART 2:{sum:?}");
}
