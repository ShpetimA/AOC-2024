use std::fs::read_to_string;

pub fn get_reports() -> Vec<Vec<i32>> {
    let file = read_to_string("input/day2.txt").expect("File doesn't exist");

    file.lines()
        .map(|line| {
            line.split_whitespace()
                .zip(line.split_whitespace().skip(1))
                .map(|(curr, next)| curr.parse::<i32>().unwrap() - next.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn safe_reports(safe_buff: i32, reports: &Vec<Vec<i32>>) -> i32 {
    let max_increase = 3;
    let mut safe_reports = 0;

    for report in reports {
        let mut safe_buff = safe_buff;
        let increasing = report[0] < 0;

        for &curr in report.iter() {
            if curr.abs() > max_increase || curr == 0 {
                safe_buff -= 1;
            }

            if increasing && curr > 0 {
                safe_buff -= 1;
            } else if !increasing && curr < 0 {
                safe_buff -= 1;
            }
        }

        if safe_buff >= 0 {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn part_one() {
    let reports: Vec<Vec<i32>> = get_reports();

    let safe_reports = safe_reports(0, &reports);
    println!("SAFE {safe_reports}")
}

pub fn part_two() {
    let reports: Vec<Vec<i32>> = get_reports();
    let safe_reports = safe_reports(1, &reports);
    println!("SAFE {safe_reports}");
}
