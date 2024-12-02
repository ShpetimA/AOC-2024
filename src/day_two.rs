use std::fs::read_to_string;

pub fn part_one() {
    let max_increase = 3;
    let file = read_to_string("input/day2.txt").expect("File doesn't exist");

    let reports: Vec<Vec<i32>> = file
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let mut safe_reports = 0;

    for report in reports {
        let mut increasing = true;
        let mut safe = true;

        if report[0] - report[1] > 0 {
            increasing = false;
        }

        for (idx, level) in report.iter().enumerate() {
            if idx == report.len() - 1 {
                break;
            }
            let diff = level - report[idx + 1];

            if diff.abs() > max_increase || diff.abs() == 0 {
                safe = false;
                break;
            }

            if increasing && diff > 0 {
                safe = false;
            } else if !increasing && diff < 0 {
                safe = false;
            }
        }
        if safe {
            safe_reports += 1;
        }
    }

    println!("SAFE {safe_reports}")
}

pub fn part_two() {
    let max_increase = 3;
    let file = read_to_string("input/day2.txt").expect("File doesn't exist");

    let reports: Vec<Vec<i32>> = file
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let mut safe_reports = 0;

    for report in reports {
        let mut increasing = true;
        let mut safe = true;
        let mut ignore_once = true;

        if report[0] - report[1] > 0 {
            increasing = false;
        }

        for (idx, level) in report.iter().enumerate() {
            if idx == report.len() - 1 {
                break;
            }
            let diff = level - report[idx + 1];

            if diff.abs() > max_increase || diff.abs() == 0 {
                if ignore_once {
                    ignore_once = false;
                    continue;
                }
                safe = false;
                break;
            }

            if increasing && diff > 0 {
                if ignore_once {
                    ignore_once = false;
                    continue;
                }
                safe = false;
            } else if !increasing && diff < 0 {
                if ignore_once {
                    ignore_once = false;
                    continue;
                }
                safe = false;
            }
        }
        if safe {
            safe_reports += 1;
        }
    }

    println!("SAFE {safe_reports}")
}
